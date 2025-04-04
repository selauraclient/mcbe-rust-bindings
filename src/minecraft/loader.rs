
use std::{mem::transmute, os::raw::c_void, path::absolute, ptr::null_mut, sync::LazyLock};

use windows::{
    Win32::{
        Foundation::{CloseHandle, GENERIC_ALL},
        Security::{
            ACL,
            Authorization::{
                EXPLICIT_ACCESS_W, SE_FILE_OBJECT, SET_ACCESS, SetEntriesInAclW,
                SetNamedSecurityInfoW, TRUSTEE_IS_NAME, TRUSTEE_IS_WELL_KNOWN_GROUP, TRUSTEE_W,
            },
            DACL_SECURITY_INFORMATION, SUB_CONTAINERS_AND_OBJECTS_INHERIT,
        },
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            Memory::{
                MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx,
            },
            Threading::{CreateRemoteThread, INFINITE, WaitForSingleObject},
        },
    },
    core::{Error, Result},
};

use crate::platform::windows::{Acl, CWSTR, Process, WSTR, procedure::Procedure};

use super::Game;

static PROCEDURE: LazyLock<Procedure> =
    LazyLock::new(|| Procedure::new("Kernel32", "LoadLibraryW").unwrap());

static ACL: LazyLock<Acl> = LazyLock::new(|| unsafe {
    let name = WSTR::new("ALL APPLICATION PACKAGES");
    let mut acl: *mut ACL = null_mut();
    _ = SetEntriesInAclW(
        Some(&[EXPLICIT_ACCESS_W {
            grfAccessPermissions: GENERIC_ALL.0,
            grfAccessMode: SET_ACCESS,
            grfInheritance: SUB_CONTAINERS_AND_OBJECTS_INHERIT,
            Trustee: TRUSTEE_W {
                TrusteeForm: TRUSTEE_IS_NAME,
                TrusteeType: TRUSTEE_IS_WELL_KNOWN_GROUP,
                ptstrName: name.0,
                ..Default::default()
            },
            ..Default::default()
        }]),
        None,
        &mut acl,
    );
    Acl(acl)
});

pub fn load(process: &Process, value: &str) -> Result<()> {
    let file = absolute(value)?;
    if file.try_exists()? {
        if let Some(value) = file.to_str() {
            let path = CWSTR::new(value);
            unsafe {
                _ = SetNamedSecurityInfoW(
                    path.0,
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION,
                    None,
                    None,
                    Some(ACL.0),
                    None,
                );

                let size = std::mem::size_of::<u16>() * (value.len() + 1);
                let parameter = VirtualAllocEx(
                    process.handle,
                    None,
                    size,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_READWRITE,
                );

                _ = WriteProcessMemory(
                    process.handle,
                    parameter,
                    path.0.as_ptr() as *const c_void,
                    size,
                    None,
                );

                if let Ok(thread) = CreateRemoteThread(
                    process.handle,
                    None,
                    0,
                    Some(transmute(PROCEDURE.0)),
                    Some(parameter as *mut c_void),
                    0,
                    None,
                ) {
                    WaitForSingleObject(thread, INFINITE);
                    _ = CloseHandle(thread);
                }

                _ = VirtualFreeEx(process.handle, parameter, 0, MEM_RELEASE);
            }
        }
    } else {
        return Err(Error::empty());
    }
    Ok(())
}

pub struct Loader;

impl Loader {
    pub fn launch(value: &str) -> Result<u32> {
        let process = Game::activate()?;
        load(&process, value)?;
        Ok(process.id)
    }
}
