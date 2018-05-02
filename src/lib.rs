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

#![cfg_attr(feature = "cargo-clippy", allow(needless_return, let_and_return))] // past habits
#![cfg_attr(feature = "cargo-clippy", allow(redundant_field_names))] // since Rust 1.17 and less readable
// #![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))] // 0.0.195 only


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

pub mod dom;
pub mod graphics;
pub mod host;
pub mod types;
pub mod utf;
pub mod value;
pub mod video;
pub mod window;

pub use dom::Element;
pub use dom::event::EventHandler;
pub use host::{Archive, Host, HostHandler};
pub use value::{Value, FromValue};
pub use window::Window;

/// Builder pattern for window creation. See [`window::Builder`](window/struct.Builder.html) documentation.
///
/// For example,
///
/// ```rust,no_run
/// let mut frame = sciter::WindowBuilder::main_window()
///   .with_size((800,600))
///   .glassy()
///   .fixed()
///   .create();
/// ```
pub type WindowBuilder = window::Builder;


/* Loader */
pub use capi::scapi::{ISciterAPI};
use capi::scgraphics::SciterGraphicsAPI;
use capi::screquest::SciterRequestAPI;

#[cfg(windows)]
mod ext {
	// Note:
	// Sciter 4.x shipped with universal "sciter.dll" library for different builds:
	// bin/32, bin/64, bin/skia32, bin/skia64
	// However it is quite inconvenient now (e.g. we can not put x64 and x86 builds in %PATH%)
	//
	#![allow(non_snake_case, non_camel_case_types)]
	use capi::scapi::{ISciterAPI};
	use capi::sctypes::{LPCSTR, LPCVOID, BOOL};

  type ApiType = *const ISciterAPI;
	type FuncType = extern "system" fn () -> *const ISciterAPI;

  pub static mut CUSTOM_DLL_PATH: Option<String> = None;

	extern "system"
	{
		fn LoadLibraryA(lpFileName: LPCSTR) -> LPCVOID;
    fn FreeLibrary(dll: LPCVOID) -> BOOL;
		fn GetProcAddress(hModule: LPCVOID, lpProcName: LPCSTR) -> LPCVOID;
	}

  pub fn try_load_library(permanent: bool) -> ::std::result::Result<ApiType, String> {
    use ::std;
    use std::ffi::CString;
    use std::path::Path;

    fn try_load(path: &Path) -> Option<LPCVOID> {
      let path = CString::new(format!("{}", path.display())).expect("invalid library path");
      let dll = unsafe { LoadLibraryA(path.as_ptr()) };
      if !dll.is_null() {
        Some(dll)
      } else {
        None
      }
    }

    fn in_global() -> Option<LPCVOID> {
      // modern dll name
      let mut dll = unsafe { LoadLibraryA(b"sciter.dll\0".as_ptr() as LPCSTR) };
      if dll.is_null() {
        // try to load with old names
        let alternate = if cfg!(target_arch="x86_64") { b"sciter64.dll\0" } else { b"sciter32.dll\0" };
        dll = unsafe { LoadLibraryA(alternate.as_ptr() as LPCSTR) };
      }
      if !dll.is_null() {
        Some(dll)
      } else {
        None
      }
    }

    // try specified path first (and only if present)
    // and several paths to lookup then
    let dll = if let Some(path) = unsafe { CUSTOM_DLL_PATH.as_ref() } {
      try_load(Path::new(path))
    } else {
      in_global()
    };

    if let Some(dll) = dll {
      // get the "SciterAPI" exported symbol
      let sym = unsafe { GetProcAddress(dll, b"SciterAPI\0".as_ptr() as LPCSTR) };
      if sym.is_null() {
        return Err("\"SciterAPI\" function was expected in the loaded library.".to_owned());
      }

      if !permanent {
        unsafe { FreeLibrary(dll) };
        return Ok(0 as ApiType);
      }

      let get_api: FuncType = unsafe { std::mem::transmute(sym) };
      return Ok(get_api());
    }
    let sdkbin = if cfg!(target_arch="x86_64") { "bin/64" } else { "bin/32" };
    let msg = format!("Please verify that Sciter SDK is installed and its binaries (from SDK/{}) are available in PATH.", sdkbin);
    Err(format!("error: '{}' was not found neither in PATH nor near the current executable.\n  {}", "sciter.dll", msg))
  }

	pub unsafe fn SciterAPI() -> *const ISciterAPI {
    match try_load_library(true) {
      Ok(api) => api,
      Err(error) => panic!(error),
    }
	}
}

#[cfg(all(feature = "shared", unix))]
mod ext {
  #![allow(non_snake_case, non_camel_case_types)]
  extern crate libc;

  pub static mut CUSTOM_DLL_PATH: Option<String> = None;

  #[cfg(target_os="linux")]
  const DLL_NAMES: &'static [&'static str] = &[ "libsciter-gtk.so" ];

  #[cfg(all(target_os="macos", target_arch="x86_64"))]
  const DLL_NAMES: &'static [&'static str] = &[ "sciter-osx-64.dylib" ];

  use capi::scapi::ISciterAPI;
  use capi::sctypes::{LPVOID, LPCSTR};

  type FuncType = extern "system" fn () -> *const ISciterAPI;
  type ApiType = *const ISciterAPI;


  pub fn try_load_library(permanent: bool) -> ::std::result::Result<ApiType, String> {
    use ::std;
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    use std::path::{Path, PathBuf};


    fn try_load(path: &Path) -> Option<LPVOID> {
      let bytes = path.as_os_str().as_bytes();
      if let Ok(cstr) = CString::new(bytes) {
        let dll = unsafe { libc::dlopen(cstr.as_ptr(), libc::RTLD_LOCAL | libc::RTLD_LAZY) };
        if !dll.is_null() {
          return Some(dll)
        }
      }
      None
    }

    fn try_load_from(dir: Option<&Path>) -> Option<LPVOID> {

      let dll = DLL_NAMES.iter()
        .map(|name| {
          let mut path = dir.map(Path::to_owned).unwrap_or(PathBuf::new());
          path.push(name);
          path
        })
        .map(|path| try_load(&path))
        .filter(|dll| dll.is_some())
        .nth(0)
        .map(|o| o.unwrap());

      if dll.is_some() {
        return dll;
      }

      if cfg!(target_os="macos") && dir.is_some() {
        // "(bundle folder)/Contents/Frameworks/"
        let mut path = dir.unwrap().to_owned();
        path.push("../Frameworks/sciter-osx-64.dylib");
        return try_load(&path);
      }
      None
    }

    fn in_current_dir() -> Option<LPVOID> {
      if let Ok(dir) = ::std::env::current_exe() {
        if let Some(dir) = dir.parent() {
          return try_load_from(Some(dir));
        }
      }
      None
    }

    fn in_global() -> Option<LPVOID> {
      try_load_from(None)
    }

    // try specified path first (and only if present)
    // and several paths to lookup then
    let dll = if let Some(path) = unsafe { CUSTOM_DLL_PATH.as_ref() } {
      try_load(Path::new(path))
    } else {
      in_current_dir().or(in_global())
    };

    if let Some(dll) = dll {
      // get the "SciterAPI" exported symbol
      let sym = unsafe { libc::dlsym(dll, b"SciterAPI\0".as_ptr() as LPCSTR) };
      if sym.is_null() {
        return Err("\"SciterAPI\" function was expected in the loaded library.".to_owned());
      }

      if !permanent {
        unsafe { libc::dlclose(dll) };
        return Ok(0 as ApiType);
      }

      let get_api: FuncType = unsafe { std::mem::transmute(sym) };
      return Ok(get_api());
    }
    let sdkbin = if cfg!(target_os="macos") { "bin.osx" } else { "bin.gtk" };
    let msg = format!("Please verify that Sciter SDK is installed and its binaries (from {}) are available in PATH.", sdkbin);
    Err(format!("error: '{}' was not found neither in PATH nor near the current executable.\n  {}", DLL_NAMES[0], msg))
  }

  pub fn SciterAPI() -> *const ISciterAPI {
    match try_load_library(true) {
      Ok(api) => api,
      Err(error) => panic!(error),
    }
  }
}


#[cfg(all(target_os="linux", not(feature = "shared")))]
mod ext {
	// Note:
	// Since 4.1.4 library name has been changed to "libsciter-gtk" (without 32/64 suffix).
	// Since 3.3.1.6 library name was changed to "libsciter".
	// However CC requires `-l sciter` form.
	#[link(name="sciter-gtk")]
	extern "system" { pub fn SciterAPI() -> *const ::capi::scapi::ISciterAPI;	}
}

#[cfg(all(target_os="macos", target_arch="x86_64", not(feature = "shared")))]
mod ext {
	#[link(name="sciter-osx-64", kind = "dylib")]
	extern "system" { pub fn SciterAPI() -> *const ::capi::scapi::ISciterAPI;	}
}

/// Getting ISciterAPI reference, can be used for manual API calling.
#[doc(hidden)]
#[allow(non_snake_case)]
pub fn SciterAPI<'a>() -> &'a ISciterAPI {
	let ap = unsafe {
		let p = ext::SciterAPI();
		&*p
	};
	return ap;
}


lazy_static! {
	static ref _API: &'static ISciterAPI = { SciterAPI() };
	static ref _GAPI: &'static SciterGraphicsAPI = { unsafe { &*(SciterAPI().GetSciterGraphicsAPI)() } };
	static ref _RAPI: &'static SciterRequestAPI = { unsafe { &*(SciterAPI().GetSciterRequestAPI)() } };
}

/// Set the custom path to the Sciter dynamic library.
///
/// Must be called first before any other functions.
/// Returns `false` if library can not be loaded.
///
/// # Example
///
/// ```rust
/// if sciter::set_dll_path("~/lib/sciter/bin.gtk/x64/libsciter-gtk.so").is_ok() {
///   println!("loaded Sciter version {}", sciter::version());
/// }
/// ```
pub fn set_dll_path(custom_path: &str) -> ::std::result::Result<(), String> {
  #[cfg(not(feature="shared"))]
  fn set_impl(_: &str) -> ::std::result::Result<(), String> {
    Err("Don't use `sciter::set_dll_path` in static builds.\n  Build with feature \"shared\" instead.".to_owned())
  }

  #[cfg(feature="shared")]
  fn set_impl(path: &str) -> ::std::result::Result<(), String> {
    unsafe {
      ext::CUSTOM_DLL_PATH = Some(path.to_owned());
    }
    ext::try_load_library(false).map(|_| ())
  }

  set_impl(custom_path)
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

/// Various global sciter engine options.
#[derive(Copy, Clone)]
pub enum RuntimeOptions<'a> {
	/// global; value: milliseconds, connection timeout of http client.
	ConnectionTimeout(u32),
	/// global; value: 0 - drop connection, 1 - use builtin dialog, 2 - accept connection silently.
	OnHttpsError(u8),
	/// global; value = LPCBYTE, json - GPU black list, see: gpu-blacklist.json resource.
	GpuBlacklist(&'a str),
	/// global or per-window; value - combination of [SCRIPT_RUNTIME_FEATURES](enum.SCRIPT_RUNTIME_FEATURES.html) flags.
	ScriptFeatures(u8),
	/// global (must be called before any window creation); value - [GFX_LAYER](enum.GFX_LAYER.html).
	GfxLayer(GFX_LAYER),
	/// global or per-window; value - TRUE/FALSE
	DebugMode(bool),
	/// global; value - BOOL, TRUE - the engine will use "unisex" theme that is common for all platforms.
	/// That UX theme is not using OS primitives for rendering input elements.
	/// Use it if you want exactly the same (modulo fonts) look-n-feel on all platforms.
	UxTheming(bool),
}

/// Set various sciter engine global options, see the [`RuntimeOptions`](enum.RuntimeOptions.html).
pub fn set_options(options: RuntimeOptions) -> std::result::Result<(), ()> {
	use RuntimeOptions::*;
	use SCITER_RT_OPTIONS::*;
	let (option, value) = match options {
		ConnectionTimeout(ms) => (SCITER_CONNECTION_TIMEOUT, ms as usize),
		OnHttpsError(behavior) => (SCITER_HTTPS_ERROR, behavior as usize),
		GpuBlacklist(json) => (SCITER_SET_GPU_BLACKLIST, json.as_bytes().as_ptr() as usize),
		ScriptFeatures(mask) => (SCITER_SET_SCRIPT_RUNTIME_FEATURES, mask as usize),
		GfxLayer(backend) => (SCITER_SET_GFX_LAYER, backend as usize),
		DebugMode(enable) => (SCITER_SET_DEBUG_MODE, enable as usize),
		UxTheming(enable) => (SCITER_SET_UX_THEMING, enable as usize),
	};
	let ok = (_API.SciterSetOption)(std::ptr::null_mut(), option, value);
	if ok != 0 {
		Ok(())
	} else {
		Err(())
	}
}
