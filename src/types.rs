//! Traits for loading/saving Registry values
extern crate winapi;
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
use super::{RegError,RegResult};

/// A trait for types that can be loaded from registry values.
pub trait FromReg {
    fn convert_from_bytes(buf: Vec<u16>, buf_type: winapi::DWORD) -> RegResult<Self>;
}

impl FromReg for String {
    fn convert_from_bytes(buf: Vec<u16>, buf_type: winapi::DWORD) -> RegResult<String> {
        match buf_type {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                match String::from_utf16(&buf) {
                    Ok(mut s) => {
                        s.pop(); // remove trailing \0
                        if buf_type == REG_MULTI_SZ {
                            return Ok(s.replace("\u{0}", "\n"))
                        }
                        Ok(s)
                    },
                    Err(_) => Err(RegError{ err: winapi::ERROR_INVALID_BLOCK })
                }
            },
            _ => Err(RegError{ err: winapi::ERROR_BAD_FILE_TYPE })
        }
    }
}

impl FromReg for u32 {
    fn convert_from_bytes(buf: Vec<u16>, buf_type: winapi::DWORD) -> RegResult<u32> {
        match buf_type {
            REG_DWORD => {
                Ok(
                    ((buf[1] as u32) << 16) |
                    (buf[0] as u32)
                )
            },
            _ => Err(RegError{ err: winapi::ERROR_BAD_FILE_TYPE })
        }
    }
}

/// A trait for types that can be written into registry values.
pub trait ToReg {
    fn get_val_type(&self) -> winapi::DWORD;
    fn convert_to_bytes(&self) -> Vec<u16>;
}

impl ToReg for String {
    fn get_val_type(&self) -> winapi::DWORD {REG_SZ}

    fn convert_to_bytes(&self) -> Vec<u16> {
        super::to_utf16(self)
    }
}

impl<'a> ToReg for &'a str {
    fn get_val_type(&self) -> winapi::DWORD {REG_SZ}

    fn convert_to_bytes(&self) -> Vec<u16> {
        super::to_utf16(self)
    }
}

impl ToReg for u32 {
    fn get_val_type(&self) -> winapi::DWORD {REG_DWORD}

    fn convert_to_bytes(&self) -> Vec<u16> {
        let mut bytes: Vec<u16> = Vec::with_capacity(2);
        bytes.push((self & 0xFFFF) as u16);
        bytes.push(((self & 0xFFFF0000) >> 16) as u16);
        bytes
    }
}
