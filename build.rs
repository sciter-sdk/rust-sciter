#[cfg(all(windows, not(feature = "dynamic")))]
fn main() {
	use std::{env, path::PathBuf};
	if let Ok(path) = env::var("SCITER_STATIC_LIBRARY") {
		let lib_dir = PathBuf::from(path);
		println!("cargo:rustc-link-search=native={}", lib_dir.display());
		if cfg!(feature = "nightly") {
            // -bundle allow msvc linker link the library with ltcg
            // this is a nightly feature now: https://github.com/rust-lang/rust/issues/81490
			println!("cargo:rustc-link-lib=static:-bundle={}", "sciter.static");
			if cfg!(feature = "skia") {
				println!("cargo:rustc-link-lib=static:-bundle={}", "atls");
			}
		} else {
			println!("cargo:rustc-link-lib=static={}", "sciter.static");
			if cfg!(feature = "skia") {
				println!("cargo:rustc-link-lib=static={}", "atls");
			}
		}
		println!("cargo:rustc-link-lib={}", "Comdlg32");
		println!("cargo:rustc-link-lib={}", "windowscodecs");
		println!("cargo:rustc-link-lib={}", "Wininet");
	} else {
		println!("cargo:warning=Set SCITER_STATIC_LIBRARY to link static library");
	}
}

#[cfg(not(all(windows, not(feature = "dynamic"))))]
fn main() {}
