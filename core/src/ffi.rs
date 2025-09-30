use std::{
    ffi::{c_double, c_ulonglong},
    mem::forget,
    slice::from_raw_parts,
};

#[repr(C)]
pub struct MLMatrixFFI {
    pub data: *mut c_double,
    pub len: c_ulonglong,
}

#[unsafe(no_mangle)]
extern "C" fn process_matrix_ffi(input: *const c_double, len: c_ulonglong) -> *mut MLMatrixFFI {
    let input = unsafe { from_raw_parts(input, len as usize).to_vec() };

    let mut output = crate::process_matrix(&input);

    // this step might look weird, but it *absolutely* necessary to avoid
    // dangling pointer
    let ptr = output.as_mut_ptr();
    let len = output.len();
    forget(output);

    Box::into_raw(Box::new(MLMatrixFFI { data: ptr, len: len as c_ulonglong }))
}

#[unsafe(no_mangle)]
extern "C" fn free_matrix_ffi(ptr: *mut MLMatrixFFI) {
    unsafe {
        _ = Box::from_raw(ptr);
    }
}
