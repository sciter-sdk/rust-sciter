#![allow(unused_variables, unused_must_use)]

#[macro_use]
extern crate sciter;

use sciter::dom::event::*;
use sciter::host::SCN_ATTACH_BEHAVIOR;
use sciter::{Element, HELEMENT};

#[derive(Default)]
struct Handler {}

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

macro_rules! cppcall {
	($ty: ident :: $func:ident ($this: ident $(, $arg:expr)* )) => {
		unsafe {
			let lp = $this.get() as *mut $ty;
			let vtbl = (*lp).vtbl;
			((*vtbl).$func)(lp, $($arg),* )
		}
	}
}

use sciter::video::{fragmented_video_destination, AssetPtr};

struct VideoGen {
  thread: Option<std::thread::JoinHandle<()>>,
}

impl VideoGen {
  fn new() -> Self {
    Self { thread: None }
  }

  fn generation_thread(site: AssetPtr<fragmented_video_destination>) {
    println!("[video] thread started.");

    const VIDEO_SIZE: (i32, i32) = (1200, 800);
    const FRAGMENT_SIZE: (i32, i32) = (256, 32);

    use sciter::video::*;

    let is_alive = cppcall!(video_destination::is_alive(site));
    println!("[video] is alive {:?}", is_alive);

    let video_control = std::ptr::null_mut();
    let ok = cppcall!(video_destination::start_streaming(
      site,
      VIDEO_SIZE.0,
      VIDEO_SIZE.1,
      sciter::video::COLOR_SPACE::Rgb32,
      video_control
    ));
    println!("[video] initialized: {:?}", ok);

    let figure = [0xFF_FFA500u32; (FRAGMENT_SIZE.0 * FRAGMENT_SIZE.1) as usize];

    let (mut calls, mut errors) = (0, 0);

    let mut x = 0;
    let mut y = 0;
    while cppcall!(video_destination::is_alive(site)) {
      calls += 1;
      if !cppcall!(fragmented_video_destination::render_frame_part(
        site,
        figure.as_ptr() as *const u8,
        figure.len() as u32 * 4,
        x,
        y,
        FRAGMENT_SIZE.0,
        FRAGMENT_SIZE.1
      )) {
        errors += 1;
      }
      // 25 FPS
      x += 1;
      y += 1;
      std::thread::sleep(std::time::Duration::from_millis(1000 / 25));
    }

    println!("[video] thread finished after {} / {} frames.", calls - errors, calls);
  }
}

impl sciter::EventHandler for VideoGen {
  fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
    Some(EVENT_GROUPS::HANDLE_BEHAVIOR_EVENT)
  }

  fn detached(&mut self, _root: HELEMENT) {
    println!("[video] detached");
    if let Some(h) = self.thread.take() {
      h.join();
    }
  }

  fn on_event(
    &mut self,
    root: HELEMENT,
    source: HELEMENT,
    target: HELEMENT,
    code: BEHAVIOR_EVENTS,
    phase: PHASE_MASK,
    reason: EventReason,
  ) -> bool {
    if phase != PHASE_MASK::BUBBLING {
      return false;
    }

    match code {
      BEHAVIOR_EVENTS::VIDEO_BIND_RQ => {
        let source = Element::from(source);
        println!("[video] {:?} {:?} ({:?})", source, code, reason);

        if let EventReason::VideoBind(ptr) = reason {
          if ptr.is_null() {
            // first phase, consume the event to mark as we will provide frames
            return true;
          }

          use sciter::video::*;

          // attach to a video destination interface
          let site = AssetPtr::attach(ptr as *mut video_destination);

          // query a fragmented video destination interface
          let mut fragmented: *mut iasset = std::ptr::null_mut();
          let mut target = &mut fragmented as *mut _;
          let ok = cppcall!(iasset::get_interface(site, INAME_VIDEO_FRAGMENTED_DESTINATION.as_ptr(), target));

          println!("query fragmented: {:?} ({:?})", ok, target);
          if ok && !fragmented.is_null() {
            // use this one
            println!("run video thread");
            let fragmented = fragmented as *mut fragmented_video_destination;
            let fragmented = AssetPtr::adopt(fragmented);
            let tid = ::std::thread::spawn(|| VideoGen::generation_thread(fragmented));
            self.thread = Some(tid);
          }
        }
      }

      BEHAVIOR_EVENTS::VIDEO_INITIALIZED => {
        eprintln!("{:?}", code);
      }

      BEHAVIOR_EVENTS::VIDEO_STARTED => {
        eprintln!("{:?}", code);
      }

      BEHAVIOR_EVENTS::VIDEO_STOPPED => {
        eprintln!("{:?}", code);
      }

      _ => return false,
    };

    return true;
  }
}

fn main() {
  if cfg!(all(target_os = "windows", target_arch = "x86")) {
    eprintln!("\nerror: Sciter video will not work on Windows x86.");
    eprintln!("error: Consider using a nightly Rust version to enable `abi_thiscall`,");
    eprintln!("error: see the https://github.com/rust-lang/rust/issues/42202.");
    std::process::exit(126);
  }

  use sciter::window;
  let mut frame = window::Window::with_size((750, 750), window::Flags::main_window(true));
  let handler = Handler::default();
  frame.sciter_handler(handler);
  frame.set_title("Video renderer sample");
  frame.load_html(include_bytes!("video.htm"), None);
  frame.run_app();
}
