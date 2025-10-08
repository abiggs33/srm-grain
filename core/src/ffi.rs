use std::{
    ffi::{c_double, c_ulonglong},
    mem::forget,
    slice::from_raw_parts,
    sync::{LazyLock, Mutex, MutexGuard},
};

pub static FPV1: LazyLock<Mutex<Vec<f64>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static FPV2: LazyLock<Mutex<Vec<f64>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static FPV3: LazyLock<Mutex<Vec<f64>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static FPV4: LazyLock<Mutex<Vec<f64>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn fpv1() -> MutexGuard<'static, Vec<f64>> {
    FPV1.try_lock().unwrap()
}

pub fn fpv2() -> MutexGuard<'static, Vec<f64>> {
    FPV2.try_lock().unwrap()
}

pub fn fpv3() -> MutexGuard<'static, Vec<f64>> {
    FPV3.try_lock().unwrap()
}

pub fn fpv4() -> MutexGuard<'static, Vec<f64>> {
    FPV4.try_lock().unwrap()
}

pub fn process_entry(input: &[f64]) -> Vec<f64> {
    // right now, does nothing. this is where your 'main' would be put
    _ = input;
    todo!()
}

#[repr(C)]
pub struct MLMatrixFFI {
    pub data: *mut c_double,
    pub len: c_ulonglong,
}

#[unsafe(no_mangle)]
extern "C" fn slot_matrix(input: *const c_double, len: c_ulonglong, slot: u8) {
    let input = unsafe { from_raw_parts(input, len as usize).to_vec() };

    match slot {
        | 1 => *fpv1() = input,
        | 2 => *fpv2() = input,
        | 3 => *fpv3() = input,
        | 4 => *fpv4() = input,
        | _ => panic!("only 4 static matrices"),
    }
}

#[unsafe(no_mangle)]
extern "C" fn fetch_matrix(slot: u8) -> *mut MLMatrixFFI {
    let mut output = match slot {
        | 1 => fpv1(),
        | 2 => fpv2(),
        | 3 => fpv3(),
        | 4 => fpv4(),
        | _ => panic!("only 4 static matrices"),
    };

    let ptr = output.as_mut_ptr();
    let len = output.len();
    forget(output);

    Box::into_raw(Box::new(MLMatrixFFI { data: ptr, len: len as c_ulonglong }))
}

#[unsafe(no_mangle)]
#[allow(unused_must_use)]
extern "C" fn free_matrix_ffi(ptr: *mut MLMatrixFFI) {
    unsafe {
        Box::from_raw(ptr);
    }
}

// basically combines everything above into one function as it is specific on
// entry-point. preferable to use above methods
#[unsafe(no_mangle)]
extern "C" fn process_matrix_ffi(input: *const c_double, len: c_ulonglong) -> *mut MLMatrixFFI {
    let input = unsafe { from_raw_parts(input, len as usize).to_vec() };

    let mut output = process_entry(&input);

    // this step might look weird, but it *absolutely* necessary to avoid
    // dangling pointer
    let ptr = output.as_mut_ptr();
    let len = output.len();
    forget(output);

    Box::into_raw(Box::new(MLMatrixFFI { data: ptr, len: len as c_ulonglong }))
}
