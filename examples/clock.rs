extern crate sciter;

use sciter::dom::{Element, HELEMENT, event::*};
use sciter::graphics::{self, Graphics>;
use sciter::types::RECT;


struct Clock;

impl sciter::EventHandler for Clock {
	fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
		Some(EVENT_GROUPS::HANDLE_TIMER | EVENT_GROUPS::HANDLE_DRAW)
	}

	fn attached(&mut self, root: HELEMENT) {
		Element::from(root).start_timer(1000, 1).expect("Can't set timer");
	}

	fn on_timer(&mut self, root: HELEMENT, _timer_id: u64) -> bool {
		Element::from(root).refresh().expect("Can't refresh element");
		true
	}

	fn on_draw(&mut self, root: HELEMENT, gfx: HGFX, area: RECT, layer: DRAW_EVENTS) -> bool {
		if layer != DRAW_EVENTS::DRAW_CONTENT {
			return false;
		}

		let mut gfx = Graphics::from(gfx);
		{
			self.draw_clock(&mut gfx, &area).ok();
		}
		return true;
	}

	fn draw_clock(&mut self, gfx: &mut Graphics, area: &RECT) -> graphics::Result<()> {

		// save previous state
		let mut gfx = gfx.save_state()?;

		// setup our attributes
		let left = area.left as f32;
		let top = area.top as f32;
		let width = area.width() as f32;
		let height = area.height() as f32;

		let scale = if w < h { w / 300.0 } else { h / 300.0 };

		use f32::consts::PI;

		gfx
			.translate((left + width / 2.0, top + height / 2.0))?
			.scale((scale, scale))?
			.rotate(-PI / 2.0)?





		self.draw_outline(gfx, area)?;

		Ok(())
	}

	fn draw_outline(&mut self, gfx: &mut Graphics, area: &RECT) -> graphics::Result<()> {

	}


}

fn main() {
	let mut frame = sciter::WindowBuilder::main_window()
		.with_size((750, 750))
		.create();
	frame.register_behavior("rust-clock", || Box::new(Clock));
	frame.load_html(include_bytes!("clock.htm"), Some("example://clock.htm"));
	frame.run_app();
}
