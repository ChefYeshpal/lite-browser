#[no_mangle]
pub extern "C" fn rust_greeting() -> *const u8 {
    let s = "Hello from Rust!";
    s.as_ptr()  // Note: This simple example returns pointer; proper JNI handling is needed for strings.
}
