//! Sciter extension library example.
//!
//! See the [blog post](https://sciter.com/include-library-name-native-extensions/).

#[macro_use]
extern crate sciter;

use sciter::types::{BOOL, VALUE};
use sciter::Value;

pub fn add(args: &[Value]) -> Value {
	let sum: i32 = args
		.iter()
		.map(|v| v.to_int())
		.filter(|v| v.is_some())
		.map(|v| v.unwrap())
		.sum();
	Value::from(sum)
}

pub fn sub(args: &[Value]) -> std::result::Result<Value, String> {
	if let [a, b] = args {
		let a = a.to_int().ok_or("`a` is not an int")?;
		let b = b.to_int().ok_or("`b` is not an int")?;
		Ok(Value::from(a - b))
	} else {
		Err(format!("sub(a,b) expects 2 parameters, given {} instead.", args.len()))
	}
}

#[no_mangle]
pub extern "system" fn SciterLibraryInit(api: &'static sciter::ISciterAPI, exported: &mut VALUE) -> BOOL
{
	sciter::set_api(api);

	let _a = Value::from(add);
	let _b = Value::from(sub);

	let ext_api = vmap! {
		"add" => add,
		"sub" => sub,
	};

	ext_api.pack_to(exported);

	true as BOOL
}
