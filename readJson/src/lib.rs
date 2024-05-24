// const length: *mut usize = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn gen_array(a: i32, b: i32) -> *const i32 {
    let v: Vec<i32> = (a..b).rev().collect();
    let ptr = v.as_ptr();
    std::mem::forget(v); // 避免 Rust 将内存释放
                         // unsafe { *length = len };
    ptr
}

// #[no_mangle]
// pub extern "C" fn gen_array_len() -> usize {
//     unsafe { *length }
// }

#[no_mangle]
pub extern "C" fn diff(a: i32, b: i32) -> i32 {
    a - b
}
