pub mod read;
pub mod write;

use std::mem;
use windows::Win32::{
    Foundation::*,
    System::{Diagnostics::ToolHelp::*, ProcessStatus::*, Threading::*},
};

pub fn open_process(proc_id: u32) -> HANDLE {
    unsafe {
        windows::Win32::System::Threading::OpenProcess(PROCESS_ALL_ACCESS, false, proc_id).unwrap()
    }
}

pub fn get_proc_id_from_string(prog_name: String) -> u32 {
    let h_snapshot: HANDLE;
    let mut pe32 = PROCESSENTRY32::default();
    let mut pid: u32 = 0;
    pe32.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;
    unsafe {
        let s = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2u32);
        h_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
        if windows::Win32::System::Diagnostics::ToolHelp::Process32First(h_snapshot, &mut pe32)
            .as_bool()
        {
            loop {
                let name =
                    String::from_utf8(pe32.szExeFile.iter().map(|&d| d.0).collect::<Vec<u8>>())
                        .unwrap();
                if prog_name == name.chars().take(prog_name.len()).collect::<String>() {
                    pid = pe32.th32ProcessID;
                    break;
                }
                for elem in pe32.szExeFile.iter_mut() {
                    *elem = CHAR { 0: 0 };
                }
                if Process32Next(h_snapshot, &mut pe32).as_bool() == false {
                    break;
                };
            }
        }

        if h_snapshot != INVALID_HANDLE_VALUE {
            CloseHandle(h_snapshot);
        }
        return pid;
    }
}

pub fn jump(h_process: HANDLE, base: u64, offsets: &[u64]) -> u64 {
    if offsets.len() == 1 {
        return base + offsets[0];
    }
    let mut b = base;
    b += offsets[0];
    let mut p: u64 = 0;
    read::read_process_memory(h_process, b, &mut p);
    jump(h_process, p, &offsets[1..])
}

pub fn get_module_base_address(h_process: HANDLE, prog_name: String, p_id: u32) -> u64 {
    let mut dw_module_base_address: u64 = 0;
    let h_snapshot: HANDLE;
    let mut mod_entry: MODULEENTRY32 = MODULEENTRY32::default();
    unsafe {
        let s = TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32;
        h_snapshot = CreateToolhelp32Snapshot(s, p_id).unwrap();
        mod_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
        if Module32First(h_snapshot, &mut mod_entry).as_bool() {
            loop {
                let name =
                    String::from_utf8(mod_entry.szModule.iter().map(|&d| d.0).collect::<Vec<u8>>())
                        .unwrap();
                println!("{}", name);
                if prog_name == name.chars().take(prog_name.len()).collect::<String>() {
                    dw_module_base_address = mod_entry.modBaseAddr as u64;
                    break;
                }
                for elem in mod_entry.szModule.iter_mut() {
                    *elem = CHAR(0);
                }
                if !Module32Next(h_snapshot, &mut mod_entry).as_bool() {
                    break;
                }
            }
        }
        CloseHandle(h_snapshot);
        dw_module_base_address
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
