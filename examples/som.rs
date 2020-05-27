// there are logs in console window
// #![windows_subsystem="windows"]
extern crate sciter;

use sciter::{HELEMENT, Value, types::{BOOL, VALUE}};

#[derive(Default)]
pub struct Object {
	age: i32,
	name: String,
}

impl Object {
	pub fn print(&self) -> String {
		format!("Name: {}, Age: {}", self.name, self.age)
	}
}

// SOM Passport of the asset.
// TODO: should be auto-generated.
impl sciter::om::Passport for Object {
	fn get_passport(&self) -> &'static sciter::om::som_passport_t {
		use sciter::om::*;

		extern "C" fn on_print(thing: *mut som_asset_t, _argc: u32, _argv: *const VALUE, p_result: &mut VALUE) -> BOOL
		{
			let me = IAsset::<Object>::from_raw(&thing);
			let r = me.print();
			let r: sciter::Value = r.into();
			r.pack_to(p_result);
			return true as BOOL;
		}

		extern "C" fn on_get_age(thing: *mut som_asset_t, p_value: &mut VALUE) -> BOOL
		{
			let me = IAsset::<Object>::from_raw(&thing);
			let r = sciter::Value::from(&me.age);
			r.pack_to(p_value);
			return true as BOOL;
		}
		extern "C" fn on_set_age(thing: *mut som_asset_t, p_value: &VALUE) -> BOOL
		{
			let me = IAsset::<Object>::from_raw(&thing);
			use sciter::FromValue;
			let v = sciter::Value::from(p_value);
			if let Some(v) = FromValue::from_value(&v) {
				me.age = v;
				true as BOOL
			} else {
				false as BOOL
			}
		}

		extern "C" fn on_get_name(thing: *mut som_asset_t, p_value: &mut VALUE) -> BOOL
		{
			let me = IAsset::<Object>::from_raw(&thing);
			let r = sciter::Value::from(&me.name);
			r.pack_to(p_value);
			return true as BOOL;
		}
		extern "C" fn on_set_name(thing: *mut som_asset_t, p_value: &VALUE) -> BOOL
		{
			let me = IAsset::<Object>::from_raw(&thing);
			use sciter::FromValue;
			let v = sciter::Value::from(p_value);
			if let Some(v) = FromValue::from_value(&v) {
				me.name = v;
				true as BOOL
			} else {
				false as BOOL
			}
		}

		let mut method = Box::new(som_method_def_t::default());
		method.name = atom("print");
		method.func = Some(on_print);
		method.params = 0;

		type ObjectProps = [som_property_def_t; 2];

		let mut props = Box::new(ObjectProps::default());
		let mut prop = &mut props[0];
		prop.name = atom("age");
		prop.getter = Some(on_get_age);
		prop.setter = Some(on_set_age);

		let mut prop = &mut props[1];
		prop.name = atom("name");
		prop.getter = Some(on_get_name);
		prop.setter = Some(on_set_name);

		let mut pst = Box::new(som_passport_t::default());
		pst.name = atom("TestGlobal");

		pst.n_methods = 1;
		pst.methods = Box::into_raw(method);

		pst.n_properties = 2;
		pst.properties = Box::into_raw(props) as *const _;

		Box::leak(pst)
	}
}


#[derive(Default, Debug)]
struct Handler {
	#[allow(dead_code)]
	prop: i32,
}

impl sciter::EventHandler for Handler {
	fn attached(&mut self, _root: HELEMENT) {
		println!("attached");
	}
	fn detached(&mut self, _root: HELEMENT) {
		println!("detached");
	}
	fn document_complete(&mut self, _root: HELEMENT, _target: HELEMENT) {
		println!("loaded");
	}

	// SOM Passport of the event handler.
	// TODO: should be auto-generated.
	fn get_passport(&mut self) -> Option<&'static sciter::om::som_passport_t> {
		use sciter::om::*;

		extern "C" fn int_getter(thing: *mut som_asset_t, p_value: &mut VALUE) -> BOOL
		{
			let r = Value::from(17);
			r.pack_to(p_value);
			println!("int_getter({:?}) -> {:?}", thing, r);

			return true as BOOL;
		}
		extern "C" fn int_setter(thing: *mut som_asset_t, p_value: &VALUE) -> BOOL
		{
			let v = Value::from(p_value);
			println!("int_setter({:?}) <- {:?}", thing, v);
			return true as BOOL;
		}

		extern "C" fn method(thing: *mut som_asset_t, argc: u32, argv: *const VALUE, p_result: &mut VALUE) -> BOOL
		{
			let args = unsafe { Value::unpack_from(argv, argc) };

			let sargs = args.iter().map(|v| v.to_string()).collect::<Vec<_>>();
			println!("int_method({:?}) <- {}", thing, sargs.join(", "));

			let r = Value::from("success");
			r.pack_to(p_result);
			return true as BOOL;
		}

		let mut prop = Box::new(som_property_def_t::default());
		prop.name = atom("prop_int");
		prop.getter = Some(int_getter);
		prop.setter = Some(int_setter);

		let mut func = Box::new(som_method_def_t::default());
		func.name = atom("method");
		func.params = 1;
		func.func = Some(method);

		let mut pst = Box::new(som_passport_t::default());
		pst.name = atom("Handler");
		pst.properties = Box::into_raw(prop);
		pst.n_properties = 1;
		pst.methods = Box::into_raw(func);
		pst.n_methods = 1;


		return Some(Box::leak(pst));
	}
}

// #[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn test() {
	// use sciter::Value;

	let i = 17_i32;
	let s = String::from("17");

	let r = Value::from(&i);
	let r = Value::from(&s);

	let r = Value::from(i);
	let r = Value::from(s);
}

fn main() {
	sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();

	let mut frame = sciter::Window::new();

	let object = Object::default();
	let object = sciter::om::IAsset::new(object);
	sciter::om::set_global(object);

	let handler = Handler { prop: 17, };
	frame.event_handler(handler);

	let html = include_bytes!("som.htm");
	frame.load_html(html, Some("example://som.htm"));
	frame.run_app();
}
