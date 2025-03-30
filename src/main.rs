use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::process;
use std::mem;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use winapi::um::processthreadsapi::{OpenProcess, CreateRemoteThread};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, PROCESS_ALL_ACCESS};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winbase::LoadLibraryW;

fn get_process_id(proc_name: &str) -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return None;
        }

        let mut proc_entry: PROCESSENTRY32 = mem::zeroed();
        proc_entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut proc_entry) != 0 {
            loop {
                let exe_name = CString::new(proc_name).unwrap();
                let current_name = CString::new(proc_entry.szExeFile.iter().map(|&c| c as u8).collect::<Vec<_>>()).unwrap();

                if exe_name.as_c_str() == current_name.as_c_str() {
                    CloseHandle(snapshot);
                    return Some(proc_entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut proc_entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
        None
    }
}

fn inject_dll(proc_id: u32, dll_path: &str) -> bool {
    unsafe {
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, proc_id);
        if h_process.is_null() {
            return false;
        }

        let wide_path: Vec<u16> = OsStr::new(dll_path).encode_wide().chain(Some(0)).collect();
        let alloc_size = wide_path.len() * mem::size_of::<u16>();
        let alloc_address = VirtualAllocEx(h_process, ptr::null_mut(), alloc_size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        
        if alloc_address.is_null() {
            CloseHandle(h_process);
            return false;
        }

        if WriteProcessMemory(h_process, alloc_address, wide_path.as_ptr() as *const _, alloc_size, ptr::null_mut()) == 0 {
            CloseHandle(h_process);
            return false;
        }

        let kernel32 = GetModuleHandleA(b"kernel32.dll\0".as_ptr() as *const _);
        if kernel32.is_null() {
            CloseHandle(h_process);
            return false;
        }

        let h_thread = CreateRemoteThread(
            h_process,
            ptr::null_mut(),
            0,
            Some(mem::transmute(LoadLibraryW as *const ())),
            alloc_address,
            0,
            ptr::null_mut(),
        );

        if !h_thread.is_null() {
            CloseHandle(h_thread);
        }

        CloseHandle(h_process);
        true
    }
}