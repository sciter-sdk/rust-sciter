// This component uses Sciter Engine,
// copyright Terra Informatica Software, Inc.
// (http://terrainformatica.com/).

/*!
# Rust bindings library for Sciter engine.

Sciter is an embeddable [multiplatform](https://sciter.com/sciter/crossplatform/) HTML/CSS/script engine
with GPU accelerated rendering designed to render modern desktop application UI.
It's a compact, single dll/dylib/so file (4-8 mb), engine without any additional dependencies.

Check the [screenshot gallery](https://github.com/oskca/sciter#sciter-desktop-ui-examples) of the desktop UI examples.

Sciter supports all standard elements defined in HTML5 specification [with some additions](https://sciter.com/developers/for-web-programmers/).
CSS extended to better support Desktop UI development, e.g. flow and flex units, vertical and horizontal alignment, OS theming.

[Sciter SDK](https://sciter.com/download/) comes with demo "browser" with builtin DOM inspector, script debugger and documentation browser:

![Sciter tools](https://sciter.com/images/sciter-tools.png)

Check <https://sciter.com> website and its [documentation resources](https://sciter.com/developers/) for engine principles, architecture and more.

## Brief look:

Here is a minimal sciter app:

```no_run
extern crate sciter;

fn main() {
    let mut frame = sciter::Window::new();
    frame.load_file("minimal.htm");
    frame.run_app();
}
```

It looks similar like this:

![Minimal sciter sample](https://i.imgur.com/ojcM5JJ.png)

Check [rust-sciter/examples](https://github.com/sciter-sdk/rust-sciter/tree/master/examples) folder for more complex usage
and module-level sections for the guides about:

* [Window](window/index.html) creation
* [Behaviors](dom/event/index.html) and event handling
* [DOM](dom/index.html) access methods
* Sciter [Value](value/index.html) interface

.
*/

#![doc(html_logo_url = "https://sciter.com/screenshots/slide-sciter-osx.png",
       html_favicon_url = "https://sciter.com/wp-content/themes/sciter/!images/favicon.ico")]

// documentation test:
// #![warn(missing_docs)]

/* Clippy lints */

#![cfg_attr(feature = "cargo-clippy", allow(needless_return, let_and_return))]


/* Macros */

#[cfg(target_os="macos")]
#[macro_use] extern crate objc;

#[macro_use] extern crate lazy_static;


#[macro_use] mod macros;

mod capi;
pub use capi::scdom::{HELEMENT};
pub use capi::scdef::{SCITER_RT_OPTIONS, GFX_LAYER};

/* Rust interface */
mod platform;
mod eventhandler;

pub mod window;
pub mod host;
pub mod value;
pub mod utf;
pub mod dom;
pub mod types;

pub use dom::Element;
pub use dom::event::EventHandler;
pub use host::{Archive, Host, HostHandler};
pub use value::{Value, FromValue};
pub use window::Window;



/* Loader */
pub use capi::scapi::{ISciterAPI};


#[cfg(windows)]
mod ext {
	// Note:
	// Sciter 4.x shipped with universal "sciter.dll" library for different builds:
	// bin/32, bin/64, bin/skia32, bin/skia64
	// However it is quiet unconvenient now (e.g. we can not put x64 and x86 builds in %PATH%)
	//
	#![allow(non_snake_case, non_camel_case_types)]
	use capi::scapi::{ISciterAPI};
	use capi::sctypes::{LPCSTR, LPCVOID, UINT};

	type SciterAPI_ptr = extern "system" fn () -> *const ISciterAPI;

	extern "system"
	{
		fn LoadLibraryA(lpFileName: LPCSTR) -> LPCVOID;
		fn GetProcAddress(hModule: LPCVOID, lpProcName: LPCSTR) -> LPCVOID;
		fn GetLastError() -> UINT;
	}

	pub unsafe fn SciterAPI() -> *const ISciterAPI {
		let dll = LoadLibraryA(b"sciter.dll\0".as_ptr() as LPCSTR);
		let err = GetLastError();
		if dll.is_null() {
			let msg = "Please verify that Sciter SDK is installed and its binaries (SDK/bin, bin.osx or bin.gtk) available in the PATH.";
			panic!("fatal: '{}' was not found in PATH (Error {})\n  {}", "sciter.dll", err, msg);
		}
		let func = GetProcAddress(dll, b"SciterAPI\0".as_ptr() as LPCSTR);
		if func.is_null() {
			panic!("Where is \"SciterAPI\"? It is expected to be in sciter.dll.");
		}
		let get_api: SciterAPI_ptr = ::std::mem::transmute(func);
		return get_api();
	}
}

#[cfg(target_os="linux")]
mod ext {
	// Note:
	// Since 4.1.4 library name has been changed to "libsciter-gtk" (without 32/64 suffix).
	// Since 3.3.1.6 library name was changed to "libsciter".
	// However CC requires `-l sciter` form.
	#[link(name="sciter-gtk")]
	extern "system" { pub fn SciterAPI() -> *const ::capi::scapi::ISciterAPI;	}
}

#[cfg(all(target_os="macos", target_arch="x86_64"))]
mod ext {
	#[link(name="sciter-osx-64", kind = "dylib")]
	extern "system" { pub fn SciterAPI() -> *const ::capi::scapi::ISciterAPI;	}
}

#[allow(non_snake_case)]
#[doc(hidden)]
/// Getting ISciterAPI reference, can be used for manual API calling.
pub fn SciterAPI<'a>() -> &'a ISciterAPI {
	let ap = unsafe {
		let p = ext::SciterAPI();
		&*p
	};
	return ap;
}


lazy_static! {
	static ref _API: &'static ISciterAPI = { SciterAPI() };
}

/// Sciter engine version number (e.g. `0x03030200`).
pub fn version_num() -> u32 {
	let v1 = (_API.SciterVersion)(true);
	let v2 = (_API.SciterVersion)(false);
	let num = ((v1 >> 16) << 24) | ((v1 & 0xFFFF) << 16) | ((v2 >> 16) << 8) | (v2 & 0xFFFF);
	return num;
}

/// Sciter engine version string (e.g. "`3.3.2.0`").
pub fn version() -> String {
	let v1 = (_API.SciterVersion)(true);
	let v2 = (_API.SciterVersion)(false);
	let num = [v1 >> 16, v1 & 0xFFFF, v2 >> 16, v2 & 0xFFFF];
	let version = format!("{}.{}.{}.{}", num[0], num[1], num[2], num[3]);
	return version;
}

/// Set various sciter engine global options, see the [`SCITER_RT_OPTIONS`](enum.SCITER_RT_OPTIONS.html).
pub fn set_option(option: SCITER_RT_OPTIONS, value: usize) -> std::result::Result<(), ()> {
	let ok = (_API.SciterSetOption)(std::ptr::null_mut(), option, value);
	if ok != 0 { Ok(()) } else { Err(()) }
}
