use wasm_memory::*;

fn main() {
    let input = vec![1u8, 2, 3];
    let result = sum_array(input);
    println!("{}", result);

    let input = String::from("convert to uppercase");
    let result = upper_string(input);
    println!("{}", result);
}

fn sum_array(input: Vec<u8>) -> i32 {
    let ptr = alloc(input.len());
    unsafe {
        std::ptr::copy(input.as_ptr(), ptr, input.len());
    }

    // ptr will be drop automatically when it goes out of scope, don't drop it manually!
    // dealloc(ptr, input.len());

    sum(ptr, input.len())
}

fn upper_string(input: String) -> String {
    let bytes = input.as_bytes();
    let ptr = alloc(bytes.len());
    unsafe {
        std::ptr::copy(bytes.as_ptr(), ptr, bytes.len());
    }

    let ptr = upper(ptr, bytes.len());
    let data: Vec<u8>;
    unsafe {
        data = Vec::from_raw_parts(ptr, bytes.len(), bytes.len());
    }

    // ptr will be drop automatically when it goes out of scope, don't drop it manually!
    // dealloc(ptr, bytes.len());

    String::from_utf8(data).unwrap()
}
