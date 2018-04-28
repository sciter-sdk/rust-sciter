#![windows_subsystem="windows"]
extern crate sciter;

use sciter::dom::{HELEMENT, Element};
use sciter::dom::event::{EVENT_GROUPS, DRAW_EVENTS};
use sciter::graphics::{self, HGFX, Graphics, rgb};
use sciter::types::RECT;

// 360°
const PI2: f32 = 2.0 * std::f32::consts::PI;


struct Clock;

impl sciter::EventHandler for Clock {
	fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
		// we need timer and draw events
		Some(EVENT_GROUPS::HANDLE_TIMER | EVENT_GROUPS::HANDLE_DRAW)
	}

	fn attached(&mut self, root: HELEMENT) {
		// timer for every second
		Element::from(root)
			.start_timer(1000, 1).expect("Can't set timer");
	}

	fn on_timer(&mut self, root: HELEMENT, _timer_id: u64) -> bool {
		// to redraw our clock
		Element::from(root)
			.refresh().expect("Can't refresh element");
		unsafe { TICKS += 1; }
		true
	}

	fn on_draw(&mut self, _root: HELEMENT, gfx: HGFX, area: &RECT, layer: DRAW_EVENTS) -> bool {

		if layer == DRAW_EVENTS::DRAW_CONTENT {
			// draw content only
			// leave the back- and foreground to be default
			let mut gfx = Graphics::from(gfx);
			let ok = self.draw_clock(&mut gfx, &area);
			if let Err(err) = ok {
				println!("oops: {:?}", err);
			}
		}

		// allow default drawing anyway
		return false;
	}
}

impl Clock {

	fn draw_clock(&mut self, gfx: &mut Graphics, area: &RECT) -> graphics::Result<()> {

		// save previous state
		let mut gfx = gfx.save_state()?;

		// setup our attributes
		let left = area.left as f32;
		let top = area.top as f32;
		let width = area.width() as f32;
		let height = area.height() as f32;

		let scale = if width < height { width / 300.0 } else { height / 300.0 };

		// translate to its center and rotate 45° left.
		gfx
			.translate((left + width / 2.0, top + height / 2.0))?
			.scale((scale, scale))?
			.rotate(-PI2 / 4.)?
			;

		gfx
			.line_color(0)?
			.line_cap(graphics::LINE_CAP::ROUND)?;

		// draw clock background
		self.draw_outline(&mut *gfx)?;

		// draw clock sticks
		self.draw_time(&mut *gfx)?;

		Ok(())
	}

	fn draw_outline(&mut self, gfx: &mut Graphics) -> graphics::Result<()> {
		// hour marks (every 5 ticks)
		{
			let mut gfx = gfx.save_state()?;
			gfx
				.line_width(8.0)?
				.line_color(rgb(0x32, 0x5F, 0xA2))?;

			for _ in 0..12 {
				gfx
					.rotate(PI2/12.)?
					.line((137., 0.), (144., 0.))?;
			}
		}

		// minute marks (every but 5th tick)
		{
			let mut gfx = gfx.save_state()?;
			gfx
				.line_width(3.0)?
				.line_color(rgb(0xA5, 0x2A, 0x2A))?;

			for i in 0..60 {
				if i % 5 != 0 {	// skip hours
					gfx.line((143., 0.), (146., 0.))?;
				}
				gfx.rotate(PI2/60.)?;
			}
		}
		Ok(())
	}

	fn draw_time(&mut self, gfx: &mut Graphics) -> graphics::Result<()> {

		let time = current_time();
		let hours = time.0 as f32;
		let minutes = time.1 as f32;
		let seconds = time.2 as f32;

		{
			// hours
			let mut gfx = gfx.save_state()?;

			// 2PI*/12, 2PI/720,
			gfx.rotate(hours  * (PI2/12 as f32) + minutes * (PI2/(12*60) as f32) + seconds * (PI2/(12*60*60) as f32))?;

			gfx
				.line_width(14.0)?
				.line_color(rgb(0x32, 0x5F, 0xA2))?
				.line((-20., 0.), (70., 0.))?;
		}
		{
			// minutes
			let mut gfx = gfx.save_state()?;

			gfx.rotate(minutes * (PI2/60 as f32) + seconds * (PI2/(60*60) as f32))?;

			gfx
				.line_width(10.0)?
				.line_color(rgb(0x32, 0x5F, 0xA2))?
				.line((-28., 0.), (100., 0.))?;
		}
		{
			// seconds
			let mut gfx = gfx.save_state()?;

			gfx.rotate(seconds * (PI2/60 as f32))?;

			gfx
				.line_width(6.0)?
				.line_color(rgb(0xD4, 0, 0))?
				.fill_color(rgb(0xD4, 0, 0))?
				.line((-30., 0.), (83., 0.))?
				.circle((0., 0.), 10.)?;
		}
		Ok(())
	}
}

// Emulating a system clock here (don't want to pull a half of the Universe with `chrono` crate)
static mut TICKS: usize = 10*60*60 + 15 * 60 + 34;

fn current_time() -> (u8, u8, u8) {
	let mut t = unsafe { TICKS };

	let s = t % 60;
	t /= 60;
	let m = t % 60;
	t /= 60;
	let h = t % 12;
	(h as u8, m as u8, s as u8)
}


fn main() {
	let mut frame = sciter::WindowBuilder::main_window()
		.with_size((500, 500))
		.create();
	frame.register_behavior("rust-clock", || Box::new(Clock));
	frame.load_html(include_bytes!("clock.htm"), Some("example://clock.htm"));
	frame.run_app();
}
