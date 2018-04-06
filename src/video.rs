//! Sciter custom video rendering primitives.
use capi::sctypes::{LONG, UINT, LPCSTR, LPCBYTE};


#[repr(C)]
pub enum COLOR_SPACE {
	Unknown,
	Yv12,
	Iyuv, // a.k.a. I420
	Nv12,
	Yuy2,
	Rgb24,
	Rgb555,
	Rgb565,
	Rgb32, // with alpha, sic!
}


#[repr(C)]
pub struct iasset {
	pub add_ref: extern "system" fn() -> LONG,
	pub release: extern "system" fn() -> LONG,
	pub get_interface: extern "system" fn(name: LPCSTR, out: *mut *const iasset) -> bool,
}


#[repr(C)]
pub struct video_source {
	pub base: iasset,

	pub play: extern "system" fn() -> bool,
	pub pause: extern "system" fn() -> bool,
	pub stop: extern "system" fn() -> bool,

	pub get_is_ended: extern "system" fn(is_end: *mut bool) -> bool,

	pub get_position: extern "system" fn(seconds: *mut f64) -> bool,
	pub set_position: extern "system" fn(seconds: f64) -> bool,

	pub get_duration: extern "system" fn(seconds: *mut f64) -> bool,

	pub get_volume: extern "system" fn(volume: *mut f64) -> bool,
	pub set_volume: extern "system" fn(volume: f64) -> bool,

	pub get_balance: extern "system" fn(balance: *mut f64) -> bool,
	pub set_balance: extern "system" fn(balance: f64) -> bool,
}

/// Video_destination interface, represents video rendering site.
#[repr(C)]
pub struct video_destination {
	pub base: iasset,

	/// Whether this instance of `video_renderer` is attached to a DOM element and is capable of playing.
	pub is_alive: extern "system" fn() -> bool,

	/// Start streaming/rendering.
	pub start_streaming: extern "system" fn(frame_width: i32, frame_height: i32, color_space: i32, src: *const video_source) -> bool,

	/// Stop streaming.
	pub stop_streaming: extern "system" fn() -> bool,

	/// Render the next frame.
	pub render_frame: extern "system" fn(data: LPCBYTE, size: UINT) -> bool,
}

#[repr(C)]
pub struct fragmented_video_destination {
	pub base: video_destination,

	/// Render the specified part of the current frame.
	pub render_frame_part: extern "system" fn(data: LPCBYTE, size: UINT, x: i32, y: i32, width: i32, height: i32) -> bool,
}
