#![allow(unused_variables)]

extern crate sciter;

use sciter::graphics::*;

const OK: Result<()> = Ok(());

macro_rules! assert_ok {
  ($left:expr, $right:expr) => {
    assert_eq!($left, $right.map(|_| ()));
  };
}

fn get() -> Image {
  Image::new((100, 100), true).expect("Can't create a `100x100` image")
}

#[test]
fn image_new() {
  let ok = Image::new((100, 100), false);
  assert_ok!(OK, ok);

  let ok = Image::new((100, 100), true);
  assert_ok!(OK, ok);
}

#[test]
fn image_dimensions() {
  let size = get().dimensions().unwrap();
  assert_eq!((100, 100), size);
}

#[test]
fn image_save() {
  let ok = get().save(SaveImageEncoding::Raw);
  assert!(ok.is_ok());
  assert_eq!(ok.unwrap().len(), 100 * 100 * 4);

  fn verify(image: &Image, format: SaveImageEncoding) -> Result<()> {
    image.save(format).map(|_| ())
  }

  let image = get();
  assert_ok!(OK, verify(&image, SaveImageEncoding::Png));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Jpeg(10)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Jpeg(100)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Webp(0)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Webp(10)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Webp(100)));
}

#[test]
fn image_load() {
  fn verify(image: &Image, format: SaveImageEncoding) -> Result<()> {
    let ok = image.save(format).and_then(|saved| Image::load(&saved));
    ok.map(|_| ())
  }

  let image = get();
  assert_ok!(OK, verify(&image, SaveImageEncoding::Png));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Jpeg(10)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Jpeg(100)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Webp(0)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Webp(10)));
  assert_ok!(OK, verify(&image, SaveImageEncoding::Webp(100)));

  let r = image.save(SaveImageEncoding::Raw).and_then(|saved| {
    let size = image.dimensions().unwrap();
    Image::with_data(size, true, &saved)
  });
  assert_ok!(OK, r);
}

#[test]
fn load_formats() {
	// A minimal BMP from https://en.wikipedia.org/wiki/BMP_file_format#Example_1
	let data = [
		0x42, 0x4D,
		0x46, 0x00, 0x00, 0x00,
		0x00, 0x00,
		0x00, 0x00,
		0x36, 0x00, 0x00, 0x00,
		0x28, 0x00, 0x00, 0x00,
		0x02, 0x00, 0x00, 0x00,
		0x02, 0x00, 0x00, 0x00,
		0x01, 0x00,
		0x18, 0x00,
		0x00, 0x00, 0x00, 0x00,
		0x10, 0x00, 0x00, 0x00,
		0x13, 0x0B, 0x00, 0x00,
		0x13, 0x0B, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0xFF,
		0xFF, 0xFF, 0xFF,
		0x00, 0x00,
		0xFF, 0x00, 0x00,
		0x00, 0xFF, 0x00,
		0x00, 0x00,
	];
	assert_eq!(data.len(), 0x44+2);
	let image = Image::load(&data).expect("unable to load BMP");
	let size = image.dimensions().unwrap();
	assert_eq!(size, (2,2) );


	// The following images were taken from https://github.com/mathiasbynens/small.

	fn verify(data: &[u8]) -> Result<()> {
		Image::load(data).map(|_| ())
	}

	let bmp = [
	  0x42,0x4d,0x1e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
  	0x1a,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x01,0x00,
  	0x01,0x00,0x01,0x00,0x18,0x00,0x00,0x00,0xff,0x00
	];

	let gif = [
	  0x47,0x49,0x46,0x38,0x39,0x61,0x01,0x00,0x01,0x00,
	  0x80,0x00,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0x21,
	  0xf9,0x04,0x01,0x00,0x00,0x00,0x00,0x2c,0x00,0x00,
	  0x00,0x00,0x01,0x00,0x01,0x00,0x00,0x02,0x01,0x44,
	  0x00,0x3b
	];

	let ico = [
	  0x00,0x00,0x01,0x00,0x01,0x00,0x01,0x01,0x00,0x00,
	  0x01,0x00,0x18,0x00,0x30,0x00,0x00,0x00,0x16,0x00,
	  0x00,0x00,0x28,0x00,0x00,0x00,0x01,0x00,0x00,0x00,
	  0x02,0x00,0x00,0x00,0x01,0x00,0x18,0x00,0x00,0x00,
	  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	  0x00,0x00,0x00,0x00,0xff,0x00,0x00,0x00,0x00,0x00
	];

	let jpeg = [ // a 121-byte one from d75477e
	  0xff,0xd8,0xff,0xdb,0x00,0x43,0x00,0x01,0x01,0x01,
	  0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,
	  0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,
	  0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,
	  0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,
	  0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,
	  0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,0x01,
	  0x01,0xff,0xc2,0x00,0x0b,0x08,0x00,0x01,0x00,0x01,
	  0x01,0x01,0x11,0x00,0xff,0xc4,0x00,0x14,0x00,0x01,
	  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
	  0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0xff,0xda,
	  0x00,0x08,0x01,0x01,0x00,0x00,0x00,0x01,0x3f,0xff,
	  0xd9
	];

	let png = [
	  0x89,0x50,0x4e,0x47,0x0d,0x0a,0x1a,0x0a,0x00,0x00,
	  0x00,0x0d,0x49,0x48,0x44,0x52,0x00,0x00,0x00,0x01,
	  0x00,0x00,0x00,0x01,0x08,0x06,0x00,0x00,0x00,0x1f,
	  0x15,0xc4,0x89,0x00,0x00,0x00,0x0a,0x49,0x44,0x41,
	  0x54,0x78,0x9c,0x63,0x00,0x01,0x00,0x00,0x05,0x00,
	  0x01,0x0d,0x0a,0x2d,0xb4,0x00,0x00,0x00,0x00,0x49,
	  0x45,0x4e,0x44,0xae,0x42,0x60,0x82
	];

	let webp = [
	  0x52,0x49,0x46,0x46,0x12,0x00,0x00,0x00,0x57,0x45,
	  0x42,0x50,0x56,0x50,0x38,0x4c,0x06,0x00,0x00,0x00,
	  0x2f,0x41,0x6c,0x6f,0x00,0x6b
	];

  println!();
  println!("verify(&bmp): {}", verify(&bmp).is_ok());
  println!("verify(&gif): {}", verify(&gif).is_ok());
  println!("verify(&ico): {}", verify(&ico).is_ok());
  println!("verify(&jpeg) {}", verify(&jpeg).is_ok());
  println!("verify(&png): {}", verify(&png).is_ok());
  println!("verify(&webp) {}", verify(&webp).is_ok());

  assert_ok!(OK, verify(&bmp));
  assert_ok!(OK, verify(&gif));
  assert_ok!(OK, verify(&ico));
  assert_ok!(OK, verify(&jpeg));
  assert_ok!(OK, verify(&png));
  assert_ok!(OK, verify(&webp));
}

#[test]
fn image_clear() {
  assert_eq!(OK, get().clear());
  assert_eq!(OK, get().clear_with(rgb(255, 255, 255)));
}

#[test]
fn make_color() {
  // ABGR
  assert_eq!(0xFF000000, rgb(0, 0, 0));
  assert_eq!(0x00000000, rgba((0, 0, 0), 0));

  assert_eq!(0xFF112233, rgb(0x33, 0x22, 0x11));
}

#[test]
fn paint() {
  let image = Image::new((100, 100), false).unwrap();
  let ok = image.paint(|gfx, size| {
    gfx.rectangle((5.0, 5.0), (size.0 - 5.0, size.1 - 5.0))?;
    Ok(())
  });
  assert_ok!(OK, ok);
}
