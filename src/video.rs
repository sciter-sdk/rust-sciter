//! Sciter custom video rendering primitives.
use capi::sctypes::{UINT, LPCBYTE};


pub static INAME_VIDEO_SOURCE: &[u8] = b"source.video.sciter.com\0";
pub static INAME_VIDEO_DESTINATION: &[u8] = b"destination.video.sciter.com\0";
pub static INAME_VIDEO_FRAGMENTED_DESTINATION: &[u8] = b"fragmented.destination.video.sciter.com\0";


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


/// COM `IUnknown` alike thing.
#[repr(C)]
pub struct iasset_vtbl {
	/// Increments the reference count for an interface on an object.
	pub add_ref: extern "system" fn(this: *mut iasset) -> i32,

	/// Decrements the reference count for an interface on an object.
	pub release: extern "system" fn(this: *mut iasset) -> i32,

	/// Retrieves pointers to the supported interfaces on an object.
	pub get_interface: extern "system" fn(this: *mut iasset, name: *const u8, out: *mut *mut iasset) -> bool,
}

/// COM `IUnknown` alike thing.
#[repr(C)]
pub struct iasset {
	pub vtbl: *const iasset_vtbl,
}


/// Video source interface, used by engine to query video state.
#[repr(C)]
pub struct video_source_vtbl {
	pub base: iasset,

	pub play: extern "system" fn(this: *mut video_source) -> bool,
	pub pause: extern "system" fn(this: *mut video_source) -> bool,
	pub stop: extern "system" fn(this: *mut video_source) -> bool,

	pub get_is_ended: extern "system" fn(this: *mut video_source, is_end: *mut bool) -> bool,

	pub get_position: extern "system" fn(this: *mut video_source, seconds: *mut f64) -> bool,
	pub set_position: extern "system" fn(this: *mut video_source, seconds: f64) -> bool,

	pub get_duration: extern "system" fn(this: *mut video_source, seconds: *mut f64) -> bool,

	pub get_volume: extern "system" fn(this: *mut video_source, volume: *mut f64) -> bool,
	pub set_volume: extern "system" fn(this: *mut video_source, volume: f64) -> bool,

	pub get_balance: extern "system" fn(this: *mut video_source, balance: *mut f64) -> bool,
	pub set_balance: extern "system" fn(this: *mut video_source, balance: f64) -> bool,
}

/// Video source interface, used by engine to query video state.
#[repr(C)]
pub struct video_source {
	pub vtbl: *const video_source_vtbl,
}


/// Video destination interface, represents video rendering site.
#[repr(C)]
pub struct video_destination_vtbl {
	pub base: iasset,

	/// Whether this instance of `video_renderer` is attached to a DOM element and is capable of playing.
	pub is_alive: extern "system" fn(this: *mut video_destination) -> bool,

	/// Start streaming/rendering.
	pub start_streaming: extern "system" fn(this: *mut video_destination, frame_width: i32, frame_height: i32, color_space: COLOR_SPACE, src: *const video_source) -> bool,

	/// Stop streaming.
	pub stop_streaming: extern "system" fn(this: *mut video_destination) -> bool,

	/// Render the next frame.
	pub render_frame: extern "system" fn(this: *mut video_destination, data: LPCBYTE, size: UINT) -> bool,
}

/// Video destination interface, represents video rendering site.
#[repr(C)]
pub struct video_destination {
	pub vtbl: *const video_destination_vtbl,
}


/// Fragmented destination interface, used for partial updates.
#[repr(C)]
pub struct fragmented_video_destination_vtbl {
	pub base: video_destination,

	/// Render the specified part of the current frame.
	pub render_frame_part: extern "system" fn(this: *mut fragmented_video_destination, data: LPCBYTE, size: UINT, x: i32, y: i32, width: i32, height: i32) -> bool,
}

/// Fragmented destination interface, used for partial updates.
#[repr(C)]
pub struct fragmented_video_destination {
	pub vtbl: *const fragmented_video_destination_vtbl,
}


pub struct AssetPtr<T> {
	ptr: *mut T,
}

impl<T> Drop for AssetPtr<T> {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			unsafe {
				let lp = self.ptr as *mut iasset;
				let vtbl = (*lp).vtbl;
				((*vtbl).add_ref)(lp);
			}
		}
	}
}

impl<T> AssetPtr<T> {
	pub fn new(lp: *mut T) -> Self {
		if !lp.is_null() {
			unsafe {
				let lp = lp as *mut iasset;
				let vtbl = (*lp).vtbl;
				((*vtbl).add_ref)(lp);
			}
		}
		Self {
			ptr: lp
		}
	}

	pub fn attach(lp: *mut T) -> Self {
		Self {
			ptr: lp
		}
	}

	pub fn is_null(&self) -> bool {
		self.ptr.is_null()
	}

	pub fn get(&self) -> *mut T {
		self.ptr
	}
}
