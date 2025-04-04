use windows::core::{PCSTR, PCWSTR, PWSTR};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct WSTR(pub PWSTR, Vec<u16>);

impl WSTR {
    pub fn new(value: &str) -> Self {
        let mut vector: Vec<u16> = value.encode_utf16().collect();
        vector.push(0);

        Self(PWSTR(vector.as_mut_ptr()), vector)
    }
}

#[allow(dead_code)]
pub struct CSTR(pub PCSTR, Vec<u8>);

impl CSTR {
    pub fn new(value: &str) -> Self {
        let mut vector = value.as_bytes().to_vec();
        vector.push(0);

        Self(PCSTR(vector.as_ptr()), vector)
    }
}