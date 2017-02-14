//! Minimalistic Sciter sample.

extern crate sciter;

fn main() {
	let html = include_bytes!("minimal.htm");
	let mut frame = sciter::Window::new();
	frame.load_html(html, None);
	frame.run_app(true);
}
