use std::ops::Deref;
use windows::{
    Win32::System::Com::{
        CLSCTX_INPROC_SERVER, COINIT_DISABLE_OLE1DDE, COINIT_MULTITHREADED, CoCreateInstance,
        CoGetContextToken, CoInitializeEx,
    },
    core::{GUID, IUnknown, Interface, Result},
};

pub struct COM<T: Interface> {
    value: T,
}

unsafe impl<T: Interface> Sync for COM<T> {}

unsafe impl<T: Interface> Send for COM<T> {}

impl<T: Interface> COM<T> {
    pub fn create(rclsid: *const GUID) -> Result<COM<T>> {
        unsafe {
            if CoGetContextToken().is_err() {
                _ = CoInitializeEx(None, COINIT_DISABLE_OLE1DDE | COINIT_MULTITHREADED);
            }

            CoCreateInstance::<Option<&IUnknown>, T>(rclsid, None, CLSCTX_INPROC_SERVER)
                .map(|value| Self { value })
        }
    }
}

impl<T: Interface> Deref for COM<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}