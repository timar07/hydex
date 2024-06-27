use std::ffi::{CStr, CString};
use libc::c_char;

use crate::Markdown;

#[no_mangle]
pub extern "C" fn md_to_html_generate(s: *const c_char) -> *mut c_char {
    let cstr = unsafe { CStr::from_ptr(s).to_str().unwrap() };
    let compiled = Markdown::compile(cstr.to_string());
    CString::new(compiled).unwrap().into_raw()
}
