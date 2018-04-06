#![allow(unused_variables, unused_must_use)]

#[macro_use]
extern crate sciter;

use sciter::{Element, HELEMENT};
use sciter::host::{SCN_ATTACH_BEHAVIOR};
use sciter::dom::event::*;

#[derive(Default)]
struct Handler {
}


impl sciter::HostHandler for Handler {
	fn on_attach_behavior(&mut self, pnm: &mut SCN_ATTACH_BEHAVIOR) -> bool {
		let name = u2s!(pnm.name);
		if name == "video-generator" {
			let handler = VideoGen::new();
			self.attach_behavior(pnm, handler);
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

			},

			BEHAVIOR_EVENTS::VIDEO_INITIALIZED => {

			},

			BEHAVIOR_EVENTS::VIDEO_STARTED => {

			},

			BEHAVIOR_EVENTS::VIDEO_STOPPED => {

			},

			_ => return false,
		};

		return true;
	}
}

fn main() {
	use sciter::window;
	let mut frame = window::Window::with_size((750,750), window::Flags::main_window(true));
	let handler = Handler::default();
	frame.sciter_handler(handler);
	frame.set_title("Video renderer sample");
	frame.load_html(include_bytes!("video.htm"), None);
	frame.run_app();
}
