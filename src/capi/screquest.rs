//! Sciter's get resource request object - represents requests made by Element/View.request() functions.

#![allow(non_camel_case_types, non_snake_case)]

// use capi::sctypes::{LPVOID};

MAKE_HANDLE!(HREQUEST, _HREQUEST);

#[repr(C)]
#[allow(missing_docs)]
pub struct SciterRequestAPI
{
	RequestUse: extern "system" fn (rq: HREQUEST),
	RequestUnUse: extern "system" fn (rq: HREQUEST),
}
