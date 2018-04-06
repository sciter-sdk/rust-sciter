//! Common Sciter declarations.

#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]

use capi::sctypes::*;
use capi::scvalue::{VALUE};
use capi::screquest::{HREQUEST};
use capi::scdom::{HELEMENT};

//////////////////////////////////////////////////////////////////////////////////
pub enum ID2D1RenderTarget {}
pub enum ID2D1Factory {}
pub enum IDWriteFactory {}
pub enum IDXGISwapChain {}
pub enum IDXGISurface {}


#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
/// `HostHandler::on_data_load()` result code.
///
/// This notification gives application a chance to override built-in loader and
/// implement loading of resources in its own way (for example images can be loaded from
/// database or other resource).
pub enum LOAD_RESULT {
	/// Do default loading if data not set.
  LOAD_DEFAULT,
  /// Discard request completely (data will not be loaded at document).
  LOAD_DISCARD,
  /// Data will be delivered later by the host application.
  LOAD_DELAYED,
  /// You return this result to indicate that your (the host) application took or
  /// will take care about `HREQUEST` in your code completely.
  LOAD_MYSELF,
}

#[repr(C)]
#[derive(Debug)]
#[allow(missing_docs)]
/// Script runtime options.
pub enum SCRIPT_RUNTIME_FEATURES
{
  ALLOW_FILE_IO = 0x0000_0001,
  ALLOW_SOCKET_IO = 0x0000_0002,
  ALLOW_EVAL = 0x0000_0004,
  ALLOW_SYSINFO = 0x0000_0008
}

#[repr(C)]
#[derive(Debug)]
#[allow(missing_docs)]
/// Explicitly set sciter graphics layer.
pub enum GFX_LAYER
{
	#[cfg(windows)]
  GFX_LAYER_GDI      = 1,

  #[cfg(osx)]
  GFX_LAYER_CG       = 1,

  #[cfg(unix)]
  GFX_LAYER_CAIRO    = 1,

  GFX_LAYER_WARP     = 2,
  GFX_LAYER_D2D      = 3,
  GFX_LAYER_SKIA_CPU = 4,
  GFX_LAYER_SKIA_OPENGL = 5,
  GFX_LAYER_AUTO     = 0xFFFF,
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
/// Various sciter engine options (global or per-window).
pub enum SCITER_RT_OPTIONS
{
	/// value:TRUE - enable, value:FALSE - disable, enabled by default.
  SCITER_SMOOTH_SCROLL = 1,
  /// global; value: milliseconds, connection timeout of http client.
  SCITER_CONNECTION_TIMEOUT = 2,
  /// global; value: 0 - drop connection, 1 - use builtin dialog, 2 - accept connection silently.
  SCITER_HTTPS_ERROR = 3,
  /// value: 0 - system default, 1 - no smoothing, 2 - std smoothing, 3 - clear type.
  SCITER_FONT_SMOOTHING = 4,
	/// Windows Aero support, value:
	/// 0 - normal drawing,
	/// 1 - window has transparent background after calls DwmExtendFrameIntoClientArea() or DwmEnableBlurBehindWindow().
  SCITER_TRANSPARENT_WINDOW = 6,
  /// global; value = LPCBYTE, json - GPU black list, see: gpu-blacklist.json resource.
  SCITER_SET_GPU_BLACKLIST  = 7,
  /// global or per-window; value - combination of [SCRIPT_RUNTIME_FEATURES](enum.SCRIPT_RUNTIME_FEATURES.html) flags.
  SCITER_SET_SCRIPT_RUNTIME_FEATURES = 8,
  /// global (must be called before any window creation); value - [GFX_LAYER](enum.GFX_LAYER.html).
  SCITER_SET_GFX_LAYER = 9,
  /// global or per-window; value - TRUE/FALSE
  SCITER_SET_DEBUG_MODE = 10,
  /// global; value - BOOL, TRUE - the engine will use "unisex" theme that is common for all platforms.
  /// That UX theme is not using OS primitives for rendering input elements.
  /// Use it if you want exactly the same (modulo fonts) look-n-feel on all platforms.
  SCITER_SET_UX_THEMING = 11,
  ///value - TRUE/FALSE - window uses per pixel alpha (e.g. WS_EX_LAYERED/UpdateLayeredWindow() window).
  SCITER_ALPHA_WINDOW  = 12,
}

#[repr(C)]
/// Window flags
pub enum SCITER_CREATE_WINDOW_FLAGS {
	/// child window only, if this flag is set all other flags ignored
  SW_CHILD      = (1 << 0),
  /// toplevel window, has titlebar
  SW_TITLEBAR   = (1 << 1),
  /// has resizeable frame
  SW_RESIZEABLE = (1 << 2),
  /// is tool window
  SW_TOOL       = (1 << 3),
  /// has minimize / maximize buttons
  SW_CONTROLS   = (1 << 4),
  /// glassy window ( DwmExtendFrameIntoClientArea on windows )
  SW_GLASSY     = (1 << 5),
  /// transparent window ( e.g. WS_EX_LAYERED on Windows )
  SW_ALPHA      = (1 << 6),
  /// main window of the app, will terminate the app on close
  SW_MAIN       = (1 << 7),
  /// the window is created as topmost window.
  SW_POPUP      = (1 << 8),
  /// make this window inspector ready
  SW_ENABLE_DEBUG = (1 << 9),
  /// it has its own script VM
  SW_OWNS_VM      = (1 << 10),
}

impl SCITER_CREATE_WINDOW_FLAGS {
	/// Main application window.
	pub fn main_window(resizeable: bool) -> SCITER_CREATE_WINDOW_FLAGS {
		let flags = SCITER_CREATE_WINDOW_FLAGS::SW_MAIN
							 | SCITER_CREATE_WINDOW_FLAGS::SW_CONTROLS
							 | SCITER_CREATE_WINDOW_FLAGS::SW_TITLEBAR;
		if resizeable {
			flags | SCITER_CREATE_WINDOW_FLAGS::SW_RESIZEABLE
		} else {
			flags
		}
	}

	/// Child window.
	pub fn child_window() -> SCITER_CREATE_WINDOW_FLAGS {
		SCITER_CREATE_WINDOW_FLAGS::SW_CHILD
	}

	/// Popup window.
	pub fn popup_window(title: bool, minmaxclose: bool, resizeable: bool) -> SCITER_CREATE_WINDOW_FLAGS {
		let mut flags = SCITER_CREATE_WINDOW_FLAGS::SW_POPUP;
		if title {
			flags = flags | SCITER_CREATE_WINDOW_FLAGS::SW_TITLEBAR;
		}
		if minmaxclose {
			flags = flags | SCITER_CREATE_WINDOW_FLAGS::SW_CONTROLS;
		}
		if resizeable {
			flags = flags | SCITER_CREATE_WINDOW_FLAGS::SW_RESIZEABLE;
		}
		return flags;
	}

	/// Tool window.
	pub fn tool_window(resizeable: bool) -> SCITER_CREATE_WINDOW_FLAGS {
		let mut flags = SCITER_CREATE_WINDOW_FLAGS::SW_TOOL;
		if resizeable {
			flags = flags | SCITER_CREATE_WINDOW_FLAGS::SW_RESIZEABLE;
		}
		return flags;
	}
}

/// Flags can be OR'ed as `SW_MAIN|SW_ALPHA`.
impl ::std::ops::BitOr for SCITER_CREATE_WINDOW_FLAGS {
  type Output = SCITER_CREATE_WINDOW_FLAGS;
  fn bitor(self, rhs: Self::Output) -> Self::Output {
    let rn = (self as UINT) | (rhs as UINT);
    unsafe { ::std::mem::transmute(rn) }
  }
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum SCITER_NOTIFICATION {
  SC_LOAD_DATA = 1,
  SC_DATA_LOADED = 2,
  SC_ATTACH_BEHAVIOR = 4,
  SC_ENGINE_DESTROYED = 5,
  SC_POSTED_NOTIFICATION = 6,
  SC_GRAPHICS_CRITICAL_FAILURE = 7,
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
/// Sciter resource type identifiers.
pub enum SCITER_RESOURCE_TYPE
{
  RT_DATA_HTML = 0,
  RT_DATA_IMAGE = 1,
  RT_DATA_STYLE = 2,
  RT_DATA_CURSOR = 3,
  RT_DATA_SCRIPT = 4,
  RT_DATA_RAW = 5,
  RT_DATA_FONT,
  RT_DATA_SOUND,    // wav bytes
}


#[repr(C)]
#[derive(Debug)]
/// Notifies that Sciter is about to download a referred resource.
pub struct SCN_LOAD_DATA
{
	/// `SC_LOAD_DATA` here.
  pub code: UINT,
  /// `HWINDOW` of the window this callback was attached to.
  pub hwnd: HWINDOW,

  /// [in] Zero terminated string, fully qualified uri, for example "http://server/folder/file.ext".
  pub uri: LPCWSTR,

  /// [in,out] pointer to loaded data to return. If data exists in the cache then this field contain pointer to it.
  pub outData: LPCBYTE,
  /// [in,out] loaded data size to return.
  pub outDataSize: UINT,

  /// [in] resource type category
  pub dataType: SCITER_RESOURCE_TYPE,

  /// [in] request handle that can be used with sciter request API.
  pub request_id: HREQUEST,

  /// [in] destination element for request.
  pub principal: HELEMENT,
  /// [in] source element for request.
  pub initiator: HELEMENT,
}

#[repr(C)]
#[derive(Debug)]
/// This notification indicates that external data (for example image) download process completed.
pub struct SCN_DATA_LOADED
{
	/// `SC_DATA_LOADED` here.
  pub code: UINT,
  /// `HWINDOW` of the window this callback was attached to.
  pub hwnd: HWINDOW,
  /// [in] zero terminated string, fully qualified uri, for example "http://server/folder/file.ext".
  pub uri: LPCWSTR,
  /// [in] pointer to loaded data.
  pub data: LPCBYTE,
  /// [in] loaded data size (in bytes).
  pub dataSize: UINT,
  /// [in] resource type category
  pub dataType: UINT,
  /// Download status code:
  ///
  /// * status = 0 and `dataSize == 0` - unknown error.
  /// * status = 100..505 - http response status, note: 200 - OK!
  /// * status > 12000 - wininet error code, see `ERROR_INTERNET_***` in wininet.h
  pub status: UINT,
}

#[repr(C)]
#[derive(Debug)]
/// This notification is sent on parsing the document and while processing elements
/// having non empty `style.behavior` attribute value.
pub struct SCN_ATTACH_BEHAVIOR
{
	/// `SC_ATTACH_BEHAVIOR` here.
  pub code: UINT,
  /// `HWINDOW` of the window this callback was attached to.
  pub hwnd: HWINDOW,

  /// [in] target DOM element handle
  pub element: HELEMENT,
  /// [in] zero terminated string, string appears as value of CSS `behavior: ` attribute.
  pub name: LPCSTR,
  /// [out] pointer to ElementEventProc function.
  pub elementProc: *mut ElementEventProc,
  /// [out] tag value, passed as is into pointer ElementEventProc function.
  pub elementTag: LPVOID,
}


#[repr(C)]
pub struct SCITER_CALLBACK_NOTIFICATION
{
	pub code: UINT,
	pub hwnd: HWINDOW,
}
pub type LPSCITER_CALLBACK_NOTIFICATION = *mut SCITER_CALLBACK_NOTIFICATION;

pub type SciterHostCallback = extern "system" fn (pns: LPSCITER_CALLBACK_NOTIFICATION, callbackParam: LPVOID) -> UINT;

pub type SciterWindowDelegate = extern "system" fn (hwnd: HWINDOW, msg: UINT, wParam: WPARAM, lParam: LPARAM, pParam: LPVOID, handled: * mut BOOL) -> LRESULT;

pub type ElementEventProc = extern "system" fn (tag: LPVOID, he: HELEMENT, evtg: UINT, prms: LPVOID) -> BOOL;

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
/// Debug output categories.
pub enum OUTPUT_SUBSYTEMS
{
	/// html parser & runtime
  DOM = 0,
  /// csss! parser & runtime
  CSSS,
  /// css parser
  CSS,
  /// TIS parser & runtime
  TIS,
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
#[allow(missing_docs)]
/// Debug output severity.
pub enum OUTPUT_SEVERITY
{
  INFO,
  WARNING,
  ERROR,
}

pub type DEBUG_OUTPUT_PROC = extern "system" fn (param: LPVOID, subsystem: OUTPUT_SUBSYTEMS, severity: OUTPUT_SEVERITY, text: LPCWSTR, text_length: UINT);

pub type LPCWSTR_RECEIVER = extern "system" fn (szstr: LPCWSTR, str_length: UINT, param: LPVOID);
pub type LPCSTR_RECEIVER  = extern "system" fn (szstr: LPCSTR,  str_length: UINT, param: LPVOID);
pub type LPCBYTE_RECEIVER = extern "system" fn (szstr: LPCBYTE, str_length: UINT, param: LPVOID);

pub type ELEMENT_BITMAP_RECEIVER = extern "system" fn (rgba: LPCBYTE, x: INT, y: INT, width: UINT, height: UINT, param: LPVOID);

#[repr(C)]
pub struct METHOD_PARAMS {
	methodID: UINT,
}

#[repr(C)]
pub struct REQUEST_PARAM {
  name: LPCWSTR,
  value: LPCWSTR,
}

pub type KeyValueCallback = extern "system" fn (param: LPVOID, pkey: *const VALUE, pval: *const VALUE) -> BOOL;
