use std::ffi::{CStr, CString};

use crate::Markdown;

#[no_mangle]
pub extern "C" fn md_to_html_generate(s: *const i8) -> *mut i8 {
    let cstr = unsafe { CStr::from_ptr(s).to_str().unwrap() };
    let compiled = Markdown::compile(cstr.to_string());
    CString::new(compiled).unwrap().into_raw()
}
