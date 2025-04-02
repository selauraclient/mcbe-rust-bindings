use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, STATUS_PENDING},
        System::Threading::{GetExitCodeProcess, OpenProcess, PROCESS_ALL_ACCESS},
    },
    core::Result,
};

pub struct Process {
    pub id: u32,
    pub handle: HANDLE,
}

impl Process {
    pub fn new(value: u32) -> Result<Self> {
        unsafe {
            Ok(Self {
                id: value,
                handle: OpenProcess(PROCESS_ALL_ACCESS, false, value)?,
            })
        }
    }

    pub fn running(&self) -> bool {
        let mut code = 0u32;
        unsafe {
            GetExitCodeProcess(self.handle, &mut code).is_ok() && code as i32 == STATUS_PENDING.0
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe { _ = CloseHandle(self.handle) }
    }
}