//! First working example of the Sciter API.

extern crate sciter;

fn main() {
  // can be called as `examples/first ~/lib/libsciter.so`
  if cfg!(feature = "shared") {
    if let Some(arg) = std::env::args().nth(1) {
      if let Err(_) = sciter::set_options(sciter::RuntimeOptions::LibraryPath(&arg)) {
        panic!("Invalid library path specified.");
      }
    }
  }

  let arch = if cfg!(target_arch = "x86_64") { "x64" } else { "x86" };
  println!("calling SciterAPI {}", arch);
  let scapi = sciter::SciterAPI();

  println!("getting abi version");
  let abi_version = scapi.version;
  println!("sciter abi version: {:?}", abi_version);

  // let class_name = scapi.SciterClassName(); // this doesn't work with rust 1.7

  let class_name = sciter::utf::w2s((scapi.SciterClassName)());
  println!("sciter class name: {}", class_name);

  let v1 = (scapi.SciterVersion)(true);
  let v2 = (scapi.SciterVersion)(false);
  let num = [v1 >> 16, v1 & 0xFFFF, v2 >> 16, v2 & 0xFFFF];
  let version = num.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(".");
  println!("sciter version: {} {:?}", version, num);
}
