//! Minimalistic Sciter sample.

extern crate sciter;

fn main() {
	// Step 1: Include the 'minimal.html' file to a byte array.
	// Hint: Take a look into 'minimal.html' which contains tiscript
	// rust Macro from std (std::include_bytes)
	let html = include_bytes!("minimal.htm");

	// Step 2: Create a new main sciter window of type sciter::Window
	// Hint: The sciter Window Wrapper (/src/window.rs) contains more interesting functions to open or attach a scriter Window
	let mut frame = sciter::Window::new();

	// Step 3: Load HTML byte array from memory to sciter::Window.
	// Hint: second parameter is a optional uri (here: None)
	frame.load_html(html, None);

	// Step 4: Show window and run the main app message loop until window been closed
	frame.run_app(true);
}

/*	Alternative explicit Declaration
fn main() {
	let html:&[u8] = include_bytes!("minimal.htm");
	let mut frame:sciter::Window = sciter::Window::new();
	frame.load_html(html, None);
	frame.run_app(true);
}
*/
