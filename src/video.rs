//! Sciter custom video rendering primitives.
use capi::sctypes::{UINT, LPCBYTE};

/// A type alias for Sciter functions that return `bool`.
pub type Result<T> = ::std::result::Result<T, ()>;


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
	Rgb32,
}

macro_rules! cppcall {
	// self.func()
	( $this:ident . $func:ident () ) => {
		unsafe {
			((*$this.vtbl).$func)($this as *mut _)
		}
	};

	// self.func(args...)
	($this:ident . $func:ident ( $( $arg:expr ),* )) => {
		unsafe {
			((*$this.vtbl).$func)($this as *mut _, $($arg),* )
		}
	};
}

macro_rules! cppresult {
	( $( $t:tt )* ) => {
		if cppcall!( $($t)* ) {
			Ok(())
		} else {
			Err(())
		}
	}
}


pub trait NamedInterface {
	fn get_interface_name() -> &'static [u8];

	fn query_interface(from: &mut iasset) -> Option<* mut iasset> {
		let mut out: *mut iasset = ::std::ptr::null_mut();
		from.get_interface(Self::get_interface_name().as_ptr(), &mut out as *mut _);
		if !out.is_null() {
			Some(out)
		} else {
			None
		}
	}
}

impl NamedInterface for video_source {
	fn get_interface_name() -> &'static [u8] {
		b"source.video.sciter.com\0"
	}
}

impl NamedInterface for video_destination {
	fn get_interface_name() -> &'static [u8] {
		b"destination.video.sciter.com\0"
	}
}

impl NamedInterface for fragmented_video_destination {
	fn get_interface_name() -> &'static [u8] {
		b"fragmented.destination.video.sciter.com\0"
	}
}


/// COM `IUnknown` alike thing.
#[repr(C)]
struct iasset_vtbl {
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
	vtbl: *const iasset_vtbl,
}

impl iasset {
	fn add_ref(&mut self) -> i32 {
		cppcall!(self.add_ref())
	}

	fn release(&mut self) -> i32 {
		cppcall!(self.release())
	}

	pub fn get_interface(&mut self, name: *const u8, out: *mut *mut iasset) -> bool {
		cppcall!(self.get_interface(name, out))
	}
}


/// Video source interface, used by engine to query video state.
#[repr(C)]
struct video_source_vtbl {
	// <-- iasset:
	/// Increments the reference count for an interface on an object.
	pub add_ref: extern "system" fn(this: *mut video_source) -> i32,

	/// Decrements the reference count for an interface on an object.
	pub release: extern "system" fn(this: *mut video_source) -> i32,

	/// Retrieves pointers to the supported interfaces on an object.
	pub get_interface: extern "system" fn(this: *mut video_source, name: *const u8, out: *mut *mut iasset) -> bool,
	// -->

	// <-- video_source
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
	// -->
}

/// Video source interface, used by engine to query video state.
#[repr(C)]
pub struct video_source {
	vtbl: *const video_source_vtbl,
}

impl video_source {
	pub fn play(&mut self) -> Result<()> {
		cppresult!(self.play())
	}
	pub fn pause(&mut self) -> Result<()> {
		cppresult!(self.pause())
	}
	pub fn stop(&mut self) -> Result<()> {
		cppresult!(self.stop())
	}

	pub fn is_ended(&mut self) -> Result<bool> {
		let mut r = false;
		cppresult!(self.get_is_ended(&mut r as *mut _)).map(|_| r)
	}

	pub fn get_position(&mut self) -> Result<f64> {
		let mut r = 0f64;
		cppresult!(self.get_position(&mut r as *mut _)).map(|_| r)
	}

	pub fn set_position(&mut self, seconds: f64) -> Result<()> {
		cppresult!(self.set_position(seconds))
	}

	pub fn get_duration(&mut self) -> Result<f64> {
		let mut r = 0f64;
		cppresult!(self.get_duration(&mut r as *mut _)).map(|_| r)
	}

	pub fn get_volume(&mut self) -> Result<f64> {
		let mut r = 0f64;
		cppresult!(self.get_volume(&mut r as *mut _)).map(|_| r)
	}

	pub fn set_volume(&mut self, volume: f64) -> Result<()> {
		cppresult!(self.set_volume(volume))
	}

	pub fn get_balance(&mut self) -> Result<f64> {
		let mut r = 0f64;
		cppresult!(self.get_balance(&mut r as *mut _)).map(|_| r)
	}

	pub fn set_balance(&mut self, balance: f64) -> Result<()> {
		cppresult!(self.set_balance(balance))
	}
}


/// Video destination interface, represents video rendering site.
#[repr(C)]
struct video_destination_vtbl {
	// <-- iasset:
	/// Increments the reference count for an interface on an object.
	pub add_ref: extern "system" fn(this: *mut video_destination) -> i32,

	/// Decrements the reference count for an interface on an object.
	pub release: extern "system" fn(this: *mut video_destination) -> i32,

	/// Retrieves pointers to the supported interfaces on an object.
	pub get_interface: extern "system" fn(this: *mut video_destination, name: *const u8, out: *mut *mut iasset) -> bool,
	// -->

	// <-- video_destination
	/// Whether this instance of `video_renderer` is attached to a DOM element and is capable of playing.
	pub is_alive: extern "system" fn(this: *mut video_destination) -> bool,

	/// Start streaming/rendering.
	pub start_streaming: extern "system" fn(this: *mut video_destination, frame_width: i32, frame_height: i32, color_space: COLOR_SPACE, src: *const video_source) -> bool,

	/// Stop streaming.
	pub stop_streaming: extern "system" fn(this: *mut video_destination) -> bool,

	/// Render the next frame.
	pub render_frame: extern "system" fn(this: *mut video_destination, data: LPCBYTE, size: UINT) -> bool,
	// -->
}

/// Video destination interface, represents video rendering site.
#[repr(C)]
pub struct video_destination {
	vtbl: *const video_destination_vtbl,
}

impl video_destination {

	/// Whether this instance of `video_renderer` is attached to a DOM element and is capable of playing.
	pub fn is_alive(&mut self) -> bool {
		cppcall!(self.is_alive())
	}

	/// Start streaming/rendering.
	pub fn start_streaming(&mut self, frame_size: (i32, i32), color_space: COLOR_SPACE, src: Option<&video_source>) -> Result<()> {
		let src_ptr = if let Some(ptr) = src { ptr as *const _ } else { ::std::ptr::null() };
		cppresult!(self.start_streaming(frame_size.0, frame_size.1, color_space, src_ptr))
	}

	/// Stop streaming.
	pub fn stop_streaming(&mut self) -> Result<()> {
		cppresult!(self.stop_streaming())
	}

	/// Render the next frame.
	pub fn render_frame(&mut self, data: &[u8]) -> Result<()> {
		cppresult!(self.render_frame(data.as_ptr(), data.len() as UINT))
	}
}


/// Fragmented destination interface, used for partial updates.
#[repr(C)]
struct fragmented_video_destination_vtbl {
	// <-- iasset:
	/// Increments the reference count for an interface on an object.
	pub add_ref: extern "system" fn(this: *mut fragmented_video_destination) -> i32,

	/// Decrements the reference count for an interface on an object.
	pub release: extern "system" fn(this: *mut fragmented_video_destination) -> i32,

	/// Retrieves pointers to the supported interfaces on an object.
	pub get_interface: extern "system" fn(this: *mut fragmented_video_destination, name: *const u8, out: *mut *mut iasset) -> bool,
	// -->

	// <-- video_destination
	/// Whether this instance of `video_renderer` is attached to a DOM element and is capable of playing.
	pub is_alive: extern "system" fn(this: *mut fragmented_video_destination) -> bool,

	/// Start streaming/rendering.
	pub start_streaming: extern "system" fn(this: *mut fragmented_video_destination, frame_width: i32, frame_height: i32, color_space: COLOR_SPACE, src: *const video_source) -> bool,

	/// Stop streaming.
	pub stop_streaming: extern "system" fn(this: *mut fragmented_video_destination) -> bool,

	/// Render the next frame.
	pub render_frame: extern "system" fn(this: *mut fragmented_video_destination, data: LPCBYTE, size: UINT) -> bool,
	// -->

	// <-- fragmented_video_destination
	/// Render the specified part of the current frame.
	pub render_frame_part: extern "system" fn(this: *mut fragmented_video_destination, data: LPCBYTE, size: UINT, x: i32, y: i32, width: i32, height: i32) -> bool,
	// -->
}

/// Fragmented destination interface, used for partial updates.
#[repr(C)]
pub struct fragmented_video_destination {
	vtbl: *const fragmented_video_destination_vtbl,
}

impl fragmented_video_destination {

	/// Whether this instance of `video_renderer` is attached to a DOM element and is capable of playing.
	pub fn is_alive(&mut self) -> bool {
		cppcall!(self.is_alive())
	}

	/// Start streaming/rendering.
	pub fn start_streaming(&mut self, frame_size: (i32, i32), color_space: COLOR_SPACE, src: Option<&video_source>) -> Result<()> {
		let src_ptr = if let Some(ptr) = src { ptr as *const _ } else { ::std::ptr::null() };
		cppresult!(self.start_streaming(frame_size.0, frame_size.1, color_space, src_ptr))
	}

	/// Stop streaming.
	pub fn stop_streaming(&mut self) -> Result<()> {
		cppresult!(self.stop_streaming())
	}

	/// Render the next frame.
	pub fn render_frame(&mut self, data: &[u8]) -> Result<()> {
		cppresult!(self.render_frame(data.as_ptr(), data.len() as UINT))
	}

	/// Render the specified part of the current frame.
	pub fn render_frame_part(&mut self, data: &[u8], update_point: (i32, i32), update_size: (i32, i32)) -> Result<()> {
		cppresult!(self.render_frame_part(data.as_ptr(), data.len() as UINT, update_point.0, update_point.1, update_size.0, update_size.1))
	}
}

/// A managed `iasset` pointer.
pub struct AssetPtr<T> {
	ptr: *mut T,
}

/// It's okay to transfer video pointers between threads.
unsafe impl<T> Send for AssetPtr<T> {}

use ::std::ops::{Deref, DerefMut};

impl Deref for AssetPtr<video_destination> {
	type Target = video_destination;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.ptr }
	}
}

impl DerefMut for AssetPtr<video_destination> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.ptr }
	}
}

impl Deref for AssetPtr<fragmented_video_destination> {
	type Target = fragmented_video_destination;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.ptr }
	}
}

impl DerefMut for AssetPtr<fragmented_video_destination> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.ptr }
	}
}

/// Decrements the reference count of a managed pointer.
impl<T> Drop for AssetPtr<T> {
	fn drop(&mut self) {
		self.get().release();
	}
}

impl<T> AssetPtr<T> {
	/// Attach to an existing pointer without reference increment.
	fn attach(lp: *mut T) -> Self {
		assert!(!lp.is_null());
		Self {
			ptr: lp
		}
	}

	/// Attach to a pointer and increment its reference count.
	pub fn adopt(lp: *mut T) -> Self {
		let mut me = Self::attach(lp);
		me.get().add_ref();
		me
	}

	/// Get as an `iasset` type.
	fn get(&mut self) -> &mut iasset {
		let ptr = self.ptr as *mut iasset;
		unsafe { &mut *ptr }
	}
}

/// Attempt to construct `Self` via a conversion.
impl<T: NamedInterface> AssetPtr<T> {

	/// Retrieve a supported interface of the managed pointer.
	///
	/// Example:
	///
	/// ```rust,no_run
	/// # use sciter::video::{AssetPtr, iasset, video_source};
	/// # let external_ptr: *mut iasset = ::std::ptr::null_mut();
	/// let mut site = AssetPtr::adopt(external_ptr);
	/// let source = AssetPtr::<video_source>::try_from(&mut site);
	/// assert!(source.is_ok());
	/// ```
	pub fn try_from<U>(other: &mut AssetPtr<U>) -> Result<Self> {
		let me = T::query_interface(other.get());
		me.map(|p| AssetPtr::adopt(p as *mut T)).ok_or(())
	}
}
