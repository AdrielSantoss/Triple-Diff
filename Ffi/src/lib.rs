use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use myers::myers_diff as myers_diff_rs;
use patience::patience_diff as patience_diff_rs;
use utils::DiffOp;

enum Algorithm {
    Myers,
    Patience,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn myers_diff(
    content_a: *mut *mut c_char,
    len_a: i32,
    content_b: *mut *mut c_char,
    len_b: i32,
    out_len: *mut i32,
) -> *mut *mut c_char {
    unsafe { run_diff_ffi(content_a, len_a, content_b, len_b, Algorithm::Myers, out_len) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn patience_diff(
    content_a: *mut *mut c_char,
    len_a: i32,
    content_b: *mut *mut c_char,
    len_b: i32,
    out_len: *mut i32,
) -> *mut *mut c_char {
    unsafe { run_diff_ffi(content_a, len_a, content_b, len_b, Algorithm::Patience, out_len) }
}

unsafe fn run_diff_ffi(
    content_a: *mut *mut c_char,
    len_a: i32,
    content_b: *mut *mut c_char,
    len_b: i32,
    algo: Algorithm,
    out_len: *mut i32,
) -> *mut *mut c_char {
    let slice_a: Vec<&str> = (0..len_a)
        .map(|i| unsafe { CStr::from_ptr(*content_a.add(i as usize)).to_str().unwrap() })
        .collect();

    let slice_b: Vec<&str> = (0..len_b)
        .map(|i| unsafe { CStr::from_ptr(*content_b.add(i as usize)).to_str().unwrap() })
        .collect();

    let diff: Vec<DiffOp> = match algo {
        Algorithm::Myers => myers_diff_rs(&slice_a, &slice_b),
        Algorithm::Patience => patience_diff_rs(&slice_a, &slice_b),
    };

    let c_ptrs: Vec<*mut c_char> = diff
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

    if !out_len.is_null() {
        unsafe { *out_len = c_ptrs.len() as i32 };
    }

    let boxed = c_ptrs.into_boxed_slice();
    Box::into_raw(boxed) as *mut *mut c_char
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_diff(ptr: *mut *mut c_char, len: i32) {
    if ptr.is_null() {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, len as usize) };

    for &mut p in slice.iter_mut() {
        if !p.is_null() {
            drop(unsafe { CString::from_raw(p) });
        }
    }

    let _ = unsafe { Box::from_raw(slice) };
}
