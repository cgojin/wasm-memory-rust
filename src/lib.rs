#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
    let mut output = Vec::with_capacity(len);
    let ptr = output.as_mut_ptr();

    std::mem::forget(output);

    return ptr;
}

/*
#[no_mangle]
pub fn alloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
        std::alloc::alloc(layout)
    }
}
*/

#[no_mangle]
pub fn dealloc(ptr: *mut u8, size: usize) {
    let input: Vec<u8>;
    unsafe {
        input = Vec::from_raw_parts(ptr, size, size);
    }

    std::mem::drop(input);
}

/*
#[no_mangle]
pub fn dealloc(ptr: *mut u8, size: usize) {
    let align = std::mem::align_of::<usize>();
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
        std::alloc::dealloc(ptr, layout);
    }
}
*/

#[no_mangle]
pub fn sum(ptr: *mut u8, len: usize) -> i32 {
    let input: Vec<u8>;
    unsafe {
        input = Vec::from_raw_parts(ptr, len, len);
    }

    input.iter().map(|n| *n as i32).sum()
}

#[no_mangle]
pub fn upper(ptr: *mut u8, len: usize) -> *mut u8 {
    let input: Vec<u8>;
    unsafe {
        input = Vec::from_raw_parts(ptr, len, len);
    }

    let mut output = input.to_ascii_uppercase();
    let ptr = output.as_mut_ptr();

    std::mem::forget(output);

    ptr
}
