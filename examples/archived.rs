//! Sciter sample with archived resources.
#[macro_use]
extern crate sciter;

use sciter::host::{Archive, LOAD_RESULT, SCN_LOAD_DATA};

struct LoadHandler {
  archive: Archive,
}

impl LoadHandler {
  fn new(archive: &[u8]) -> Self {
    Self {
      archive: Archive::open(archive),
    }
  }
}

impl sciter::HostHandler for LoadHandler {
  fn on_data_load(&mut self, pnm: &mut SCN_LOAD_DATA) -> LOAD_RESULT {
    let uri = w2s!(pnm.uri);
    eprintln!("[handler] loading {:?}", uri);

    if uri.starts_with("this://app/") {
      if let Some(data) = self.archive.get(&uri) {
        self.data_ready(pnm.hwnd, &uri, data, None);
      } else {
        eprintln!("[handler] error: can't load {:?}", uri);
      }
    }
    return LOAD_RESULT::LOAD_DEFAULT;
  }
}

fn main() {
  let resources = include_bytes!("archived.dat");

  let handler = LoadHandler::new(resources);

  // just to be sure
  assert!(handler.archive.get("index.htm").is_some(), "no `index.htm`?");

  let mut frame = sciter::Window::with_size((600, 400), sciter::window::Flags::main_window(false));
  frame.sciter_handler(handler);
  frame.load_file("this://app/index.htm");
  frame.run_app();
}
