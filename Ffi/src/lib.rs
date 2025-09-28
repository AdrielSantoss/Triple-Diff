use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use myers::myers_diff;
use utils::DiffOp;

#[unsafe(no_mangle)]
pub extern "C" fn myers_diff_c(
    content_a: *const *const c_char,
    len_a: usize,
    content_b: *const *const c_char,
    len_b: usize,
    out_len: *mut usize,
) -> *mut *mut c_char {
    assert!(!content_a.is_null());
    assert!(!content_b.is_null());
    assert!(!out_len.is_null());

    let slice_a = unsafe { std::slice::from_raw_parts(content_a, len_a) };
    let slice_b = unsafe { std::slice::from_raw_parts(content_b, len_b) };

    let rust_lines_a: Vec<&str> = slice_a
        .iter()
        .map(|&s| unsafe { CStr::from_ptr(s).to_str().unwrap() })
        .collect();

    let rust_lines_b: Vec<&str> = slice_b
        .iter()
        .map(|&s| unsafe { CStr::from_ptr(s).to_str().unwrap() })
        .collect();

    let edits = myers_diff(&rust_lines_a, &rust_lines_b);

    let c_ptrs: Vec<*mut c_char> = edits
        .iter()
        .map(|d| {
            let prefixed = match d {
                DiffOp::Match(s) => format!(" {}", s),
                DiffOp::Insert(s) => format!("+{}", s),
                DiffOp::Delete(s) => format!("-{}", s),
            };
            CString::new(prefixed).unwrap().into_raw()
        })
        .collect();

    let len = c_ptrs.len();
    unsafe { *out_len = len; }

    let boxed_ptr = c_ptrs.into_boxed_slice();
    Box::into_raw(boxed_ptr) as *mut *mut c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn free_diff(result: *mut *mut c_char, len: usize) {
    if result.is_null() {
        return;
    }
    unsafe {
        let slice = std::slice::from_raw_parts_mut(result, len);

        for ptr in slice.iter_mut() {
            if !ptr.is_null() {
                let _ = CString::from_raw(*ptr);
            }
        }

        let _ = Box::from_raw(slice);
    }
}
