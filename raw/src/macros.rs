#[macro_export]
macro_rules! to_cstr {
    ($s:expr) => {{
        let s = std::ffi::CString::new($s).unwrap();
        s.as_ptr()
    }};
}
