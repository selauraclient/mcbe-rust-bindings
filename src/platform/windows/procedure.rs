use super::{CSTR, CWSTR};
use windows::{
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
    core::{Error, Result},
};

pub struct Procedure(pub usize);

unsafe impl Sync for Procedure {}

unsafe impl Send for Procedure {}

impl Procedure {
    pub fn new(module: &str, procedure: &str) -> Result<Self> {
        unsafe {
            let hmodule = GetModuleHandleW(CWSTR::new(module).0)?;
            match GetProcAddress(hmodule, CSTR::new(procedure).0) {
                Some(value) => Ok(Self(value as usize)),
                None => Err(Error::empty()),
            }
        }
    }
}