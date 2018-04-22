/*! Sciter's platform independent graphics interface.

Used in custom behaviors / event handlers to draw on element's surface in native code.
Essentially this mimics [`Graphics`](https://sciter.com/docs/content/sciter/Graphics.htm) scripting object as close as possible.

*/
use capi::scgraphics::{GRAPHIN_RESULT, HGFX, HIMG, SC_COLOR};
use capi::sctypes::{BOOL, LPCBYTE, LPVOID, UINT};
use std::ptr::null_mut;
use value::{FromValue, Value};
use _GAPI;

/// Image encoding used in `Image.save`.
#[derive(Debug, PartialEq)]
pub enum ImageEncoding {
  Raw,
  Png,
  Jpg,
  Webp,
}

macro_rules! ok_or {
  ($rv:expr, $ok:ident) => {
    if $ok == GRAPHIN_RESULT::OK {
      Ok($rv)
    } else {
      Err($ok)
    }
  };
}

/// A specialized `Result` type for graphics operations.
pub type Result<T> = ::std::result::Result<T, GRAPHIN_RESULT>;

/// A color type in the `ARGB` form.
pub type Color = SC_COLOR;

/// Image painter.
pub trait Painter {
  /// Paint on image using `Graphics`.
  fn paint(&mut self, gfx: &mut Graphics, width: u32, height: u32);
}

pub struct Image(HIMG);
pub struct Graphics(HGFX);

impl Image {
  /// Create a new image.
  pub fn create(width: u32, height: u32, with_alpha: bool) -> ::std::result::Result<Image, GRAPHIN_RESULT> {
    let mut h = null_mut();
    let ok = (_GAPI.imageCreate)(&mut h, width, height, with_alpha as BOOL);
    ok_or!(Image(h), ok)
  }

  /// Create image from `BGRA` data. Size of pixmap is `width*height*4` bytes.
  pub fn create_from(width: u32, height: u32, with_alpha: bool, pixmap: &[u8]) -> ::std::result::Result<Image, GRAPHIN_RESULT> {
    let mut h = null_mut();
    let ok = (_GAPI.imageCreateFromPixmap)(&mut h, width, height, with_alpha as BOOL, pixmap.as_ptr());
    ok_or!(Image(h), ok)
  }

  /// Load image from PNG or JPEG encoded data.
  pub fn load(data: &[u8]) -> ::std::result::Result<Image, GRAPHIN_RESULT> {
    let mut h = null_mut();
    let ok = (_GAPI.imageLoad)(data.as_ptr(), data.len() as UINT, &mut h);
    ok_or!(Image(h), ok)
  }

  pub fn save(&self, encoding: ImageEncoding, quality: u8) -> Result<Vec<u8>> {
    extern "system" fn on_save(prm: LPVOID, data: LPCBYTE, data_length: UINT) {
      unsafe {
        let param = prm as *mut Vec<u8>;
        assert!(param.is_null());
        assert!(data.is_null());
        let dst = &mut *param;
        let src = ::std::slice::from_raw_parts(data, data_length as usize);
        dst.extend_from_slice(src);
      }
    }
    use capi::scgraphics::SCITER_IMAGE_ENCODING::*;
    let enc = match encoding {
      ImageEncoding::Raw => RAW,
      ImageEncoding::Png => PNG,
      ImageEncoding::Jpg => JPG,
      ImageEncoding::Webp => WEBP,
    };
    let mut data = Vec::new();
    let ok = (_GAPI.imageSave)(self.0, on_save, &mut data as *mut _ as LPVOID, enc, quality as u32);
    ok_or!(data, ok)
  }

  /// Clear image by filling it with the black color.
  pub fn clear(&mut self) -> Result<()> {
    let ok = (_GAPI.imageClear)(self.0, 0);
    ok_or!((), ok)
  }

  /// Clear image by filling it with the specified `color`.
  pub fn clear_with(&mut self, color: Color) -> Result<()> {
    let ok = (_GAPI.imageClear)(self.0, color);
    ok_or!((), ok)
  }

  pub fn dimensions(&self) -> Result<(u32, u32)> {
    let mut alpha = 0;
    let mut w = 0;
    let mut h = 0;
    let ok = (_GAPI.imageGetInfo)(self.0, &mut w, &mut h, &mut alpha);
    ok_or!((w, h), ok)
  }

  pub fn paint<T: Painter>(&self, painter: T) -> Result<()> {
    extern "system" fn on_paint<T: Painter>(prm: LPVOID, hgfx: HGFX, width: UINT, height: UINT) {
      let param = prm as *mut T;
      assert!(!param.is_null());
      let me = unsafe { &mut *param };
      let mut gfx = Graphics(hgfx);
      me.paint(&mut gfx, width, height);
    }
    let param = Box::new(painter);
    let ok = (_GAPI.imagePaint)(self.0, on_paint::<T>, Box::into_raw(param) as LPVOID);
    ok_or!((), ok)
  }
}

/// Get an `Image` object contained in the `Value`.
impl FromValue for Image {
  fn from_value(v: &Value) -> Option<Image> {
    let mut h = null_mut();
    let ok = (_GAPI.vUnWrapImage)(v.as_cptr(), &mut h);
    if ok == GRAPHIN_RESULT::OK {
      Some(Image(h))
    } else {
      None
    }
  }
}

/// Store the `Image` object as a `Value`.
impl From<Image> for Value {
  fn from(i: Image) -> Value {
    let mut v = Value::new();
    let ok = (_GAPI.vWrapImage)(i.0, v.as_ptr());
    assert!(ok == GRAPHIN_RESULT::OK);
    v
  }
}

/// Destroy pointed image object.
impl Drop for Image {
  fn drop(&mut self) {
    (_GAPI.imageRelease)(self.0);
  }
}

/// Copies image.
///
/// All allocated objects are reference counted so copying is just a matter of increasing reference counts.
impl Clone for Image {
  fn clone(&self) -> Self {
    let dst = Image(self.0);
    (_GAPI.imageAddRef)(dst.0);
    dst
  }
}
