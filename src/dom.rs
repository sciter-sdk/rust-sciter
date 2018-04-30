/*! DOM access methods via the [`dom::Element`](struct.Element.html).


## Introduction.

Let’s assume you have already integrated Sciter in your application and so you have Sciter window with loaded content.

From Sciter's point of view the loaded document is a tree of DOM elements (elements of Document Object Model).
Sciter builds this tree while loading/parsing of input HTML.
As a rule each tag in source HTML gets matching DOM element (there are exceptions, see below).

You can change text, attributes, state flags of DOM elements;
add new or remove existing DOM elemdoents.
You can also attach your own DOM event handlers to DOM elements to receive events and notifications.

Therefore your UI in Sciter is a collection of uniform DOM elements
that can be styled by CSS and manipulated by native or script code.


## Basic operations

To access the DOM tree we need to get reference of its root element
(the root element is the element representing the `<html>` tag in HTML source).

```rust,no_run
# use sciter::dom::Element;
# let hwnd = ::std::ptr::null_mut();
let root = Element::from_window(hwnd).unwrap();
assert_eq!(root.get_tag(), "html");
```

*TBD:* Other ways to access DOM tree.

By having a root element reference we are able to access any other element in the tree
using various access and search functions like `SciterGetNthChild`, `SciterSelectElements`, …
All of them are wrapped into methods of `dom::Element`.
Here is how you would get reference to first `<div>` element with class "sidebar" using CSS selectors:

```rust,no_run
# let root = sciter::dom::Element::from(::std::ptr::null_mut());
let sidebar = root.find_first("div.sidebar").unwrap();
```

The same in script:

```tiscript
var sidebar = self.select("div.sidebar"); // or
var sidebar = self.$(div.sidebar); // using a stringizer variant of select()
```

*TBD:* Other select methods.

## DOM element operations

You can change the **text** or HTML of a DOM element:

```rust,no_run
# let root = sciter::dom::Element::from(::std::ptr::null_mut());
if let Some(mut el) = root.find_first("#cancel").unwrap() {
	el.set_text("Abort!");
	el.set_html(br##"<img src="http://lorempixel.com/32/32/cats/" alt="some cat"/>"##, None);
}
```

The same but in script:

```tiscript
var el = ...;
el.text = "Hello world"; // text
el.html = "Hello <b>wrold</b>!"; // inner html
```

You can also get or set DOM **attributes** of any DOM element:

```rust,no_run
# let mut el = sciter::dom::Element::from(::std::ptr::null_mut());
let val = el.get_attribute("class").unwrap();
el.set_attribute("class", "new-class");
```

To **remove** an existing DOM element (to detach it from the DOM) you will do this:

```rust,no_run
# let mut el = sciter::dom::Element::from(::std::ptr::null_mut());
el.detach();
```

and when code will leave the scope where the `el` variable is defined, the DOM element will be destroyed.

Creation and population of DOM elements looks like this:

```rust,no_run
# use sciter::dom::Element;
# let mut el = sciter::dom::Element::from(::std::ptr::null_mut());
let p = Element::with_text("p", "Hello").unwrap(); // create <p> element
el.append(&p); // append it to existing element, or use insert() ...
```

And in script:

```tiscript
var p = new Element("p", "Hello");
el.append(p);
```

To change runtime state flags of a DOM element we do something like this:

```rust,ignore
# let mut el = sciter::dom::Element::from(::std::ptr::null_mut());
el.set_state(STATE_VISITED);
```

And in script:

```tiscript
el.state.visited = true;
```

(after such call the element will match `:visited` CSS selector)


## Getting and setting values of DOM elements.

By default a value of a DOM element is its text but some DOM elements may have
so called behaviors attached to them (see below).
`<input>`’s elements, for example, are plain DOM elements but each input type has its own behavior assigned to the element.
The behavior, among other things, is responsible for providing and setting the value of the element.

For example, value of an `input type=checkbox>` is boolean – _true_ or _false_,
and value of a `<form>` element is a collection (name/value map) of all named inputs on the form.

In native code values are represented by `sciter::Value` objects.
`sciter::Value` is a structure that can hold different types of values: numbers, strings, arrays, objects, etc
(see [documentation](https://sciter.com/docs/content/script/language/Types.htm)).

Here is how to set a numeric value of a DOM element in native code:

```rust,no_run
# use sciter::Value;
# let root = sciter::dom::Element::from(::std::ptr::null_mut());
if let Some(mut num) = root.find_first("input[type=number]").unwrap() {
	num.set_value( Value::from(12) );  // sciter::Value with T_INT type (i32 in Rust)
	num.set_value(12);  // equivalent but with implicit conversion
}
```

In script the same will look like:

```tiscript
if (var num = self.select("input[type=number]")) {
	num.value = 12;
}
```

.
*/

use ::{_API};
use capi::sctypes::*;
use value::Value;

pub use capi::scdom::{SCDOM_RESULT, HELEMENT, SET_ELEMENT_HTML, ELEMENT_AREAS};
use capi::scbehavior::{CLICK_REASON, BEHAVIOR_EVENTS, BEHAVIOR_EVENT_PARAMS};

pub use dom::event::EventHandler;
pub use dom::event::EventReason;

/// A specialized `Result` type for DOM operations.
pub type Result<T> = ::std::result::Result<T, SCDOM_RESULT>;


/// Initialize HELEMENT by nullptr.
macro_rules! HELEMENT {
	() => { ::std::ptr::null_mut() }
}


macro_rules! ok_or {
	($rv:expr, $ok:ident) => {
		if $ok == SCDOM_RESULT::OK {
			Ok($rv)
		} else {
			Err($ok)
		}
	};

	// for DOM access not_handled is ok
	// for calling function operation_failed is also ok
	($rv:expr, $ok:ident, $skip_not_handled:expr) => {
		if $ok == SCDOM_RESULT::OK || ($ok == $skip_not_handled) {
			Ok($rv)
		} else {
			Err($ok)
		}
	};
}


trait ElementVisitor {
	fn on_element(&mut self, el: Element) -> bool;
	fn result(&self) -> Vec<Element>;
}

#[derive(Default)]
struct FindFirstElement {
	all: Vec<Element>,
}

impl ElementVisitor for FindFirstElement {
	fn on_element(&mut self, el: Element) -> bool {
		self.all.push(el);
		return true;	// stop enumeration
	}
	fn result(&self) -> Vec<Element> {
		self.all.clone()
	}
}

#[derive(Default)]
struct FindAllElements {
	all: Vec<Element>,
}

impl ElementVisitor for FindAllElements {
	fn on_element(&mut self, el: Element) -> bool {
		self.all.push(el);
		return false;	// continue enumeration
	}
	fn result(&self) -> Vec<Element> {
		self.all.clone()
	}
}


/// DOM element wrapper. See the module-level documentation also.
#[derive(PartialEq)]
pub struct Element {
	he: HELEMENT,
}

/// `sciter::Element` can be transferred across thread boundaries.
unsafe impl Send for Element {}

/// It is safe to share `sciter::Element` between threads - underlaying API is thread-safe.
unsafe impl Sync for Element {}

impl Element {

	//\name Creation

	/// Construct Element object from `HELEMENT` handle.
	pub fn from(he: HELEMENT) -> Element {
		Element { he: Element::use_or(he) }
	}

	/// Create new element, it is disconnected initially from the DOM.
	pub fn create(tag: &str) -> Result<Element> {
		let mut e = Element { he: HELEMENT!() };
		let (tag,_) = s2u!(tag);
		let text = 0 as LPCWSTR;
		let ok = (_API.SciterCreateElement)(tag.as_ptr(), text, &mut e.he);
		ok_or!(e, ok)
	}

	/// Create new element as child of `parent`.
	pub fn with_parent(tag: &str, parent: &mut Element) -> Result<Element> {
		let mut e = Element { he: HELEMENT!() };
		let (tag,_) = s2u!(tag);
		let text = 0 as LPCWSTR;
		(_API.SciterCreateElement)(tag.as_ptr(), text, &mut e.he);
		let ok = parent.append(&e);
		ok.map(|_| e)
	}

	/// Create new element as child of `parent`. Deprecated.
	#[deprecated(since="0.5.0", note="please use `Element::with_parent()` instead.")]
	pub fn create_at(tag: &str, parent: &mut Element) -> Result<Element> {
		Element::with_parent(tag, parent)
	}

	/// Create new element with specified `text`, it is disconnected initially from the DOM.
	pub fn with_text(tag: &str, text: &str) -> Result<Element> {
		let mut e = Element { he: HELEMENT!() };
		let (tag,_) = s2u!(tag);
		let (text,_) = s2w!(text);
		let ok = (_API.SciterCreateElement)(tag.as_ptr(), text.as_ptr(), &mut e.he);
		ok_or!(e, ok)
	}

	/// Create new element with specified `type`, which is useful for controls and widgets (initially disconnected).
	pub fn with_type(tag: &str, el_type: &str) -> Result<Element> {
		let mut e = Element { he: HELEMENT!() };
		let (tag,_) = s2u!(tag);
		let text = 0 as LPCWSTR;
		let ok = (_API.SciterCreateElement)(tag.as_ptr(), text, &mut e.he);
		if ok == SCDOM_RESULT::OK {
			let r = e.set_attribute("type", el_type);
			r.map(|_| e)
		} else {
			Err(ok)
		}
	}
	/// Get root DOM element of the Sciter document.
	pub fn from_window(hwnd: HWINDOW) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetRootElement)(hwnd, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Get focus DOM element of the Sciter document.
	pub fn from_focus(hwnd: HWINDOW) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetFocusElement)(hwnd, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Get highlighted element.
	pub fn from_highlighted(hwnd: HWINDOW) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetHighlightedElement)(hwnd, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Find DOM element of the Sciter document by coordinates.
	pub fn from_point(hwnd: HWINDOW, pt: POINT) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterFindElement)(hwnd, pt, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Get element handle by its UID.
	pub fn from_uid(hwnd: HWINDOW, uid: u32) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetElementByUID)(hwnd, uid, &mut p);
		ok_or!(Element::from(p), ok)
	}

	fn use_or(he: HELEMENT) -> HELEMENT {
		let ok = (_API.Sciter_UseElement)(he);
		if ok == SCDOM_RESULT::OK {
			he
		} else {
			HELEMENT!()
		}
	}


	//\name Common methods

	/// Access element pointer.
	pub fn as_ptr(&self) -> HELEMENT {
		self.he
	}

	/// Get element UID - identifier suitable for storage.
	pub fn get_uid(&self) -> u32 {
		let mut n = 0;
		(_API.SciterGetElementUID)(self.he, &mut n);
		return n;
	}

	/// Return element tag as string (e.g. 'div', 'body').
	pub fn get_tag(&self) -> String {
		let mut s = String::new();
		(_API.SciterGetElementTypeCB)(self.he, store_astr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Get inner text of the element as string.
	pub fn get_text(&self) -> String {
		let mut s = String::new();
		(_API.SciterGetElementTextCB)(self.he, store_wstr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Set inner text of the element.
	pub fn set_text(&mut self, text: &str) -> Result<()> {
		let (s,n) = s2w!(text);
		let ok = (_API.SciterSetElementText)(self.he, s.as_ptr(), n);
		ok_or!((), ok)
	}

	/// Get html representation of the element as utf-8 bytes.
	pub fn get_html(&self, with_outer_html: bool) -> Vec<u8> {
		let mut s = Vec::new();
		(_API.SciterGetElementHtmlCB)(self.he, with_outer_html as BOOL, store_bstr, &mut s as *mut Vec<u8> as LPVOID);
		return s;
	}

	/// Set inner or outer html of the element.
	pub fn set_html(&mut self, html: &[u8], how: Option<SET_ELEMENT_HTML>) -> Result<()> {
		if html.is_empty() {
			return self.clear();
		}
		let ok = (_API.SciterSetElementHtml)(self.he, html.as_ptr(), html.len() as UINT, how.unwrap_or(SET_ELEMENT_HTML::SIH_REPLACE_CONTENT) as UINT);
		ok_or!((), ok)
	}

	/// Get value of the element.
	pub fn get_value(&self) -> Value {
		let mut rv = Value::new();
		(_API.SciterGetValue)(self.he, rv.as_ptr());
		return rv;
	}

	/// Set value of the element.
	pub fn set_value<T: Into<Value>>(&mut self, val: T) -> Result<()> {
		let ok = (_API.SciterSetValue)(self.he, val.into().as_cptr());
		ok_or!((), ok)
	}

	/// Get `HWINDOW` of containing window.
	pub fn get_hwnd(&self, for_root: bool) -> HWINDOW {
		let mut hwnd: HWINDOW = ::std::ptr::null_mut();
		(_API.SciterGetElementHwnd)(self.he, &mut hwnd as *mut HWINDOW, for_root as BOOL);
		return hwnd;
	}

	/// Get bounding rectangle of the element. See the [`ELEMENT_AREAS`](enum.ELEMENT_AREAS.html) enum for `kind` flags.
	pub fn get_location(&self, kind: u32) -> Result<RECT> {
		let mut rc = RECT::default();
		let ok = (_API.SciterGetElementLocation)(self.he, &mut rc as *mut _, kind as u32);
		ok_or!(rc, ok)
	}

	// TODO: request_data, request_html
	// TODO: send_request

	/// Sends sinking/bubbling event to the child/parent chain of element.
	pub fn send_event(&self, code: BEHAVIOR_EVENTS, reason: Option<CLICK_REASON>, source: Option<HELEMENT>) -> Result<bool> {
		let mut handled = false as BOOL;
		let r = reason.unwrap_or(CLICK_REASON::SYNTHESIZED);
		let s = source.unwrap_or(self.he);
		let ok = (_API.SciterSendEvent)(self.he, code as u32, s, r as u32, &mut handled);
		ok_or!(handled != 0, ok)
	}

	/// Post asynchronously a sinking/bubbling event to the child/parent chain of element.
	pub fn post_event(&self, code: BEHAVIOR_EVENTS, reason: Option<CLICK_REASON>, source: Option<HELEMENT>) -> Result<()> {
		let r = reason.unwrap_or(CLICK_REASON::SYNTHESIZED);
		let s = source.unwrap_or(self.he);
		let ok = (_API.SciterPostEvent)(self.he, code as u32, s, r as u32);
		ok_or!((), ok)
	}

	/// Send or posts event to the child/parent chain of element.
	pub fn fire_event(&self, code: BEHAVIOR_EVENTS, reason: Option<CLICK_REASON>, source: Option<HELEMENT>, post: bool, data: Option<Value>) -> Result<bool> {
		let mut handled = false as BOOL;
		let mut params = BEHAVIOR_EVENT_PARAMS {
			cmd: code as UINT,
			reason: reason.unwrap_or(CLICK_REASON::SYNTHESIZED) as UINT_PTR,
			he: source.unwrap_or(self.he),
			heTarget: self.he,
			data: Default::default(),
		};
		if data.is_some() {
			data.unwrap().pack_to(&mut params.data);
		}
		let ok = (_API.SciterFireEvent)(&params, post as BOOL, &mut handled);
		ok_or!(handled != 0, ok)
	}

	/// Send or posts event with specified params to the child/parent chain of element.
	pub fn fire_event_params(evt: &BEHAVIOR_EVENT_PARAMS, post: bool) -> Result<bool> {
		let mut handled = false as BOOL;
		let ok = (_API.SciterFireEvent)(evt, post as BOOL, &mut handled);
		ok_or!(handled != 0, ok)
	}


	/// Evaluate the given script in context of the element.
	pub fn eval_script(&self, script: &str) -> Result<Value> {
		let mut rv = Value::new();
		let (s,n) = s2w!(script);
		let ok = (_API.SciterEvalElementScript)(self.he, s.as_ptr(), n, rv.as_ptr());
		return ok_or!(rv, ok, SCDOM_RESULT::OPERATION_FAILED);
	}

	/// Call scripting function defined in the namespace of the element (a.k.a. global function).
	///
	/// You can use the [`make_args!(args...)`](../macro.make_args.html) macro which helps you
	/// to construct script arguments from Rust types.
	pub fn call_function(&self, name: &str, args: &[Value]) -> Result<Value> {
		let mut rv = Value::new();
		let (name,_) = s2u!(name);
		let argv = Value::pack_args(args);
		let ok = (_API.SciterCallScriptingFunction)(self.he, name.as_ptr(), argv.as_ptr(), argv.len() as UINT, rv.as_ptr());
		return ok_or!(rv, ok, SCDOM_RESULT::OPERATION_FAILED);
	}

	/// Call scripting method defined for the element.
	///
	/// You can use the [`make_args!(args...)`](../macro.make_args.html) macro which helps you
	/// to construct script arguments from Rust types.
	pub fn call_method(&self, name: &str, args: &[Value]) -> Result<Value> {
		let mut rv = Value::new();
		let (name,_) = s2u!(name);
		let argv = Value::pack_args(args);
		let ok = (_API.SciterCallScriptingMethod)(self.he, name.as_ptr(), argv.as_ptr(), argv.len() as UINT, rv.as_ptr());
		return ok_or!(rv, ok, SCDOM_RESULT::OPERATION_FAILED);
	}


	//\name Attributes
	/// Get number of the attributes.
	pub fn attribute_count(&self) -> usize {
		let mut n = 0u32;
		(_API.SciterGetAttributeCount)(self.he, &mut n);
		return n as usize;
	}

	/// Get attribute name by its index.
	pub fn attribute_name(&self, index: usize) -> String {
		let mut s = String::new();
		(_API.SciterGetNthAttributeNameCB)(self.he, index as UINT, store_astr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Get attribute value by its index.
	pub fn attribute(&self, index: usize) -> String {
		let mut s = String::new();
		(_API.SciterGetNthAttributeValueCB)(self.he, index as UINT, store_wstr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Get attribute value by its name.
	pub fn get_attribute(&self, name: &str) -> Option<String> {
		let mut s = String::new();
		let (name,_) = s2u!(name);
		let ok = (_API.SciterGetAttributeByNameCB)(self.he, name.as_ptr(), store_wstr, &mut s as *mut String as LPVOID);
		match ok {
			SCDOM_RESULT::OK => Some(s),
			// SCDOM_RESULT::OK_NOT_HANDLED => None,
			_ => None,
		}
	}

	/// Add or replace attribute.
	pub fn set_attribute(&mut self, name: &str, value: &str) -> Result<()> {
		let (name,_) = s2u!(name);
		let (value,_) = s2w!(value);
		let ok = (_API.SciterSetAttributeByName)(self.he, name.as_ptr(), value.as_ptr());
		ok_or!((), ok)
	}

	/// Remove attribute.
	pub fn remove_attribute(&mut self, name: &str) -> Result<()> {
		let (name,_) = s2u!(name);
		let value = ::std::ptr::null();
		let ok = (_API.SciterSetAttributeByName)(self.he, name.as_ptr(), value);
		ok_or!((), ok)
	}

	/// Toggle attribute.
	pub fn toggle_attribute(&mut self, name: &str, isset: bool, value: Option<&str>) -> Result<()> {
		if isset {
			self.set_attribute(name, value.unwrap())
		} else {
			self.remove_attribute(name)
		}
	}

	/// Remove all attributes from the element.
	pub fn clear_attributes(&mut self) -> Result<()> {
		let ok = (_API.SciterClearAttributes)(self.he);
		ok_or!((), ok)
	}


	//\name Style Attributes

	/// Get [style attribute](https://sciter.com/docs/content/sciter/Style.htm) of the element by its name.
	pub fn get_style_attribute(&self, name: &str) -> String {
		let mut s = String::new();
		let (name,_) = s2u!(name);
		(_API.SciterGetStyleAttributeCB)(self.he, name.as_ptr(), store_wstr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Set [style attribute](https://sciter.com/docs/content/sciter/Style.htm).
	pub fn set_style_attribute(&mut self, name: &str, value: &str) -> Result<()> {
		let (name,_) = s2u!(name);
		let (value,_) = s2w!(value);
		let ok = (_API.SciterSetStyleAttribute)(self.he, name.as_ptr(), value.as_ptr());
		ok_or!((), ok)
	}

	//\name State methods


	//\name DOM tree access

	/// Get index of this element in its parent collection.
	pub fn index(&self) -> usize {
		let mut n = 0u32;
		(_API.SciterGetElementIndex)(self.he, &mut n as *mut UINT);
		return n as usize;
	}

	/// Get root of the element.
	pub fn root(&self) -> Element {
		if let Some(dad) = self.parent() {
			dad.root()
		} else {
			self.clone()
		}
	}

	/// Get parent element.
	pub fn parent(&self) -> Option<Element> {
		let mut p = HELEMENT!();
		(_API.SciterGetParentElement)(self.he, &mut p);
		if p.is_null() {
			None
		} else {
			Some(Element::from(p))
		}
	}

	/// Get first sibling element.
	pub fn first_sibling(&self) -> Option<Element> {
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if count > 0 {
				return dad.child(0);
			}
		}
		None
	}

	/// Get last sibling element.
	pub fn last_sibling(&self) -> Option<Element> {
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if count > 0 {
				return dad.child(count - 1);
			}
		}
		None
	}

	/// Get next sibling element.
	pub fn next_sibling(&self) -> Option<Element> {
		let idx = self.index() + 1;
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if idx < count {
				return dad.child(idx);
			}
		}
		None
	}

	/// Get previous sibling element.
	pub fn prev_sibling(&self) -> Option<Element> {
		let idx = self.index();
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if idx > 0 && (idx - 1) < count {
				return dad.child(idx - 1);
			}
		}
		None
	}

	/// Get first child element.
	pub fn first_child(&self) -> Option<Element> {
		return self.child(0);
	}

	/// Get last child element.
	pub fn last_child(&self) -> Option<Element> {
		let count = self.len();
		if count > 0 {
			return self.child(count - 1);
		}
		None
	}

	/// Get element's child at specified index.
	pub fn get(&self, index: usize) -> Option<Element> {
		return self.child(index);
	}

	/// An iterator over the direct children of a DOM element.
	pub fn children(&self) -> Children {
		Children {
			base: self,
			index: 0,
			count: self.children_count(),
		}
	}

	/// Get element's child at specified index.
	pub fn child(&self, index: usize) -> Option<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetNthChild)(self.he, index as UINT, &mut p);
		match ok {
			SCDOM_RESULT::OK => Some(Element::from(p)),
			_ => None
		}
	}

	/// Get number of child elements.
	pub fn children_count(&self) -> usize {
		let mut n = 0u32;
		(_API.SciterGetChildrenCount)(self.he, &mut n as *mut UINT);
		return n as usize;
	}

	/// Get number of child elements.
	pub fn len(&self) -> usize {
		return self.children_count();
	}

	/// Returns `true` is `self` has zero elements.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Clear content of the element.
	pub fn clear(&mut self) -> Result<()> {
		let ok = (_API.SciterSetElementText)(self.he, ::std::ptr::null(), 0);
		ok_or!((), ok)
	}

	/// Create new element as copy of existing element.
	///
	/// The new element is a full (deep) copy of the element and is initially disconnected from the DOM.
	/// Note that `Element.clone()` does not clone DOM element, just increments its reference count.
	pub fn clone_element(&self) -> Element {
		let mut e = Element { he: HELEMENT!() };
		(_API.SciterCloneElement)(self.he, &mut e.he);
		return e;
	}

	/// Insert element at `index` position of this element.
	///
	/// Note that we cannot follow Rust semantic here
	/// because the newly created `Element` is unusable before it will be inserted at DOM.
	pub fn insert(&mut self, index: usize, child: &Element) -> Result<()> {
		let ok = (_API.SciterInsertElement)(child.he, self.he, index as UINT);
		ok_or!((), ok)
	}

	/// Append element as last child of this element.
	pub fn append(&mut self, child: &Element) -> Result<()> {
		self.insert(0x7FFF_FFFF, child)
	}

	/// Append element as last child of this element.
	#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
	pub fn push(&mut self, element: Element) {
		self.append(&element).expect("Could not append element.");
	}

	/// Remove the last child from this element and returns it, or `None` if this element is empty.
	pub fn pop(&mut self) -> Option<Element> {
		let count = self.len();
		if count > 0 {
			if let Some(mut child) = self.get(count - 1) {
				child.detach().expect("Could not detach element.");
				return Some(child);
			}
		}
		return None;
	}

	/// Take element out of its container (and DOM tree).
	pub fn detach(&mut self) -> Result<()> {
		let ok = (_API.SciterDetachElement)(self.he);
		ok_or!((), ok)
	}

	/// Take element out of its container (and DOM tree) and force destruction of all behaviors.
	pub fn destroy(&mut self) -> Result<()> {
		let mut p = HELEMENT!();
		::std::mem::swap(&mut self.he, &mut p);
		let ok = (_API.SciterDeleteElement)(p);
		ok_or!((), ok)
	}

	/// Swap element positions.
	pub fn swap(&mut self, other: &mut Element) -> Result<()> {
		let ok = (_API.SciterSwapElements)(self.he, other.he);
		ok_or!((), ok)
	}

	//\name Selectors

	/// Test this element against CSS selector(s).
	pub fn test(&self, selector: &str) -> bool {
		let mut p = HELEMENT!();
		let (s,_) = s2u!(selector);
		(_API.SciterSelectParent)(self.he, s.as_ptr(), 1, &mut p);
		return !p.is_null();
	}

	/// Call specified function for every element in a DOM that meets specified CSS selectors.
	fn select_elements<T: ElementVisitor>(&self, selector: &str, callback: T) -> Result<Vec<Element>> {
		use ::capi::schandler::NativeHandler;
		extern "system" fn inner<T: ElementVisitor>(he: HELEMENT, param: LPVOID) -> BOOL {
			let obj = NativeHandler::get_data::<T>(&param);
			let e = Element::from(he);
			let stop = obj.on_element(e);
			return stop as BOOL;
		}
		let (s,_) = s2u!(selector);
		let handler = NativeHandler::from(callback);
		let ok = (_API.SciterSelectElements)(self.he, s.as_ptr(), inner::<T>, handler.as_mut_ptr());
		if ok != SCDOM_RESULT::OK {
			return Err(ok);
		}
		let obj = handler.as_ref::<T>();
		return Ok(obj.result());
	}

	/// Will find first parent element starting from this satisfying given css selector(s).
	pub fn find_nearest_parent(&self, selector: &str) -> Result<Option<Element>> {
		let mut p = HELEMENT!();
		let (s,_) = s2u!(selector);
		let ok = (_API.SciterSelectParent)(self.he, s.as_ptr(), 0, &mut p);
		if ok != SCDOM_RESULT::OK {
			return Err(ok);
		}
		if p.is_null() { Ok(None) } else { Ok(Some(Element::from(p))) }
	}

	/// Will find first element starting from this satisfying given css selector(s).
	pub fn find_first(&self, selector: &str) -> Result<Option<Element>> {
		let cb = FindFirstElement::default();
		let all = self.select_elements(selector, cb);
		all.map(|mut x| { x.pop() })
	}

	/// Will find all elements starting from this satisfying given css selector(s).
	pub fn find_all(&self, selector: &str) -> Result<Option<Vec<Element>>> {
		let cb = FindAllElements::default();
		let all = self.select_elements(selector, cb);
		all.map(Some)
	}

	//\name Scroll methods:

	//\name Other methods:

	/// Apply changes and refresh element area in its window.
	pub fn update(&self, render_now: bool) -> Result<()> {
		let ok = (_API.SciterUpdateElement)(self.he, render_now as BOOL);
		ok_or!((), ok)
	}

	/// Refresh element area in its window.
	///
	/// If the element has drawing behavior attached it will receive [`on_draw`](event/trait.EventHandler.html#method.on_draw) call after that.
	pub fn refresh(&self) -> Result<()> {
		let rect = self.get_location(ELEMENT_AREAS::self_content())?;
		let ok = (_API.SciterRefreshElementArea)(self.he, rect);
		ok_or!((), ok)
	}

	/// Start Timer for the element.
	///
	/// Element will receive [`on_timer`](event/trait.EventHandler.html#method.on_timer) events.
	///
	/// Note that timer events are not bubbling, so you need attach handler to the target element directly.
	pub fn start_timer(&self, period_ms: u32, timer_id: u64) -> Result<()> {
		let ok = (_API.SciterSetTimer)(self.he, period_ms as UINT, timer_id as ::capi::sctypes::UINT_PTR);
		ok_or!((), ok)
	}

	/// Stop Timer for the element.
	pub fn stop_timer(&self, timer_id: u64) -> Result<()> {
		if !self.he.is_null() {
			let ok = (_API.SciterSetTimer)(self.he, 0 as UINT, timer_id as ::capi::sctypes::UINT_PTR);
			ok_or!((), ok)
		} else {
			Ok(())
		}
	}

	/// Attach the native event handler to this element.
	pub fn attach_handler<Handler: EventHandler>(&mut self, handler: Handler) -> Result<u64> {
		// make native handler
		let boxed = Box::new(handler);
		let ptr = Box::into_raw(boxed);	// dropped in `_event_handler_proc`
		let token = ptr as usize as u64;
		let ok = (_API.SciterAttachEventHandler)(self.he, ::eventhandler::_event_handler_proc::<Handler>, ptr as LPVOID);
		ok_or!(token, ok)
	}

	/// Detach your handler from the element. Handlers identified by `token` from `attach_handler()` result.
	pub fn detach_handler<Handler: EventHandler>(&mut self, token: u64) -> Result<()> {
		let ptr = token as usize as *mut Handler;
		let ok = (_API.SciterDetachEventHandler)(self.he, ::eventhandler::_event_handler_proc::<Handler>, ptr as LPVOID);
		ok_or!((), ok)
	}
}

/// Release element pointer.
impl Drop for Element {
	fn drop(&mut self) {
		(_API.Sciter_UnuseElement)(self.he);
		self.he = HELEMENT!();
	}
}

/// Increment reference count of the dom element.
impl Clone for Element {
	fn clone(&self) -> Self {
		Element::from(self.he)
	}
}

/// Human element representation.
impl ::std::fmt::Display for Element {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		if self.he.is_null() {
			return f.write_str("None");
		}
		// "tag#id.class|type(name)"
		// "tag#id.class"
		let (t,n,i,c) = (self.get_attribute("type"), self.get_attribute("name"), self.get_attribute("id"), self.get_attribute("class"));
		let tag = self.get_tag();
		try!(f.write_str(&tag));
		if i.is_some() {
			try!(write!(f, "#{}", i.unwrap()));
		}
		if c.is_some() {
			try!(write!(f, ".{}", c.unwrap()));
		}
		if t.is_some() {
			try!(write!(f, "|{}", t.unwrap()));
		}
		if n.is_some() {
			try!(write!(f, "({})", n.unwrap()));
		}
		return Ok(());
	}
}

/// Machine-like element visualization.
impl ::std::fmt::Debug for Element {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		// "tag#id.class(name):0xdfdfdf"
		write!(f, "{{{}:{:?}}}", self, self.he)
	}
}


/// An iterator over the direct children of a DOM element.
pub struct Children<'a> {
	base: &'a Element,
	index: usize,
	count: usize,
}

/// Allows `for child in el.children() {}` enumeration.
impl<'a> ::std::iter::Iterator for Children<'a> {
	type Item = Element;

	fn next(&mut self) -> Option<Element> {
		if self.index < self.count {
			let pos = self.index;
			self.index += 1;
			self.base.child(pos)
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let remain = self.count - self.index;
		(remain, Some(remain))
	}

	fn count(self) -> usize {
		self.count
	}
}

impl<'a> ::std::iter::DoubleEndedIterator for Children<'a> {
	fn next_back(&mut self) -> Option<Element> {
		if self.index == self.count || self.count == 0 {
			None
		} else {
			self.count -= 1;
			self.base.child(self.count)
		}
	}
}

/// Allows `for child in &el {}` enumeration.
impl<'a> ::std::iter::IntoIterator for &'a Element {
	type Item = Element;
	type IntoIter = Children<'a>;

	fn into_iter(self) -> Children<'a> {
		self.children()
	}
}


use ::utf;

/// Convert an incoming UTF-16 to `String`.
extern "system" fn store_wstr(szstr: LPCWSTR, str_length: UINT, param: LPVOID) {
	let s = utf::w2sn(szstr, str_length as usize);
	let out = param as *mut String;
	unsafe { *out = s };
}

/// Convert an incoming UTF-8 to `String`.
extern "system" fn store_astr(szstr: LPCSTR,  str_length: UINT, param: LPVOID) {
	let s = utf::u2sn(szstr, str_length as usize);
	let out = param as *mut String;
	unsafe { *out = s };
}

/// Convert an incoming html string (UTF-8 in fact) to `String`.
extern "system" fn store_bstr(szstr: LPCBYTE, str_length: UINT, param: LPVOID) {
	let s = unsafe { ::std::slice::from_raw_parts(szstr, str_length as usize) };
	let pout = param as *mut Vec<u8>;
	let out = unsafe {&mut *pout};
	out.extend_from_slice(s);
}

/* Not implemented yet or not used APIs:

SciterAttachHwndToElement

SciterCallBehaviorMethod
SciterCombineURL
SciterControlGetType
SciterGetElementIntrinsicHeight
SciterGetElementIntrinsicWidths
SciterGetElementLocation
SciterGetElementNamespace
SciterGetElementState
SciterGetElementType
SciterGetExpando
SciterGetObject
SciterGetScrollInfo
SciterHidePopup
SciterHttpRequest
SciterIsElementEnabled
SciterIsElementVisible
SciterReleaseCapture
SciterRequestElementData
SciterScrollToView
SciterSetCapture
SciterSetElementState
SciterSetHighlightedElement
SciterSetScrollPos
SciterShowPopup
SciterShowPopupAt
SciterSortElements
SciterTraverseUIEvent

SciterCreateCommentNode
SciterCreateTextNode
SciterNodeAddRef
SciterNodeCastFromElement
SciterNodeCastToElement
SciterNodeChildrenCount
SciterNodeFirstChild
SciterNodeGetText
SciterNodeInsert
SciterNodeLastChild
SciterNodeNextSibling
SciterNodeNthChild
SciterNodeParent
SciterNodePrevSibling
SciterNodeRelease
SciterNodeRemove
SciterNodeSetText
SciterNodeType

*/

pub mod event {
	//!
	//! Behaviors support (a.k.a windowless controls).
	//!
	/*!

# Behaviors and event handling.

Primary goal of User Interface (UI) as a subsystem is to present some information to the user
and generate some events according to user’s actions.
Your application handles UI events and acts accordingly executing its functions.

To be able to handle events in native code you will need to attach instance of `sciter::EventHandler`
to existing DOM element or to the window itself. In the `EventHandler` you will receive all events
dispatched to the element and its children as before children (in `PHASE_MASK::SINKING` phase)
as after them (`PHASE_MASK::BUBBLING` event phase), see [Events Propagation](https://sciter.com/developers/for-native-gui-programmers/#events-propagation).

`EventHandler` attached to the window will receive all DOM events no matter which element they are targeted to.

`EventHandler` contains various methods – receivers of events of various types.
You can override any of these methods in order to receive events you are interested in your implementation of `sciter::EventHandler` class.


To attach native event handler to DOM element or to the window you can do one of these:

* "Manually", to Sciter window: `sciter::Window.event_handler(your_event_handler)`
* "Manually", to arbitrary DOM element: `sciter::dom::Element.attach_handler(your_event_handler)`
* To group of DOM elements by declaration in CSS: `selector { behavior:your-behavior-name }`

You also can assign events handlers defined in script code:

* "Manually", individual events: if you have reference `el` of some element then to handle mouse events you can do this for example:

```tiscript
el.onMouse = function(evt) { ... }
```

* "Manually", by assigning behavior class to the [Element](https://sciter.com/docs/content/sciter/Element.htm):

```tiscript
class MyEventsHandler: Element { ... }  // your behavior class which inherits sciter's Element class
el.prototype = MyEventsHandler; // "sub-class" the element.
```

* By declaration in CSS to all elements satisfying some CSS selector:

```css
selector { prototype: MyEventsHandler; }
```

In this case `MyEventsHandler` class should be defined in one of script files loaded by your HTML.

See the **Behavior attributes** section of [Sciter CSS property map](https://sciter.com/docs/content/css/cssmap.html)
and [this blog article](http://www.terrainformatica.com/2014/07/sciter-declarative-behavior-assignment-by-css-prototype-and-aspect-properties/) which covers
Behaviors, Prototypes and Aspects of Sciter CSS behavior assignment.





# Script and native code interaction


In Sciter you may want to define native functions that can be called by script.
At the same time you may need to call script functions from native code.
Sciter supports such interaction providing set of simple API functions:

## Evaluating scripts and invoking script functions from native code

You can use one of these methods to call scripts from code of your application:

* To evaluate arbitrary script in context of current document loaded into the window:

```rust,no_run
# use sciter::dom::Element;
# use sciter::Value;
# let hwnd = ::std::ptr::null_mut();
let root = Element::from_window(hwnd).unwrap();
let result: Value = root.eval_script("... script ...").unwrap();
```

* To call global function defined in script using its full name (may include name of namespaces where it resides):

```ignore
# #[macro_use] extern crate sciter;
# use sciter::Value;
# let root = sciter::dom::Element::from(::std::ptr::null_mut());
let result: Value = root.call_function("namespace.name", &make_args!(1, "2", 3.0)).unwrap();
```
parameters – `&[Value]` slice.

* To call method (function) defined in script for particular DOM element:

```ignore
# #[macro_use] extern crate sciter;
# use sciter::Value;
# let root = sciter::dom::Element::from(::std::ptr::null_mut());
if let Some(el) = root.find_first("input").unwrap() {
	let result: Value = el.call_method("canUndo", &make_args!()).unwrap();
}
```


## Calling native code from script

If needed your application may expose some [native] functions to be called by script code.
Usually this is made by implementing your own `EventHandler` and overriding its `on_script_call` method.
If you will do this then you can invoke this callback from script as:

* "global" native functions: `var r = view.funcName( p0, p1, ... );` – calling `on_script_call` of `EventHandler` instance
**attached to the window**.
* As element’s methods: `var r = el.funcName( p0, p1, ... );` – calling `on_script_call` of `EventHandler` instance
(native behavior) attached to the **element**.

This way you can establish interaction between scipt and native code inside your application.

*/

	pub use capi::scbehavior::{CLICK_REASON, EVENT_GROUPS, EDIT_CHANGED_REASON, BEHAVIOR_EVENTS, PHASE_MASK};
	pub use capi::scbehavior::{DRAW_EVENTS};

	use capi::sctypes::*;
	use capi::scdom::HELEMENT;
	use capi::scgraphics::HGFX;
	use value::Value;

	/// Default subscription events
	///
	/// Default is `HANDLE_BEHAVIOR_EVENT | HANDLE_SCRIPTING_METHOD_CALL` which covers behavior events
	/// (like `document_complete` or `button_click`) and script calls to native window.
	pub fn default_events() -> EVENT_GROUPS {
		return EVENT_GROUPS::HANDLE_BEHAVIOR_EVENT | EVENT_GROUPS::HANDLE_SCRIPTING_METHOD_CALL;
	}

	/// UI action causing change.
	#[derive(Debug)]
	pub enum EventReason {
		/// General event source triggers (by mouse, key or synthesized).
		General(CLICK_REASON),
		/// Edit control change trigger.
		EditChanged(EDIT_CHANGED_REASON),
		/// `<video>` request for frame source binding (*unsupported yet*).
		VideoBind(LPVOID),
	}


	/// DOM event handler which can be attached to any DOM element.
	///
	/// In notifications:
	///
	/// * `root` means the DOM element to which we attached (`<html>` for `Window` event handler).
	/// * `target` contains reference to the notification target DOM element.
	/// * `source` element e.g. in `SELECTION_CHANGED` it is new selected `<option>`, in `MENU_ITEM_CLICK` it is menu item (`<li>`) element.
	///
	/// For example, if we attached to `<body>` element, we will receive `document_complete` with `target` set to `<html>`.
	///
	#[allow(unused_variables)]
	pub trait EventHandler {

		/// Return list of event groups this event_handler is subscribed to.
		///
		/// Default is `HANDLE_BEHAVIOR_EVENT | HANDLE_SCRIPTING_METHOD_CALL`.
		/// See also [`default_events()`](fn.default_events.html).
		fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
			return Some(default_events());
		}

		/// Called when handler was attached to element or window.
		/// `root` is NULL if attaching to window without loaded document.
		fn attached(&mut self, root: HELEMENT) {}

		/// Called when handler was detached from element or window.
		fn detached(&mut self, root: HELEMENT) {}

		/// Notification that document finishes its loading - all requests for external resources are finished
		fn document_complete(&mut self, root: HELEMENT, target: HELEMENT) {}

		/// The last notification before document removal from the DOM.
		fn document_close(&mut self, root: HELEMENT, target: HELEMENT) {}

		/// Script calls from CSSS! script and TIScript.
		fn on_script_call(&mut self, root: HELEMENT, name: &str, args: &[Value]) -> Option<Value> {
			return self.dispatch_script_call(root, name, args);
		}

		/// Autogenerated dispatcher for script calls.
		fn dispatch_script_call(&mut self, root: HELEMENT, name: &str, args: &[Value]) -> Option<Value> {
			return None;
		}

		/// Notification event from builtin behaviors.
		fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
			return false;
		}

		/// Timer event from attached element.
		fn on_timer(&mut self, root: HELEMENT, timer_id: u64) -> bool { return false; }

		/// Drawing request event.
		///
		/// It allows to intercept drawing events of an `Element` and to manually draw its content, background and foreground layers.
		fn on_draw(&mut self, root: HELEMENT, gfx: HGFX, area: &RECT, layer: DRAW_EVENTS) -> bool { return false; }

		/// Size changed event.
		fn on_size(&mut self, root: HELEMENT) {}

	}

}
