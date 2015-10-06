// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
//! Traits for loading/saving Registry values
extern crate winapi;
use std::slice;
use std::io;
use winapi::winerror;
use super::{RegValue};
use enums::*;
use super::{to_utf16,v16_to_v8};

/// A trait for types that can be loaded from registry values.
pub trait FromRegValue : Sized {
    fn from_reg_value(val: &RegValue) -> io::Result<Self>;
}

impl FromRegValue for String {
    fn from_reg_value(val: &RegValue) -> io::Result<String> {
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
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE)
        }
    }
}

impl FromRegValue for u32 {
    fn from_reg_value(val: &RegValue) -> io::Result<u32> {
        match val.vtype {
            REG_DWORD => {
                Ok(unsafe{ *(val.bytes.as_ptr() as *const u32) })
            },
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE)
        }
    }
}

impl FromRegValue for u64 {
    fn from_reg_value(val: &RegValue) -> io::Result<u64> {
        match val.vtype {
            REG_QWORD => {
                Ok(unsafe{ *(val.bytes.as_ptr() as *const u64) })
            },
            _ => werr!(winerror::ERROR_BAD_FILE_TYPE)
        }
    }
}

/// A trait for types that can be written into registry values.
pub trait ToRegValue {
    fn to_reg_value(&self) -> RegValue;
}

impl ToRegValue for String {
    fn to_reg_value(&self) -> RegValue {
        RegValue{
            bytes: v16_to_v8(&to_utf16(self)),
            vtype: REG_SZ
        }
    }
}

impl<'a> ToRegValue for &'a str {
    fn to_reg_value(&self) -> RegValue {
        RegValue{
            bytes: v16_to_v8(&to_utf16(self)),
            vtype: REG_SZ
        }
    }
}

impl ToRegValue for u32 {
    fn to_reg_value(&self) -> RegValue {
        let bytes: Vec<u8> = unsafe {
            slice::from_raw_parts((self as *const u32) as *const u8, 4).to_vec()
        };
        RegValue{
            bytes: bytes,
            vtype: REG_DWORD
        }
    }
}

impl ToRegValue for u64 {
    fn to_reg_value(&self) -> RegValue {
        let bytes: Vec<u8> = unsafe {
            slice::from_raw_parts((self as *const u64) as *const u8, 8).to_vec()
        };
        RegValue{
            bytes: bytes,
            vtype: REG_QWORD
        }
    }
}
