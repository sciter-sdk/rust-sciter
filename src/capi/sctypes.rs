//! Sciter platform-dependent types.

#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

use self::libc::*;


// common
MAKE_HANDLE!(HWINDOW, _HWINDOW); // HWND or NSView* or GtkWidget*
MAKE_HANDLE!(HSARCHIVE, _HSARCHIVE);

pub type BYTE = uint8_t;
pub type INT = int32_t;
pub type LONG = int32_t;
pub type UINT = uint32_t;
pub type INT64 = int64_t;
pub type UINT64 = uint64_t;

pub type FLOAT_VALUE = f64;

pub type WPARAM = size_t;
pub type LPARAM = ssize_t;

pub type UINT_PTR = uintptr_t;
pub type LRESULT = ssize_t;

pub type CHAR = c_char;
pub type LPSTR = *mut CHAR;
pub type LPCSTR = *const CHAR;

pub type WCHAR = uint16_t;
pub type LPWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;

pub type LPCBYTE = *const BYTE;
pub type LPUINT = *mut UINT;

pub type VOID = c_void;
pub type LPVOID = *mut VOID;
pub type LPCVOID = *const VOID;

#[cfg(windows)]
pub type BOOL = int32_t;

#[cfg(not(windows))]
pub type BOOL = int8_t;

pub type PBOOL = *mut BOOL;


#[repr(C)]
#[derive(Default, Debug)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub type LPRECT = *mut RECT;
pub type LPCRECT = *const RECT;

impl RECT {
	/// Calculate height of rect.
	pub fn height(&self) -> LONG {
		self.bottom - self.top
	}

	/// Calculate width of rect.
	pub fn width(&self) -> LONG {
		self.right - self.left
	}

	/// Return the size of rect in width and height form.
	pub fn size(&self) -> SIZE {
		SIZE {
			cx: self.width(),
			cy: self.height(),
		}
	}

	/// Returns the top-left point of rect.
	pub fn topleft(&self) -> POINT {
		POINT {
			x: self.left,
			y: self.top,
		}
	}
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
pub type LPPOINT = *mut POINT;


#[repr(C)]
#[derive(Default, Debug)]
pub struct SIZE {
    pub cx: LONG,
    pub cy: LONG,
}
pub type LPSIZE = *mut SIZE;


#[cfg(windows)]
#[repr(C)]
#[derive(Debug)]
pub struct MSG {
    pub hwnd: HWINDOW,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: UINT,
    pub pt: POINT,
}
#[cfg(windows)]
pub type LPMSG = *mut MSG;

