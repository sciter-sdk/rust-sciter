//! DOM access methods, C interface.

#![allow(non_camel_case_types, non_snake_case)]

use capi::sctypes::*;

MAKE_HANDLE!(HELEMENT, _HELEMENT);
MAKE_HANDLE!(HNODE, _HNODE);

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
/// Type of the result value for Sciter DOM functions.
pub enum SCDOM_RESULT {
	/// Function completed successfully.
	OK = 0,
	/// Invalid `HWINDOW`.
	INVALID_HWND = 1,
	/// Invalid `HELEMENT`.
	INVALID_HANDLE = 2,
	/// Attempt to use `HELEMENT` which is not attached to document.
	PASSIVE_HANDLE = 3,
	/// Parameter is invalid, e.g. pointer is null.
	INVALID_PARAMETER = 4,
	/// Operation failed, e.g. invalid html passed.
	OPERATION_FAILED = 5,
	/// Function completed successfully, but no result (e.g. no such attribute at element).
	OK_NOT_HANDLED = -1,
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
/// `dom::Element.set_html()` options.
pub enum SET_ELEMENT_HTML
{
	SIH_REPLACE_CONTENT     = 0,
	SIH_INSERT_AT_START     = 1,
	SIH_APPEND_AFTER_LAST   = 2,
	SOH_REPLACE             = 3,
	SOH_INSERT_BEFORE       = 4,
	SOH_INSERT_AFTER        = 5,
}

/// Bounding rectangle of the element.
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum ELEMENT_AREAS {

	/// `or` this flag if you want to get Sciter window relative coordinates,
	/// otherwise it will use nearest windowed container e.g. popup window.
	ROOT_RELATIVE = 0x01,

	/// `or` this flag if you want to get coordinates relative to the origin of element iself.
	SELF_RELATIVE = 0x02,

	/// Position inside immediate container.
	CONTAINER_RELATIVE = 0x03,

	/// Position relative to view - Sciter window.
	VIEW_RELATIVE = 0x04,

	/// Content (inner)  box.
	CONTENT_BOX = 0x00,

	/// Content + paddings.
	PADDING_BOX = 0x10,

	/// Content + paddings + border.
	BORDER_BOX  = 0x20,

	/// Content + paddings + border + margins.
	MARGIN_BOX  = 0x30,

	/// Relative to content origin - location of background image (if it set `no-repeat`).
	BACK_IMAGE_AREA = 0x40,

	/// Relative to content origin - location of foreground image (if it set `no-repeat`).
	FORE_IMAGE_AREA = 0x50,

	/// Scroll_area - scrollable area in content box.
	SCROLLABLE_AREA = 0x60,
}

impl ELEMENT_AREAS {
	/// Size of content (i.e `(0, 0, width, height)`).
	pub fn self_content() -> u32 {
		ELEMENT_AREAS::SELF_RELATIVE as u32
	}

	/// Size of rect (i.e `(left, top, width, height)`)
	pub fn self_rect() -> u32 {
		ELEMENT_AREAS::ROOT_RELATIVE as u32
	}
}

pub type SciterElementCallback = extern "system" fn (he: HELEMENT, param: LPVOID) -> BOOL;

pub type ELEMENT_COMPARATOR = extern "system" fn (he1: HELEMENT, he2: HELEMENT, param: LPVOID) -> INT;
