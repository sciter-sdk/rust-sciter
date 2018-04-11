/*! High level window wrapper.

To create instance of Sciter you will need either to create new Sciter window or to attach (mix-in) Sciter engine to existing window.

Handle of the Sciter engine is defined as `HWINDOW` type which is:

* `HWND` handle on Microsoft Windows.
* `NSView*` – pointer to [`NSView`](https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ApplicationKit/Classes/NSView_Class/) instance that is a contentView of Sciter window on OS X.
* `GtkWidget*` – pointer to [`GtkWidget`](https://developer.gnome.org/gtk3/stable/GtkWidget.html) instance
that is a root widget of Sciter window on Linux/GTK.

## Creation of new window

```no_run
extern crate sciter;

fn main() {
	let mut frame = sciter::Window::new();
	frame.load_file("minimal.htm");
	frame.run_app();
}
```

Also you can register the [host](../host/trait.HostHandler.html) and [DOM](../dom/event/index.html) event handlers.

.
*/
use ::{_API};
use capi::sctypes::*;

use platform::{BaseWindow, OsWindow};
use host::{Host, HostHandler};
use dom::event::{EventHandler};

use std::rc::Rc;


/// `SCITER_CREATE_WINDOW_FLAGS` alias.
pub type Flags = SCITER_CREATE_WINDOW_FLAGS;

pub use capi::scdef::{SCITER_CREATE_WINDOW_FLAGS};


/// Per-window sciter engine options.
pub enum Options {
	/// value: `true` to enable, `false` to disable, enabled by default.
	SmoothScroll(bool),

	/// value: `0` - system default, `1` - no smoothing, `2` - standard smoothing, `3` - clear type.
	FontSmoothing(u8),

	/// Windows Aero support, value: `false` - normal drawing, `true` - window has transparent background after calls
	/// [`DwmExtendFrameIntoClientArea()`](https://msdn.microsoft.com/en-us/library/windows/desktop/aa969512(v=vs.85).aspx)
	/// or [`DwmEnableBlurBehindWindow()`](https://msdn.microsoft.com/en-us/library/windows/desktop/aa969508(v=vs.85).aspx).
	TransparentWindow(bool),

	///value - TRUE/FALSE - window uses per pixel alpha (e.g. WS_EX_LAYERED/UpdateLayeredWindow() window).
	AlphaWindow(bool),
}


/// Sciter window.
pub struct Window
{
	base: OsWindow,
	host: Rc<Host>,
}

impl Window {

	/// Create a new main window.
	pub fn new() -> Window {
		let flags = SCITER_CREATE_WINDOW_FLAGS::main_window(true);
		Window::create((0,0,0,0), flags, None)
	}

	/// Create new window with specified `size(width, height)` and flags.
	pub fn with_size(size: (i32, i32), flags: SCITER_CREATE_WINDOW_FLAGS) -> Window {
		let (w, h) = size;
		Window::create((0,0,w,h), flags, None)
	}

	/// Create new window with specified position as `rect(x, y, width, height)` and flags.
	pub fn with_rect(rect: (i32, i32, i32, i32), flags: SCITER_CREATE_WINDOW_FLAGS) -> Window {
		Window::create(rect, flags, None)
	}

	/// Create new window with specified position as `rect(x, y, width, height)`, flags and optional parent window.
	pub fn create(rect: (i32, i32, i32, i32), flags: SCITER_CREATE_WINDOW_FLAGS, parent: Option<HWINDOW>) -> Window {
		let mut base = OsWindow::new();
		let hwnd = base.create(rect, flags as UINT, parent.unwrap_or(0 as HWINDOW));
		assert!(!hwnd.is_null());

		let wnd = Window { base: base, host: Rc::new(Host::attach(hwnd))};
		return wnd;
	}

	/// Attach Sciter to an existing native window.
	pub fn attach(hwnd: HWINDOW) -> Window {
		assert!(!hwnd.is_null());
		Window { base: OsWindow::from(hwnd), host: Rc::new(Host::attach(hwnd)) }
	}

	/// Obtain reference to `Host` which allows you to control sciter engine and windows.
	pub fn get_host(&self) -> Rc<Host> {
		Rc::clone(&self.host)
	}

	/// Set callback for sciter engine events.
	pub fn sciter_handler<Callback: HostHandler + Sized>(&mut self, handler: Callback) {
		self.host.setup_callback(handler);
	}

	/// Attach `dom::EventHandler` to the Sciter window.
	///
	/// You can install Window EventHandler only once - it will survive all document reloads.
	pub fn event_handler<Handler: EventHandler>(&mut self, handler: Handler) {
		self.host.attach_handler(handler);
	}

	/// Register a native event handler for the specified behavior name.
	pub fn register_behavior<Factory>(&mut self, name: &str, factory: Factory)
	where
		Factory: Fn() -> Box<EventHandler> + 'static
	{
		self.host.register_behavior(name, factory);
	}

	/// Load HTML document from file.
	pub fn load_file(&mut self, uri: &str) {
		self.host.load_file(uri)
	}

	/// Load HTML document from memory.
	pub fn load_html(&mut self, html: &[u8], uri: Option<&str>) {
		self.host.load_html(html, uri)
	}

	/// Get native window handle.
	pub fn get_hwnd(&self) -> HWINDOW {
		self.base.get_hwnd()
	}

	/// Minimize or hide window.
	pub fn collapse(&self, hide: bool) {
		self.base.collapse(hide)
	}

	/// Show or maximize window.
	pub fn expand(&self, maximize: bool) {
		self.base.expand(maximize)
	}

	/// Close window.
	pub fn dismiss(&self) {
		self.base.dismiss()
	}

	/// Set title of native window.
	pub fn set_title(&mut self, title: &str) {
		self.base.set_title(title)
	}

	/// Get native window title.
	pub fn get_title(&self) -> String {
		self.base.get_title()
	}

	/// Set various sciter engine options, see the [`Options`](enum.Options.html).
	pub fn set_options(&self, options: Options) -> Result<(), ()> {
		use self::Options::*;
		use capi::scdef::SCITER_RT_OPTIONS;
		let (option, value) = match options {
			SmoothScroll(enable) => (SCITER_RT_OPTIONS::SCITER_SMOOTH_SCROLL, enable as usize),
			FontSmoothing(technology) => (SCITER_RT_OPTIONS::SCITER_FONT_SMOOTHING, technology as usize),
			TransparentWindow(enable) => (SCITER_RT_OPTIONS::SCITER_TRANSPARENT_WINDOW, enable as usize),
			AlphaWindow(enable) => (SCITER_RT_OPTIONS::SCITER_ALPHA_WINDOW, enable as usize),
		};
		let ok = (_API.SciterSetOption)(self.get_hwnd(), option, value);
		if ok != 0 {
			Ok(())
		} else {
			Err(())
		}
	}

	/// Show window and run the main app message loop until window been closed.
	pub fn run_app(self) {
		self.base.expand(false);
		self.base.run_app();
	}

	/// Run the main app message loop with already configured window.
	pub fn run_loop(&self) {
		self.base.run_app();
	}

	/// Post app quit message.
	pub fn quit_app(&self) {
		self.base.quit_app()
	}
}
