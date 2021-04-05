//! Sciter value, native C interface.

#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]

use capi::sctypes::*;

/// A JSON value.
///
/// An opaque union that can hold different types of values: numbers, strings, arrays, objects, etc.
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct VALUE
{
	/// Value type.
	pub t: VALUE_TYPE,

	/// Value unit type.
	pub u: UINT,

	/// Value data.
	pub d: UINT64,
}

impl VALUE {
	pub(crate) const fn new() -> Self {
		Self {
			t: VALUE_TYPE::T_UNDEFINED,
			u: 0,
			d: 0,
		}
	}
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum VALUE_RESULT
{
  OK_TRUE = -1,
  OK = 0,
  BAD_PARAMETER = 1,
  INCOMPATIBLE_TYPE = 2,
}

impl std::error::Error for VALUE_RESULT {}

impl std::fmt::Display for VALUE_RESULT {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}


#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum VALUE_STRING_CVT_TYPE {
	SIMPLE = 0,
	JSON_LITERAL = 1,
	JSON_MAP = 2,
	XJSON_LITERAL = 3,
}


/// Type identifier of the value.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum VALUE_TYPE {
	/// Just undefined, the data is zero, the unit can be [`UT_NOTHING`](VALUE_UNIT_UNDEFINED::UT_NOTHING).
	T_UNDEFINED = 0,
	/// Explicit `null` type, the rest fields are zero.
	T_NULL = 1,
	/// Data is `1` or `0`; units are used but unknown.
	T_BOOL,
	/// Data is integer.
	T_INT,
	/// Data is float.
	T_FLOAT,
	/// Data is a Sciter internal string, unit is [`VALUE_UNIT_TYPE_STRING`].
	T_STRING,
	/// Data is `FILETIME` (64-bit value in 100ns since the unix epoch).
	T_DATE,
	/// Data is a 64-bit number; no unit is stored but `is_utc` boolean is used during creation.
	T_CURRENCY,
	/// Data is a 32-bit number, unit is [`VALUE_UNIT_TYPE_LENGTH`].
	T_LENGTH,
	/// Data is a Sciter internal array, unit is
	T_ARRAY,
	T_MAP,
	T_FUNCTION,
	T_BYTES,
	T_OBJECT,
	T_DOM_OBJECT,
	T_RESOURCE,
	T_RANGE,
	T_DURATION,
	T_ANGLE,
	T_COLOR,
	T_ENUM,
	T_ASSET,

	T_UNKNOWN,
}

impl Default for VALUE_TYPE {
    fn default() -> Self {
        Self::T_UNDEFINED
    }
}

/// `undefined` sub-state.
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum VALUE_UNIT_UNDEFINED
{
	/// 'nothing' a.k.a. 'void' value in script.
	UT_NOTHING = 1,
}

/// String sub-types.
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum VALUE_UNIT_TYPE_STRING
{
	STRING = 0,        // string
	ERROR  = 1,        // is an error string
	SECURE = 2,        // secure string ("wiped" on destroy)
	URL 	 = 3,				 // url(...)
	SELECTOR = 4,			 // selector(...)
	FILE = 0xfffe,     // file name
	SYMBOL = 0xffff,   // symbol in tiscript sense
}

/// Length sub-types.
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum VALUE_UNIT_TYPE_LENGTH
{
	EM = 1, //height of the element's font.
	EX = 2, //height of letter 'x'
	PR = 3, //%
	SP = 4, //%% "springs", a.k.a. flex units
	PX = 7, //pixels
	IN = 8, //inches (1 inch = 2.54 centimeters).
	CM = 9, //centimeters.
	MM = 10, //millimeters.
	PT = 11, //points (1 point = 1/72 inches).
	PC = 12, //picas (1 pica = 12 points).
	DIP = 13,
	URL   = 16,  // url in string
}

// Sciter or TIScript specific
#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum VALUE_UNIT_TYPE_OBJECT
{
	ARRAY  = 0,   // type T_OBJECT of type Array
	OBJECT = 1,   // type T_OBJECT of type Object
	CLASS  = 2,   // type T_OBJECT of type Class (class or namespace)
	NATIVE = 3,   // type T_OBJECT of native Type with data slot (LPVOID)
	FUNCTION = 4, // type T_OBJECT of type Function
	ERROR = 5,    // type T_OBJECT of type Error
}

pub type NATIVE_FUNCTOR_INVOKE = extern "C" fn (tag: LPVOID, argc: UINT, argv: *const VALUE, retval: * mut VALUE);
pub type NATIVE_FUNCTOR_RELEASE = extern "C" fn (tag: LPVOID);
