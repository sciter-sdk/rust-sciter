//! Windowless mode example (for Sciter.Lite build).
extern crate sciter;
extern crate winit;
extern crate winapi;
extern crate raw_window_handle;

use winit::event::{Event, WindowEvent, ElementState, MouseButton};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;


fn main() {
	if let Some(arg) = std::env::args().nth(1) {
		println!("loading sciter from {:?}", arg);
		if let Err(_) = sciter::set_options(sciter::RuntimeOptions::LibraryPath(&arg)) {
			panic!("Invalid sciter-lite dll specified.");
		}
	} else {
		panic!("usage: cargo run -p windowless -- sciter-sdk/bin.win/x64lite/sciter.dll")
	}

	// prepare and create a new window
	println!("create window");
	let events = EventLoop::new();

	use raw_window_handle::HasRawWindowHandle;
	let wnd = WindowBuilder::new();
	let wnd = wnd.build(&events).expect("Failed to create window");
	let window_handle = wnd.raw_window_handle();

	// configure Sciter
	println!("create sciter instance");
	sciter::set_options(sciter::RuntimeOptions::UxTheming(true)).unwrap();
	sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();
	sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(0xFF)).unwrap();

	// create an engine instance with an opaque pointer as an identifier
	use sciter::windowless::{Message, handle_message};
	let scwnd = { &wnd as *const _ as sciter::types::HWINDOW };
	handle_message(scwnd, Message::Create { backend: sciter::types::GFX_LAYER::SKIA_OPENGL, transparent: false, });

	#[cfg(windows)]
	{
		// Windows-specific: we need to redraw window in response to a corresponding notification.
		// winit 0.20 has an explicit `Window::request_redraw` method,
		// here we use `winapi::InvalidateRect` for this.
		struct WindowlessHandler {
			hwnd: winapi::shared::windef::HWND,
		}

		impl sciter::HostHandler for WindowlessHandler {
			fn on_invalidate(&mut self, pnm: &sciter::host::SCN_INVALIDATE_RECT) {
				unsafe {
					let rc = &pnm.invalid_rect;
					let dst = winapi::shared::windef::RECT {
						left: rc.left,
						top: rc.top,
						right: rc.right,
						bottom: rc.bottom,
					};
					winapi::um::winuser::InvalidateRect(self.hwnd, &dst as *const _, 0);
					// println!("- {} {}", rc.width(), rc.height());
				}
			}
		}

		let handler = WindowlessHandler {
			hwnd: match window_handle {
				raw_window_handle::RawWindowHandle::Windows(data) => data.hwnd as winapi::shared::windef::HWND,
				_ => unreachable!(),
			},
		};

		let instance = sciter::Host::attach_with(scwnd, handler);

		let html = include_bytes!("../../minimal.htm");
		instance.load_html(html, Some("example://minimal.htm"));
	}

	// events processing
	use sciter::windowless::{MouseEvent, KeyboardEvent, RenderEvent};
	use sciter::windowless::{MOUSE_BUTTONS, MOUSE_EVENTS, KEYBOARD_STATES, KEY_EVENTS};

	let mut mouse_button = MOUSE_BUTTONS::NONE;
	let mut mouse_pos = (0, 0);
	let mut current_modifiers = KEYBOARD_STATES::default();

	println!("running...");

	let startup = std::time::Instant::now();

	// release CPU a bit, hackish
	std::thread::sleep(std::time::Duration::from_millis(0));

	// Sciter processes timers and fading effects here
	handle_message(scwnd, Message::Heartbit {
		milliseconds: std::time::Instant::now().duration_since(startup).as_millis() as u32,
	});

	// the actual event loop polling

	events.run(move |event, _, control_flow| {
		match event {

			Event::RedrawRequested(_) => {

				let on_render = move |bitmap_area: &sciter::types::RECT, bitmap_data: &[u8]|
				{
					#[cfg(unix)]
					{
						let _ = bitmap_area;
						let _ = bitmap_data;
						let _ = window_handle;
					}

					// Windows-specific bitmap rendering on the window
					#[cfg(windows)]
					{
						use winapi::um::winuser::*;
						use winapi::um::wingdi::*;
						use winapi::shared::minwindef::LPVOID;

						let hwnd = match window_handle {
							raw_window_handle::RawWindowHandle::Windows(data) => data.hwnd as winapi::shared::windef::HWND,
							_ => unreachable!(),
						};

						unsafe {
							// NOTE: we use `GetDC` here instead of `BeginPaint`, because the way
							// winit 0.19 processed the `WM_PAINT` message (it always calls `DefWindowProcW`).

							// let mut ps = PAINTSTRUCT::default();
							// let hdc = BeginPaint(hwnd, &mut ps as *mut _);

							let hdc = GetDC(hwnd);

							let (w, h) = (bitmap_area.width(), bitmap_area.height());

							let mem_dc = CreateCompatibleDC(hdc);
							let mem_bm = CreateCompatibleBitmap(hdc, w, h);

							let mut bmi = BITMAPINFO::default();
							{
								let mut info = &mut bmi.bmiHeader;
								info.biSize = std::mem::size_of::<BITMAPINFO>() as u32;
								info.biWidth = w;
								info.biHeight = -h;
								info.biPlanes = 1;
								info.biBitCount = 32;
							}

							let old_bm = SelectObject(mem_dc, mem_bm as LPVOID);

							let _copied = StretchDIBits(mem_dc, 0, 0, w, h, 0, 0, w, h, bitmap_data.as_ptr() as *const _, &bmi as *const _, 0, SRCCOPY);
							let _ok = BitBlt(hdc, 0, 0, w, h, mem_dc, 0, 0, SRCCOPY);

							SelectObject(mem_dc, old_bm);

							// EndPaint(hwnd, &ps as *const _);
							ReleaseDC(hwnd, hdc);

							// println!("+ {} {}", w, h);
						}
					}

				};

				let cb = RenderEvent {
					layer: None,
					callback: Box::new(on_render),
				};

				handle_message(scwnd, Message::RenderTo(cb));
			},

			Event::WindowEvent { event, window_id: _ } => {
				match event {
					WindowEvent::Destroyed => {
						// never called due to loop break on close
						println!("destroy");
						handle_message(scwnd, Message::Destroy);
						*control_flow = ControlFlow::Exit
					},

					WindowEvent::CloseRequested => {
						println!("close");
						*control_flow = ControlFlow::Exit
					},

					WindowEvent::Resized(size) => {
						// println!("{:?}, size: {:?}", event, size);
						let (width, height): (u32, u32) = size.into();
						handle_message(scwnd, Message::Size { width, height });
					},

					WindowEvent::Focused(enter) => {
						println!("focus {}", enter);
						handle_message(scwnd, Message::Focus { enter });
					},

					WindowEvent::ModifiersChanged(modifiers) => {
						let mut keys = 0;
						if modifiers.ctrl() {
							keys |= KEYBOARD_STATES::CONTROL_KEY_PRESSED;
						}
						if modifiers.shift() {
							keys |= KEYBOARD_STATES::SHIFT_KEY_PRESSED;
						}
						if modifiers.alt() {
							keys |= KEYBOARD_STATES::ALT_KEY_PRESSED;
						}
						current_modifiers = keys.into();
					},

					WindowEvent::CursorEntered { device_id: _ } => {
						println!("mouse enter");
						let event = MouseEvent {
							event: MOUSE_EVENTS::MOUSE_ENTER,
							button: mouse_button,
							modifiers: KEYBOARD_STATES::from(0),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(scwnd, Message::Mouse(event));
					},

					WindowEvent::CursorLeft { device_id: _ } => {
						println!("mouse leave");
						let event = MouseEvent {
							event: MOUSE_EVENTS::MOUSE_LEAVE,
							button: mouse_button,
							modifiers: KEYBOARD_STATES::from(0),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(scwnd, Message::Mouse(event));
					},

					WindowEvent::CursorMoved { position, .. } => {
						mouse_pos = position.into();

						let event = MouseEvent {
							event: MOUSE_EVENTS::MOUSE_MOVE,
							button: mouse_button,
							modifiers: current_modifiers.clone(),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(scwnd, Message::Mouse(event));
					},

					WindowEvent::MouseInput { state, button, .. } => {
						mouse_button = match button {
							MouseButton::Left => MOUSE_BUTTONS::MAIN,
							MouseButton::Right => MOUSE_BUTTONS::PROP,
							MouseButton::Middle => MOUSE_BUTTONS::MIDDLE,
							_ => MOUSE_BUTTONS::NONE,
						};
						println!("mouse {:?} as {:?}", mouse_button, mouse_pos);

						let event = MouseEvent {
							event: if state == ElementState::Pressed { MOUSE_EVENTS::MOUSE_DOWN } else { MOUSE_EVENTS::MOUSE_UP },
							button: mouse_button,
							modifiers: current_modifiers.clone(),
							pos: sciter::types::POINT {
								x: mouse_pos.0,
								y: mouse_pos.1,
							},
						};

						handle_message(scwnd, Message::Mouse(event));
					},

					WindowEvent::KeyboardInput { input, .. } => {
						println!("key {} {}", input.scancode, if input.state == ElementState::Pressed { "down" } else { "up" });

						let event = KeyboardEvent {
							event: if input.state == ElementState::Pressed { KEY_EVENTS::KEY_DOWN } else { KEY_EVENTS::KEY_UP },
							code: input.scancode,
							modifiers: current_modifiers.clone(),
						};

						handle_message(scwnd, Message::Keyboard(event));
					},

					_	=> (),
				}
			},
			_ => (),
		}
	});
}
