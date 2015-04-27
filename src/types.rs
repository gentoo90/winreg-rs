// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
//! Traits for loading/saving Registry values
extern crate winapi;
use std::slice;
pub use winapi::{HKEY_CLASSES_ROOT,
                 HKEY_CURRENT_USER,
                 HKEY_LOCAL_MACHINE,
                 HKEY_USERS,
                 HKEY_PERFORMANCE_DATA,
                 HKEY_PERFORMANCE_TEXT,
                 HKEY_PERFORMANCE_NLSTEXT,
                 HKEY_CURRENT_CONFIG,
                 HKEY_DYN_DATA,
                 HKEY_CURRENT_USER_LOCAL_SETTINGS};
pub use winapi::{KEY_QUERY_VALUE,
                 KEY_SET_VALUE,
                 KEY_CREATE_SUB_KEY,
                 KEY_ENUMERATE_SUB_KEYS,
                 KEY_NOTIFY,
                 KEY_CREATE_LINK,
                 KEY_WOW64_32KEY,
                 KEY_WOW64_64KEY,
                 KEY_WOW64_RES,
                 KEY_READ,
                 KEY_WRITE,
                 KEY_EXECUTE,
                 KEY_ALL_ACCESS};
pub use winapi::{REG_NONE,
                 REG_SZ,
                 REG_EXPAND_SZ,
                 REG_BINARY,
                 REG_DWORD,
                 REG_DWORD_LITTLE_ENDIAN,
                 REG_DWORD_BIG_ENDIAN,
                 REG_LINK,
                 REG_MULTI_SZ,
                 REG_RESOURCE_LIST,
                 REG_FULL_RESOURCE_DESCRIPTOR,
                 REG_RESOURCE_REQUIREMENTS_LIST,
                 REG_QWORD,
                 REG_QWORD_LITTLE_ENDIAN};
use super::{RegError,RegResult,RegValue};
use super::{to_utf16,v16_to_v8};

/// A trait for types that can be loaded from registry values.
pub trait FromReg {
    fn convert_from_bytes(val: &RegValue) -> RegResult<Self>;
}

impl FromReg for String {
    fn convert_from_bytes(val: &RegValue) -> RegResult<String> {
        match val.vtype {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                let words = unsafe {
                    let pwords = val.bytes.as_ptr() as *mut [u16; 2048 as usize];
                    *pwords
                };
                let mut s:String = String::from_utf16_lossy(&words[..(val.bytes.len()/2)]);
                while s.ends_with("\u{0}") {s.pop();}
                if val.vtype == REG_MULTI_SZ {
                    return Ok(s.replace("\u{0}", "\n"))
                }
                Ok(s)
            },
            _ => Err(RegError{ err: winapi::ERROR_BAD_FILE_TYPE })
        }
    }
}

impl FromReg for u32 {
    fn convert_from_bytes(val: &RegValue) -> RegResult<u32> {
        match val.vtype {
            REG_DWORD => {
                Ok(unsafe{ *(val.bytes.as_ptr() as *const u32) })
            },
            _ => Err(RegError{ err: winapi::ERROR_BAD_FILE_TYPE })
        }
    }
}

impl FromReg for u64 {
    fn convert_from_bytes(val: &RegValue) -> RegResult<u64> {
        match val.vtype {
            REG_QWORD => {
                Ok(unsafe{ *(val.bytes.as_ptr() as *const u64) })
            },
            _ => Err(RegError{ err: winapi::ERROR_BAD_FILE_TYPE })
        }
    }
}

/// A trait for types that can be written into registry values.
pub trait ToReg {
    fn convert_to_bytes(&self) -> RegValue;
}

impl ToReg for String {
    fn convert_to_bytes(&self) -> RegValue {
        RegValue{
            bytes: v16_to_v8(&to_utf16(self)),
            vtype: REG_SZ
        }
    }
}

impl<'a> ToReg for &'a str {
    fn convert_to_bytes(&self) -> RegValue {
        RegValue{
            bytes: v16_to_v8(&to_utf16(self)),
            vtype: REG_SZ
        }
    }
}

impl ToReg for u32 {
    fn convert_to_bytes(&self) -> RegValue {
        let bytes: Vec<u8> = unsafe {
            slice::from_raw_parts((self as *const u32) as *const u8, 4).to_vec()
        };
        RegValue{
            bytes: bytes,
            vtype: REG_DWORD
        }
    }
}

impl ToReg for u64 {
    fn convert_to_bytes(&self) -> RegValue {
        let bytes: Vec<u8> = unsafe {
            slice::from_raw_parts((self as *const u64) as *const u8, 8).to_vec()
        };
        RegValue{
            bytes: bytes,
            vtype: REG_QWORD
        }
    }
}
