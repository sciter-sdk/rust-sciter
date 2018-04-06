#![allow(unused_variables, unused_must_use)]

#[macro_use]
extern crate sciter;

use sciter::{HELEMENT};
use sciter::host::{SCN_ATTACH_BEHAVIOR};
use sciter::dom::event::*;

#[derive(Default)]
struct Handler {
}

impl sciter::HostHandler for Handler {
	fn on_attach_behavior(&mut self, pnm: &mut SCN_ATTACH_BEHAVIOR) -> bool {
		let name = u2s!(pnm.name);
		eprintln!("behavior: {:?}", name);
		if name == "video-generator" {
			self.attach_behavior(pnm, VideoGen::new());
			return true;
		}
		return false;
	}
}


struct VideoGen {

}

impl VideoGen {
	fn new() -> Self {
		Self {}
	}
}


macro_rules! ccall {
	($ty: ident :: $func:ident ($this: ident, $($arg:expr),* )) => {
		unsafe {
			let lp = $this.get() as *mut $ty;
			let vtbl = (*lp).vtbl;
			((*vtbl).$func)(lp, $($arg),* )
		}
	}
}

impl sciter::EventHandler for VideoGen {

	fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
		Some(EVENT_GROUPS::HANDLE_BEHAVIOR_EVENT)
	}

	fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
		if phase != PHASE_MASK::BUBBLING {
			return false;
		}

		match code {
			BEHAVIOR_EVENTS::VIDEO_BIND_RQ => {
				eprintln!("{:?}", code);

				if let EventReason::VideoBind(ptr) = reason {
					if ptr.is_null() {
						// first phase, consume the event to mark as we will provide frames
						return true;
					}

					use sciter::video::*;
					let site = AssetPtr::attach(ptr as *mut video_destination);
					let mut fragmented: *mut iasset = std::ptr::null_mut();
					let mut target = &mut fragmented as *mut _;
					let ok = ccall!(iasset::get_interface(site, INAME_VIDEO_FRAGMENTED_DESTINATION.as_ptr(), target));
					if ok && !fragmented.is_null() {
						let fragmented = fragmented as *mut fragmented_video_destination;
						let fragmanted = AssetPtr::new(fragmented);
					}



				}
			},

			BEHAVIOR_EVENTS::VIDEO_INITIALIZED => {
				eprintln!("{:?}", code);

			},

			BEHAVIOR_EVENTS::VIDEO_STARTED => {
				eprintln!("{:?}", code);

			},

			BEHAVIOR_EVENTS::VIDEO_STOPPED => {
				eprintln!("{:?}", code);

			},

			_ => return false,
		};

		return true;
	}
}


fn main() {
	if cfg!(all(target_os="windows", target_arch="x86")) {
		eprintln!("\nerror: Sciter video will not work on Windows x86.");
		eprintln!("error: Consider using a nightly Rust version to enable `abi_thiscall`,");
		eprintln!("error: see the https://github.com/rust-lang/rust/issues/42202.");
		::std::process::exit(126);
	}


	use sciter::window;
	let mut frame = window::Window::with_size((750,750), window::Flags::main_window(true));
	let handler = Handler::default();
	frame.sciter_handler(handler);
	frame.set_title("Video renderer sample");
	frame.load_html(include_bytes!("video.htm"), None);
	frame.run_app();
}
