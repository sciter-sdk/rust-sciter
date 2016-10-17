//! Minimalistic Sciter sample.

extern crate sciter;

use sciter::Element;
use self::sciter::dom::event::*;
use self::sciter::dom::HELEMENT;

fn main() {
	let html = include_bytes!("minimal.htm");
	let mut frame = sciter::Window::new();
	frame.load_html(html, None);
	frame.run_app(true);
}