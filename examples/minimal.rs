//! Minimalistic Sciter sample.

extern crate sciter;

use sciter::Element;
use self::sciter::dom::event::*;
use self::sciter::dom::HELEMENT;

fn main() {
	let html = include_bytes!("minimal.htm");
	let mut frame = sciter::Window::new();
	frame.load_html(html, None);
	frame.event_handler(EventReceiver);
	//    if let Some(mut el) = Element::from_window(frame.get_hwnd()).unwrap().find_first("div#send-event-message").unwrap() {
	//        el.attach_handler(EventReceiver);
	//    }
	if let Some(mut el) = Element::from_window(frame.get_hwnd()).unwrap().find_first("button#fire-event").unwrap() {
		el.attach_handler(SendEvent);
	}
	frame.run_app(true);
}

struct SendEvent;

impl sciter::EventHandler for SendEvent {
	#[warn(unused_variables)]
	fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
		if phase != PHASE_MASK::BUBBLING {
			return false;
		}

		match code {
			sciter::dom::event::BEHAVIOR_EVENTS::BUTTON_CLICK => {
				if let Some(el) = Element::from(root).parent().unwrap().find_first("div#fire-event-message").unwrap() {
					el.fire_event(root, el.as_ptr(), BEHAVIOR_EVENTS::CHANGE, EVENT_REASON::SYNTHESIZED, false, None);
					return false
				} else {
					return false
				}
			},
			_ => return false
		}

		true
	}
}

struct EventReceiver;

impl sciter::EventHandler for EventReceiver {
	#[warn(unused_variables)]
	fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
		if phase != PHASE_MASK::BUBBLING {
			return false;
		}
		match code {
			sciter::dom::event::BEHAVIOR_EVENTS::CHANGE => {
				println!("Phase: {:?}, Target: {}, source: {}, code: {:?}", phase, Element::from(target), Element::from(source), code);
				Element::from(source).set_text("An event was fired!!!");
			},
			_ => return false
		}

		false
	}
}