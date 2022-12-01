#![allow(unused_imports)]
use std::error::Error;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;

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
pub struct ReadMemoryError {
    str: String,
}

impl ReadMemoryError {
    fn new(msg: i32) -> ReadMemoryError {
        ReadMemoryError {
            str: format!("Read memory error: {}", msg),
        }
    }
}

impl Display for ReadMemoryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", &self.str)
    }
}

impl Error for ReadMemoryError {
    fn description(&self) -> &str {
        &self.str
    }
}

pub fn read_process_memory<T: std::fmt::Debug + Default>(
    h_process: HANDLE,
    lpbaseaddress: u64,
    value: &mut T,
) -> std::result::Result<(), ReadMemoryError> {
    unsafe {
        match windows::Win32::System::Diagnostics::Debug::ReadProcessMemory(
            h_process,
            lpbaseaddress as *const c_void,
            value as *mut _ as *mut c_void,
            size_of::<T>(),
            std::ptr::null_mut(),
        )
        .0
        {
            1 => Ok(()),
            default => Err(ReadMemoryError::new(default)),
        }
    }
}
