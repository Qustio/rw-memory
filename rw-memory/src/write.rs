#![allow(unused_imports)]
use std::error::Error;
use std::ffi::c_void;
use std::mem::size_of;
use std::{error, ptr};

use std::fmt::{Display, Formatter, Result};
use std::mem;
use std::str;
use windows::Win32::Foundation::*;
use windows::Win32::Security;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::System::Threading::*;
use windows::Win32::System::*;
#[derive(Debug)]
pub struct WriteMemoryError {
    str: String,
}

impl WriteMemoryError {
    fn new(msg: i32) -> WriteMemoryError {
        WriteMemoryError {
            str: format!("Write memory error: {}", msg),
        }
    }
}

impl Display for WriteMemoryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", &self.str)
    }
}

impl Error for WriteMemoryError {
    fn description(&self) -> &str {
        &self.str
    }
}

pub fn write_process_memory<T: std::fmt::Debug + Default>(
    h_process: HANDLE,
    lpbaseaddress: u64,
    value: T,
) -> std::result::Result<(), WriteMemoryError> {
    unsafe {
        match windows::Win32::System::Diagnostics::Debug::WriteProcessMemory(
            h_process,
            lpbaseaddress as *const c_void,
            &value as *const _ as *const c_void,
            mem::size_of::<T>(),
            std::ptr::null_mut(),
        )
        .0
        {
            1 => Ok(()),
            default => Err(WriteMemoryError::new(default)),
        }
    }
}
