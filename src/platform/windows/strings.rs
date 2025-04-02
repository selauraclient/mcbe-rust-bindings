use windows::core::{PCSTR, PCWSTR, PWSTR};

pub struct CWSTR(pub PCWSTR, Vec<u16>);

unsafe impl Send for CWSTR {}

unsafe impl Sync for CWSTR {}

impl CWSTR {
    pub fn new(value: &str) -> Self {
        let mut vector: Vec<u16> = value.encode_utf16().collect();
        vector.push(0);

        Self(PCWSTR(vector.as_ptr()), vector)
    }
}

pub struct WSTR(pub PWSTR, Vec<u16>);

unsafe impl Send for WSTR {}

unsafe impl Sync for WSTR {}

impl WSTR {
    pub fn new(value: &str) -> Self {
        let mut vector: Vec<u16> = value.encode_utf16().collect();
        vector.push(0);

        Self(PWSTR(vector.as_mut_ptr()), vector)
    }
}

pub struct CSTR(pub PCSTR, Vec<u8>);

unsafe impl Send for CSTR {}

unsafe impl Sync for CSTR {}

impl CSTR {
    pub fn new(value: &str) -> Self {
        let mut vector = value.as_bytes().to_vec();
        vector.push(0);

        Self(PCSTR(vector.as_ptr()), vector)
    }
}