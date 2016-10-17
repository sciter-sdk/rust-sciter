//! Fire event Sciter sample.
#![allow(unused_variables)]
#![allow(non_snake_case)]

#[macro_use]
extern crate sciter;

use sciter::Element;
use self::sciter::dom::event::*;
use self::sciter::dom::HELEMENT;
use self::sciter::value::Value;

struct FireEvent;

impl sciter::EventHandler for FireEvent {
    #[warn(unused_variables)]
    fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
        if phase != PHASE_MASK::BUBBLING {
            return false;
        }

        match code {
            sciter::dom::event::BEHAVIOR_EVENTS::BUTTON_CLICK => {
                if let Some(el) = Element::from(root).parent().unwrap().find_first("div#fire-event-message").unwrap() {
                    el.fire_event(el.as_ptr(), el.as_ptr(), BEHAVIOR_EVENTS::CHANGE, EVENT_REASON::SYNTHESIZED, false, Some(Value::from("Data from fired event")));
                    return true
                } else {
                    return false
                }
            },
            _ => return false
        }

        true
    }
}

fn main() {
    let html = include_bytes!("fire_event.htm");
    let mut frame = sciter::Window::new();
    frame.load_html(html, None);
    if let Some(mut el) = Element::from_window(frame.get_hwnd()).unwrap().find_first("button#fire-event").unwrap() {
        el.attach_handler(FireEvent);
    }
    frame.run_app(true);
}