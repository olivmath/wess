extern "C" {
    fn randomi32() -> i32;
}

#[no_mangle]
pub extern "C" fn random() -> i32 {
    unsafe { randomi32() }
}

#[no_mangle]
pub extern "C" fn random_numbers(n: i32) -> *mut i32 {
    let mut randoms: Vec<i32> = Vec::new();

    for _ in 0..n {
        randoms.push(unsafe { randomi32() });
    }

    let ptr = randoms.as_mut_ptr();

    std::mem::forget(randoms);

    ptr
}

#[no_mangle]
pub extern "C" fn random_numbers_len() -> i32 {
    random_numbers(0) as i32
}

#[no_mangle]
pub extern "C" fn free_memory(ptr: *mut i32) {
    if !ptr.is_null() {
        unsafe {
            Vec::from_raw_parts(ptr, 0, 0);
        }
    }
}
