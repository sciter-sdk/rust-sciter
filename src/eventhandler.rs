use capi::sctypes::*;
use capi::scbehavior::*;
use capi::scdom::{HELEMENT};
use value::Value;
use dom::event::{EventHandler, EventReason};

#[repr(C)]
pub struct WindowHandler<T>
{
	pub hwnd: HWINDOW,
	pub handler: T,
}

#[repr(C)]
pub struct BoxedHandler {
	pub handler: Box<EventHandler>,
}

fn is_detach_event(evtg: UINT, params: LPVOID) -> bool {
	let evtg : EVENT_GROUPS = unsafe { ::std::mem::transmute(evtg) };
	if evtg == EVENT_GROUPS::HANDLE_INITIALIZATION {
		assert!(!params.is_null());
		let scnm = params as *const INITIALIZATION_EVENTS;
		let cmd = unsafe { *scnm };
		if cmd == INITIALIZATION_EVENTS::BEHAVIOR_DETACH {
			return true;
		}
	}
	false
}

pub extern "system" fn _event_handler_window_proc<T: EventHandler>(tag: LPVOID, _he: ::capi::scdom::HELEMENT, evtg: UINT, params: LPVOID) -> BOOL
{
	use capi::scdom::HELEMENT;

	let boxed = tag as *mut WindowHandler<T>;
	let tuple: &mut WindowHandler<T> = unsafe { &mut *boxed };

	let root = ::dom::Element::from_window(tuple.hwnd);
	let hroot: HELEMENT = if root.is_ok() {
		root.unwrap().as_ptr()
	} else {
		::std::ptr::null_mut()
	};

	// custom initialization (because there is no DOM in plain window)
	let group : EVENT_GROUPS = unsafe { ::std::mem::transmute(evtg) };
	if group == EVENT_GROUPS::HANDLE_INITIALIZATION {
		assert!(!params.is_null());
		let scnm = params as *const INITIALIZATION_EVENTS;
		let cmd = unsafe { *scnm };
		match cmd {
			INITIALIZATION_EVENTS::BEHAVIOR_ATTACH => {
				tuple.handler.attached(hroot);
			},
			INITIALIZATION_EVENTS::BEHAVIOR_DETACH => {
				tuple.handler.detached(hroot);

				// here we dropping our tuple
				let ptr = unsafe { Box::from_raw(boxed) };
				drop(ptr);
			},
		};
		return true as BOOL;
	}

	process_events(&mut tuple.handler, hroot, evtg, params)
}

pub extern "system" fn _event_handler_behavior_proc(tag: LPVOID, he: HELEMENT, evtg: UINT, params: LPVOID) -> BOOL {
	// reconstruct pointer to Handler
	let boxed = tag as *mut BoxedHandler;
	let me = unsafe { &mut *boxed };
	let me = &mut *me.handler;

	if is_detach_event(evtg, params) {
		me.detached(he);

		// here we dropping our handler
		let ptr = unsafe { Box::from_raw(boxed) };
		drop(ptr);
		return true as BOOL;
	}

	process_events(me, he, evtg, params)
}

pub extern "system" fn _event_handler_proc<T: EventHandler>(tag: LPVOID, he: HELEMENT, evtg: UINT, params: LPVOID) -> BOOL
{
	// reconstruct pointer to Handler
	let boxed = tag as *mut T;
	let me = unsafe { &mut *boxed };

	if is_detach_event(evtg, params) {
		me.detached(he);

		// here we dropping our handler
		let ptr = unsafe { Box::from_raw(boxed) };
		drop(ptr);
		return true as BOOL;
	}

	process_events(me, he, evtg, params)
}

fn process_events(me: &mut EventHandler, he: HELEMENT, evtg: UINT, params: LPVOID) -> BOOL
{
	let evtg : EVENT_GROUPS = unsafe { ::std::mem::transmute(evtg) };
	// assert!(!he.is_null() || evtg == EVENT_GROUPS::SUBSCRIPTIONS_REQUEST);
	if he.is_null() && evtg != EVENT_GROUPS::SUBSCRIPTIONS_REQUEST {
		println!("warning! null element for {:?}", evtg);
	}

	let result = match evtg {

		EVENT_GROUPS::SUBSCRIPTIONS_REQUEST => {
			assert!(!params.is_null());
			let scnm = params as *mut EVENT_GROUPS;
			let nm = unsafe {&mut *scnm};
			let handled = me.get_subscription();
			if let Some(needed) = handled {
				*nm = needed;
			}
			handled.is_some()
		},

		EVENT_GROUPS::HANDLE_INITIALIZATION => {
			assert!(!params.is_null());
			let scnm = params as *const INITIALIZATION_EVENTS;
			let cmd = unsafe { *scnm };
			match cmd {
				INITIALIZATION_EVENTS::BEHAVIOR_DETACH => {
					me.detached(he);
				},

				INITIALIZATION_EVENTS::BEHAVIOR_ATTACH => {
					me.attached(he);
				},
			};
			true
		},

		EVENT_GROUPS::HANDLE_BEHAVIOR_EVENT => {
			assert!(!params.is_null());
			let scnm = params as *const BEHAVIOR_EVENT_PARAMS;
			let nm = unsafe { &*scnm };

			let code :BEHAVIOR_EVENTS = unsafe{ ::std::mem::transmute(nm.cmd & 0x0_0FFF) };
			let phase: PHASE_MASK = unsafe { ::std::mem::transmute(nm.cmd & 0xFFFF_F000) };
			let reason = match code {
				BEHAVIOR_EVENTS::EDIT_VALUE_CHANGED | BEHAVIOR_EVENTS::EDIT_VALUE_CHANGING => {
					let reason: EDIT_CHANGED_REASON = unsafe{ ::std::mem::transmute(nm.reason as UINT) };
					EventReason::EditChanged(reason)
				},

				BEHAVIOR_EVENTS::VIDEO_BIND_RQ => {
					EventReason::VideoBind(nm.reason as LPVOID)
				}

				_ => {
					let reason: CLICK_REASON = unsafe{ ::std::mem::transmute(nm.reason as UINT) };
					EventReason::General(reason)
				}
			};

			if he.is_null() {
				println!("warning! null element for {:?}:{:?}", evtg, code);
			}

			if phase == PHASE_MASK::SINKING {	// catch this only once
				match code {
					BEHAVIOR_EVENTS::DOCUMENT_COMPLETE => {
						me.document_complete(he, nm.heTarget);
					},
					BEHAVIOR_EVENTS::DOCUMENT_CLOSE => {
						me.document_close(he, nm.heTarget);
					},
					_ => ()
				};
			}

			let handled = me.on_event(he, nm.he, nm.heTarget, code, phase, reason);
			handled
		},

		EVENT_GROUPS::HANDLE_SCRIPTING_METHOD_CALL => {
			assert!(!params.is_null());
			let scnm = params as *mut SCRIPTING_METHOD_PARAMS;
			let nm = unsafe { &mut *scnm };
			let name = u2s!(nm.name);
			let argv = unsafe { Value::unpack_from(nm.argv, nm.argc) };
			let rv = me.on_script_call(he, &name, &argv);
			let handled = if let Some(v) = rv {
				v.pack_to(&mut nm.result);
				true
			} else {
				false
			};
			handled
		},

		EVENT_GROUPS::HANDLE_TIMER => {
			assert!(!params.is_null());
			let scnm = params as *const TIMER_PARAMS;
			let nm = unsafe { & *scnm };
			let handled = me.on_timer(he, nm.timerId as u64);
			handled
		},

		EVENT_GROUPS::HANDLE_DRAW => {
			assert!(!params.is_null());
			let scnm = params as *const DRAW_PARAMS;
			let nm = unsafe { & *scnm };
			let handled = me.on_draw(he, nm.gfx, &nm.area, nm.layer);
			handled
		},

		_ => false
	};
	return result as BOOL;
}
