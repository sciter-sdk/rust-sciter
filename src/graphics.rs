/*! Sciter's platform independent graphics interface.

Used in custom behaviors / event handlers to draw on element's surface in native code.
Essentially this mimics [`Graphics`](https://sciter.com/docs/content/sciter/Graphics.htm) scripting object as close as possible.

*/
use capi::scgraphics::DRAW_PATH_MODE;
use capi::scgraphics::{HGFX, HIMG, HPATH, SC_ANGLE, SC_COLOR, SC_DIM, SC_POS};
use capi::sctypes::{BOOL, LPCBYTE, LPVOID, POINT, SIZE, UINT};
use std::ptr::null_mut;
use value::{FromValue, Value};
use _GAPI;

pub use capi::scgraphics::GRAPHIN_RESULT;

/// Image encoding used in [`Image.save`](struct.Image.html#method.save).
#[derive(Debug, PartialEq)]
pub enum SaveImageEncoding {
  /// Raw bitmap in a `[a,b,g,r, a,b,g,r, ...]` form.
  Raw,
  /// Portable Network Graphics format.
  Png,
  /// JPEG with the specified quality level (in range of `10..100`).
  Jpeg(u8),
  /// WebP with the specified quality level (in range of `10..100`).
  Webp(u8),
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

/// Position on a surface in `(x, y)` form.
pub type Pos = (SC_POS, SC_POS);

/// Size in `(width, height)` form.
pub type Size = (SC_DIM, SC_DIM);

/// Angle (in radians).
pub type Angle = SC_ANGLE;

/// Dimension.
pub type Dim = SC_DIM;

/// Graphics image object.
pub struct Image(HIMG);

/// Graphics path object.
pub struct Path(HPATH);

/// Graphics object. Represents graphic surface of the element.
pub struct Graphics(HGFX);

/// Construct a color value from the `red`, `green`, `blue` and `opacity` components.
pub fn color(red: u8, green: u8, blue: u8, opacity: Option<u8>) -> Color {
  (_GAPI.RGBA)(u32::from(red), u32::from(green), u32::from(blue), u32::from(opacity.unwrap_or(255)))
}
///////////////////////////////////////////////////////////////////////////////
// Image

/// Destroy pointed image object.
impl Drop for Image {
  fn drop(&mut self) {
    (_GAPI.imageRelease)(self.0);
  }
}

/// Copies image object.
///
/// All allocated objects are reference counted so copying is just a matter of increasing reference counts.
impl Clone for Image {
  fn clone(&self) -> Self {
    let dst = Image(self.0);
    (_GAPI.imageAddRef)(dst.0);
    dst
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

impl Image {
  /// Create a new blank image.
  pub fn new((width, height): (u32, u32), with_alpha: bool) -> Result<Image> {
    let mut h = null_mut();
    let ok = (_GAPI.imageCreate)(&mut h, width, height, with_alpha as BOOL);
    ok_or!(Image(h), ok)
  }

  /// Create image from `BGRA` data. Size of pixmap is `width*height*4` bytes.
  pub fn with_data((width, height): (u32, u32), with_alpha: bool, pixmap: &[u8]) -> Result<Image> {
    let mut h = null_mut();
    let ok = (_GAPI.imageCreateFromPixmap)(&mut h, width, height, with_alpha as BOOL, pixmap.as_ptr());
    ok_or!(Image(h), ok)
  }

  /// Load image from PNG or JPEG (or any other supported) encoded data.
  pub fn load(image_data: &[u8]) -> Result<Image> {
    let mut h = null_mut();
    let ok = (_GAPI.imageLoad)(image_data.as_ptr(), image_data.len() as UINT, &mut h);
    ok_or!(Image(h), ok)
  }

  /// Save content of the image as a byte vector.
  pub fn save(&self, encoding: SaveImageEncoding) -> Result<Vec<u8>> {
    extern "system" fn on_save(prm: LPVOID, data: LPCBYTE, data_length: UINT) {
      assert!(!prm.is_null());
      assert!(!data.is_null());
      unsafe {
        let param = prm as *mut Vec<u8>;
        let dst = &mut *param;
        let src = ::std::slice::from_raw_parts(data, data_length as usize);
        dst.extend_from_slice(src);
      }
    }
    use capi::scgraphics::SCITER_IMAGE_ENCODING::*;
    let (enc, q) = match encoding {
      SaveImageEncoding::Raw => (RAW, 0),
      SaveImageEncoding::Png => (PNG, 0),
      SaveImageEncoding::Jpeg(q) => (JPG, q),
      SaveImageEncoding::Webp(q) => (WEBP, q),
    };
    let mut data = Vec::new();
    let ok = (_GAPI.imageSave)(self.0, on_save, &mut data as *mut _ as LPVOID, enc, q as u32);
    ok_or!(data, ok)
  }

  /// Render on image using methods of the [`Graphics`](struct.Graphics.html) object.
  ///
  /// `PaintFn` must be the following:
  /// `fn paint(gfx: &mut Graphics, (width, height): (f32, f32))`.
  ///
  /// # Example:
  ///
  /// ```rust
  /// # use sciter::graphics::Image;
  /// let image = Image::new((100, 100), false).unwrap();
  /// image.paint(|gfx, size| {
  ///   gfx.rectangle((5.0, 5.0), (size.0 - 5.0, size.1 - 5.0))?;
  ///   Ok(())
  ///	}).unwrap();
  /// ```
  pub fn paint<PaintFn>(&self, painter: PaintFn) -> Result<()>
  where
    PaintFn: Fn(&mut Graphics, (f32, f32)) -> Result<()>,
  {
    #[repr(C)]
    struct Payload<PaintFn> {
      painter: PaintFn,
      result: Result<()>,
    }
    extern "system" fn on_paint<PaintFn: Fn(&mut Graphics, (f32, f32)) -> Result<()>>(prm: LPVOID, hgfx: HGFX, width: UINT, height: UINT) {
      let param = prm as *mut Payload<PaintFn>;
      assert!(!param.is_null());
      let payload = unsafe { &mut *param };
      let mut gfx = Graphics(hgfx);
      let ok = (payload.painter)(&mut gfx, (width as f32, height as f32));
      payload.result = ok;
    }
    let payload = Payload {
      painter: painter,
      result: Ok(()),
    };
    let param = Box::new(payload);
    let param = Box::into_raw(param);
    let ok = (_GAPI.imagePaint)(self.0, on_paint::<PaintFn>, param as LPVOID);
    let ok = ok_or!((), ok);
    let param = unsafe { Box::from_raw(param) };
    ok.and(param.result)
  }

  /// Get width and height of the image (in pixels).
  pub fn dimensions(&self) -> Result<(u32, u32)> {
    let mut alpha = 0;
    let mut w = 0;
    let mut h = 0;
    let ok = (_GAPI.imageGetInfo)(self.0, &mut w, &mut h, &mut alpha);
    ok_or!((w, h), ok)
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
}

///////////////////////////////////////////////////////////////////////////////
// Path

/// Destroy pointed path object.
impl Drop for Path {
  fn drop(&mut self) {
    (_GAPI.pathRelease)(self.0);
  }
}

/// Copies path object.
///
/// All allocated objects are reference counted so copying is just a matter of increasing reference counts.
impl Clone for Path {
  fn clone(&self) -> Self {
    let dst = Path(self.0);
    (_GAPI.pathAddRef)(dst.0);
    dst
  }
}

/// Get a `Path` object contained in the `Value`.
impl FromValue for Path {
  fn from_value(v: &Value) -> Option<Path> {
    let mut h = null_mut();
    let ok = (_GAPI.vUnWrapPath)(v.as_cptr(), &mut h);
    if ok == GRAPHIN_RESULT::OK {
      Some(Path(h))
    } else {
      None
    }
  }
}

/// Store the `Path` object as a `Value`.
impl From<Path> for Value {
  fn from(i: Path) -> Value {
    let mut v = Value::new();
    let ok = (_GAPI.vWrapPath)(i.0, v.as_ptr());
    assert!(ok == GRAPHIN_RESULT::OK);
    v
  }
}

impl Path {
  /// Create a new empty path.
  pub fn new() -> Result<Path> {
    let mut h = null_mut();
    let ok = (_GAPI.pathCreate)(&mut h);
    ok_or!(Path(h), ok)
  }

  /// Close the current path/figure.
  pub fn close(&mut self) -> Result<()> {
    let ok = (_GAPI.pathClosePath)(self.0);
    ok_or!((), ok)
  }

  /// Move the current drawing path position to `x,y`.
  ///
  /// If `is_relative` is `true` then `x` and `y` are interpreted as deltas from the current path position.
  pub fn move_to(&mut self, point: Pos, is_relative: bool) -> Result<&mut Path> {
    let ok = (_GAPI.pathMoveTo)(self.0, point.0, point.1, is_relative as BOOL);
    ok_or!(self, ok)
  }

  /// Draw a line and move the current drawing path position to `x,y`.
  ///
  /// If `is_relative` is `true` then `x` and `y` are interpreted as deltas from the current path position.
  pub fn line_to(&mut self, point: Pos, is_relative: bool) -> Result<&mut Path> {
    let ok = (_GAPI.pathLineTo)(self.0, point.0, point.1, is_relative as BOOL);
    ok_or!(self, ok)
  }

  /// Draw an arc.
  pub fn arc_to(&mut self, xy: Pos, angle: Angle, rxy: Pos, is_large: bool, is_clockwise: bool, is_relative: bool) -> Result<&mut Path> {
    let ok = (_GAPI.pathArcTo)(
      self.0,
      xy.0,
      xy.1,
      angle,
      rxy.0,
      rxy.1,
      is_large as BOOL,
      is_clockwise as BOOL,
      is_relative as BOOL,
    );
    ok_or!(self, ok)
  }

  /// Draw a quadratic Bézier curve.
  ///
  /// If `is_relative` is `true` then `x` and `y` are interpreted as deltas from the current path position.
  pub fn quadratic_curve_to(&mut self, control: Pos, end: Pos, is_relative: bool) -> Result<&mut Path> {
    let ok = (_GAPI.pathQuadraticCurveTo)(self.0, control.0, control.1, end.0, end.1, is_relative as BOOL);
    ok_or!(self, ok)
  }

  /// Draw a quadratic Bézier curve.
  ///
  /// If `is_relative` is `true` then `x` and `y` are interpreted as deltas from the current path position.
  pub fn bezier_curve_to(&mut self, control1: Pos, control2: Pos, end: Pos, is_relative: bool) -> Result<&mut Path> {
    let ok = (_GAPI.pathBezierCurveTo)(
      self.0,
      control1.0,
      control1.1,
      control2.0,
      control2.1,
      end.0,
      end.1,
      is_relative as BOOL,
    );
    ok_or!(self, ok)
  }
}

///////////////////////////////////////////////////////////////////////////////
// Graphics

/// Destroy pointed graphics object.
impl Drop for Graphics {
  fn drop(&mut self) {
    (_GAPI.gRelease)(self.0);
  }
}

/// Copies graphics object.
///
/// All allocated objects are reference counted so copying is just a matter of increasing reference counts.
impl Clone for Graphics {
  fn clone(&self) -> Self {
    let dst = Graphics(self.0);
    (_GAPI.gAddRef)(dst.0);
    dst
  }
}

/// Get an `Graphics` object contained in the `Value`.
impl FromValue for Graphics {
  fn from_value(v: &Value) -> Option<Graphics> {
    let mut h = null_mut();
    let ok = (_GAPI.vUnWrapGfx)(v.as_cptr(), &mut h);
    if ok == GRAPHIN_RESULT::OK {
      Some(Graphics(h))
    } else {
      None
    }
  }
}

/// Store the `Graphics` object as a `Value`.
impl From<Graphics> for Value {
  fn from(i: Graphics) -> Value {
    let mut v = Value::new();
    let ok = (_GAPI.vWrapGfx)(i.0, v.as_ptr());
    assert!(ok == GRAPHIN_RESULT::OK);
    v
  }
}

impl Graphics {
  /// Save the current graphics attributes on top of the internal state stack.
  pub fn push_state(&mut self) -> Result<()> {
    let ok = (_GAPI.gStateSave)(self.0);
    ok_or!((), ok)
  }

  /// Restore graphics attributes from top of the internal state stack.
  pub fn pop_state(&mut self) -> Result<()> {
    let ok = (_GAPI.gStateRestore)(self.0);
    ok_or!((), ok)
  }

  /// Draws a line from the `start` to the `end` points using the current stroke brushes.
  pub fn line(&mut self, start: Pos, end: Pos) -> Result<&mut Self> {
    let ok = (_GAPI.gLine)(self.0, start.0, start.1, end.0, end.1);
    ok_or!(self, ok)
  }

  /// Draw a rectangle using the current fill and stroke brushes.
  pub fn rectangle(&mut self, left_top: Pos, right_bottom: Pos) -> Result<&mut Self> {
    let ok = (_GAPI.gRectangle)(self.0, left_top.0, left_top.1, right_bottom.0, right_bottom.1);
    ok_or!(self, ok)
  }

  /// Draw a rounded rectangle using the current fill and stroke brushes.
  pub fn round_rect(&mut self, left_top: Pos, right_bottom: Pos, radius: Dim) -> Result<&mut Self> {
    let rad: [Dim; 8] = [radius; 8usize];
    let ok = (_GAPI.gRoundedRectangle)(self.0, left_top.0, left_top.1, right_bottom.0, right_bottom.1, rad.as_ptr());
    ok_or!(self, ok)
  }

  /// Draw a rounded rectangle using the current fill and stroke brushes.
  pub fn round_rect4(&mut self, left_top: Pos, right_bottom: Pos, radius: (Dim, Dim, Dim, Dim)) -> Result<&mut Self> {
    let r = radius;
    let rad: [Dim; 8] = [r.0, r.0, r.1, r.1, r.2, r.2, r.3, r.3];
    let ok = (_GAPI.gRoundedRectangle)(self.0, left_top.0, left_top.1, right_bottom.0, right_bottom.1, rad.as_ptr());
    ok_or!(self, ok)
  }

  /// Draw an ellipse using the current fill and stroke brushes.
  pub fn ellipse(&mut self, xy: Pos, radii: Pos) -> Result<&mut Self> {
    let ok = (_GAPI.gEllipse)(self.0, xy.0, xy.1, radii.0, radii.1);
    ok_or!(self, ok)
  }

  /// Draw a circle using the current fill and stroke brushes.
  pub fn circle(&mut self, xy: Pos, radius: Dim) -> Result<&mut Self> {
    let ok = (_GAPI.gEllipse)(self.0, xy.0, xy.1, radius, radius);
    ok_or!(self, ok)
  }

  // TODO: the rest of drawing operations
  // TODO: drawing attributes
  // TODO: affine transformations

  /// Draw the path object using current fill and stroke brushes.
  pub fn draw_path(&mut self, path: &Path, mode: DRAW_PATH_MODE) -> Result<&mut Self> {
    let ok = (_GAPI.gDrawPath)(self.0, path.0, mode);
    ok_or!(self, ok)
  }

  /// Draw the whole image onto the graphics surface.
  ///
  /// With the current transformation applied (scale, rotation).
  ///
  /// Performance: expensive.
  pub fn draw_image(&mut self, image: &Image, pos: Pos) -> Result<&mut Self> {
    let ok = (_GAPI.gDrawImage)(self.0, image.0, pos.0, pos.1, None, None, None, None, None, None, None);
    ok_or!(self, ok)
  }

  /// Draw a part of the image onto the graphics surface.
  ///
  /// With the current transformation applied (scale, rotation).
  ///
  /// Performance: expensive.
  pub fn draw_image_part(&mut self, image: &Image, dst_pos: Pos, dst_size: Size, src_pos: POINT, src_size: SIZE) -> Result<&mut Self> {
    let ix = src_pos.x as UINT;
    let iy = src_pos.y as UINT;
    let iw = src_size.cx as UINT;
    let ih = src_size.cy as UINT;
    let ok = (_GAPI.gDrawImage)(
      self.0,
      image.0,
      dst_pos.0,
      dst_pos.1,
      Some(&dst_size.0),
      Some(&dst_size.1),
      Some(&ix),
      Some(&iy),
      Some(&iw),
      Some(&ih),
      None,
    );
    ok_or!(self, ok)
  }

  /// Blend the image with the graphics surface.
  ///
  /// No affine transformations.
  ///
  /// Performance: less expensive.
  pub fn blend_image(&mut self, image: &Image, dst_pos: Pos, opacity: f32) -> Result<&mut Self> {
    let ok = (_GAPI.gDrawImage)(
      self.0,
      image.0,
      dst_pos.0,
      dst_pos.1,
      None,
      None,
      None,
      None,
      None,
      None,
      Some(&opacity),
    );
    ok_or!(self, ok)
  }

  /// Blend a part of the image with the graphics surface.
  ///
  /// No affine transformations.
  ///
  /// Performance: less expensive.
  pub fn blend_image_part(&mut self, image: &Image, dst_pos: Pos, opacity: f32, src_pos: POINT, src_size: SIZE) -> Result<&mut Self> {
    let ix = src_pos.x as UINT;
    let iy = src_pos.y as UINT;
    let iw = src_size.cx as UINT;
    let ih = src_size.cy as UINT;
    let ok = (_GAPI.gDrawImage)(
      self.0,
      image.0,
      dst_pos.0,
      dst_pos.1,
      None,
      None,
      Some(&ix),
      Some(&iy),
      Some(&iw),
      Some(&ih),
      Some(&opacity),
    );
    ok_or!(self, ok)
  }
}
