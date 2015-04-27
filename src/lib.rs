// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
//! Crate for accessing MS Windows registry
extern crate winapi;
extern crate kernel32;
extern crate advapi32;
use std::ptr;
use std::slice;
use std::fmt;
use std::default::Default;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem::transmute;
use winapi::winerror;
use enums::*;
use types::{FromReg, ToReg};

pub mod enums;
pub mod types;

pub struct RegError {
    err: winapi::DWORD,
}

impl fmt::Debug for RegError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegError {{ err: {:?}, message: {:?} }}",
                   self.err, error_string(self.err))
    }
}

pub type RegResult<T> = std::result::Result<T, RegError>;

#[derive(Debug,Default)]
pub struct RegKeyMetadata {
    // Class: winapi::LPWSTR,
    // ClassLen: winapi::DWORD,
    sub_keys: winapi::DWORD,
    max_sub_key_len: winapi::DWORD,
    max_class_len: winapi::DWORD,
    values: winapi::DWORD,
    max_value_name_len: winapi::DWORD,
    max_value_len: winapi::DWORD,
    // SecurityDescriptor: winapi::DWORD,
    // LastWriteTime: winapi::PFILETIME,
}

pub struct RegValue {
    bytes: Vec<u8>,
    vtype: RegType,
}

impl fmt::Debug for RegValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_val = match self.vtype {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                format!("{:?}", String::convert_from_bytes(self).unwrap())
            },
            REG_DWORD => {
                let dword_val = u32::convert_from_bytes(self).unwrap();
                format!("{:?}", dword_val)
            },
            REG_QWORD => {
                let dword_val = u64::convert_from_bytes(self).unwrap();
                format!("{:?}", dword_val)
            },
            _ => format!("{:?}", self.bytes) //TODO: implement more types
        };
        write!(f, "RegValue({:?}: {})", self.vtype, f_val)
    }
}

#[derive(Debug)]
pub struct RegKey {
    hkey: winapi::HKEY,
}

impl RegKey {
    pub fn predef(hkey: winapi::HKEY) -> RegKey {
        RegKey{ hkey: hkey }
    }

    /// Open subkey with `KEY_ALL_ACCESS` permissions.
    /// To open with different permissions use `open_subkey_with_flags`.
    pub fn open_subkey<P: AsRef<OsStr>>(&self, path: P) -> RegResult<RegKey> {
        self.open_subkey_with_flags(path, winapi::KEY_ALL_ACCESS)
    }

    pub fn open_subkey_with_flags<P: AsRef<OsStr>>(&self, path: P, perms: winapi::REGSAM) -> RegResult<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: winapi::HKEY = ptr::null_mut();
        match unsafe {
            advapi32::RegOpenKeyExW(
                self.hkey,
                c_path.as_ptr(),
                0,
                perms,
                &mut new_hkey,
            ) as winapi::DWORD
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => Err(RegError{ err: err })
        }
    }

    /// Create subkey (and all missing parent keys)
    /// and open it with `KEY_ALL_ACCESS` permissions.
    /// Will just open key if it already exists.
    /// To create with different permissions use `create_subkey_with_flags`.
    pub fn create_subkey<P: AsRef<OsStr>>(&self, path: P) -> RegResult<RegKey> {
        self.create_subkey_with_flags(path, winapi::KEY_ALL_ACCESS)
    }

    pub fn create_subkey_with_flags<P: AsRef<OsStr>>(&self, path: P, perms: winapi::REGSAM) -> RegResult<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: winapi::HKEY = ptr::null_mut();
        let mut disp: winapi::DWORD = 0;
        match unsafe {
            advapi32::RegCreateKeyExW(
                self.hkey,
                c_path.as_ptr(),
                0,
                ptr::null(),
                winapi::REG_OPTION_NON_VOLATILE,
                perms,
                ptr::null_mut(),
                &mut new_hkey,
                &mut disp // TODO: return this somehow
            ) as winapi::DWORD
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => Err(RegError{ err: err })
        }
    }

    pub fn query_info(&self) -> RegResult<RegKeyMetadata> {
        let mut info: RegKeyMetadata = Default::default();
        match unsafe {
            advapi32::RegQueryInfoKeyW(
                self.hkey,
                ptr::null_mut(), // Class: winapi::LPWSTR,
                ptr::null_mut(), // ClassLen: winapi::DWORD,
                ptr::null_mut(), // Reserved
                &mut info.sub_keys,
                &mut info.max_sub_key_len,
                &mut info.max_class_len,
                &mut info.values,
                &mut info.max_value_name_len,
                &mut info.max_value_len,
                ptr::null_mut(), // lpcbSecurityDescriptor: LPDWORD,
                ptr::null_mut(), // lpftLastWriteTime: PFILETIME,
            ) as winapi::DWORD
        } {
            0 => Ok(info),
            err => Err(RegError{ err: err })
        }
    }

    pub fn enum_keys<'a>(&'a self) -> EnumKeys<'a> {
        EnumKeys{key: self, index: 0}
    }

    pub fn enum_values<'a>(&'a self) -> EnumValues<'a> {
        EnumValues{key: self, index: 0}
    }

    /// Delete key. Cannot delete if it nas subkeys.
    /// Use `delete_subkey_all` for that.
    pub fn delete_subkey<P: AsRef<OsStr>>(&self, path: P) -> RegResult<()> {
        let c_path = to_utf16(path);
        match unsafe {
            advapi32::RegDeleteKeyW(
                self.hkey,
                c_path.as_ptr(),
            ) as winapi::DWORD
        } {
            0 => Ok(()),
            err => Err(RegError{ err: err })
        }
    }

    /// Recursively delete key with all its subkeys and values.
    pub fn delete_subkey_all<P: AsRef<OsStr>>(&self, path: P) -> RegResult<()> {
        let c_path = to_utf16(path);
        match unsafe{
            advapi32::RegDeleteTreeW(
                self.hkey,
                c_path.as_ptr(),
            ) as winapi::DWORD
        } {
            0 => Ok(()),
            err => Err(RegError{ err: err })
        }
    }

    /// Get the `Default` value if `name` is an empty string
    pub fn get_value<T: FromReg, P: AsRef<OsStr>>(&self, name: P) -> RegResult<T> {
        match self.get_raw_value(name) {
            Ok(ref val) => FromReg::convert_from_bytes(val),
            Err(err) => Err(err)
        }
    }

    pub fn get_raw_value<P: AsRef<OsStr>>(&self, name: P) -> RegResult<RegValue> {
        let c_name = to_utf16(name);
        let mut buf_len: winapi::DWORD = 2048;
        let mut buf_type: winapi::DWORD = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
        match unsafe {
            advapi32::RegQueryValueExW(
                self.hkey,
                c_name.as_ptr() as *const u16,
                ptr::null_mut(),
                &mut buf_type,
                buf.as_mut_ptr() as winapi::LPBYTE,
                &mut buf_len
            ) as winapi::DWORD
        } {
            0 => {
                unsafe{ buf.set_len(buf_len as usize); }
                // minimal check before transmute to RegType
                if buf_type > winapi::REG_QWORD {
                    return Err(RegError{
                        err: winerror::ERROR_BAD_FILE_TYPE
                    });
                }
                let t: RegType = unsafe{ transmute(buf_type as u8) };
                Ok(RegValue{ bytes: buf, vtype: t })
            },
            err => Err(RegError{ err: err })
        }
    }

    /// Set the `Default` value if `name` is an empty string
    pub fn set_value<T: ToReg, P: AsRef<OsStr>>(&self, name: P, value: &T) -> RegResult<()> {
        self.set_raw_value(name, &value.convert_to_bytes())
    }

    pub fn set_raw_value<P: AsRef<OsStr>>(&self, name: P, value: &RegValue) -> RegResult<()> {
        let c_name = to_utf16(name);
        let t = value.vtype.clone() as winapi::DWORD;
        match unsafe{
            advapi32::RegSetValueExW(
                self.hkey,
                c_name.as_ptr(),
                0,
                t,
                value.bytes.as_ptr() as *const winapi::BYTE,
                value.bytes.len() as u32
            ) as winapi::DWORD
        } {
            0 => Ok(()),
            err => Err(RegError{ err: err })
        }
    }

    /// Delete the `Default` value if `name` is an empty string
    pub fn delete_value<P: AsRef<OsStr>>(&self, name: P) -> RegResult<()> {
        let c_name = to_utf16(name);
        match unsafe {
            advapi32::RegDeleteValueW(
                self.hkey,
                c_name.as_ptr(),
            ) as winapi::DWORD
        } {
            0 => Ok(()),
            err => Err(RegError{ err: err })
        }
    }

    fn close_(&mut self) -> RegResult<()> {
        // don't try to close predefined keys
        if self.hkey >= winapi::HKEY_CLASSES_ROOT { return Ok(()) };
        match unsafe {
            advapi32::RegCloseKey(self.hkey) as winapi::DWORD
        } {
            0 => Ok(()),
            err => Err(RegError{ err: err })
        }
    }
}

impl Drop for RegKey {
    fn drop(&mut self) {
        self.close_().unwrap();
    }
}

pub struct EnumKeys<'key> {
    key: &'key RegKey,
    index: winapi::DWORD,
}

impl<'key> Iterator for EnumKeys<'key> {
    type Item = RegResult<String>;

    fn next(&mut self) -> Option<RegResult<String>> {
        let mut name_len = 2048;
        let mut name = [0 as winapi::WCHAR; 2048];
        match unsafe {
            advapi32::RegEnumKeyExW(
                self.key.hkey,
                self.index,
                name.as_mut_ptr(),
                &mut name_len,
                ptr::null_mut(), // reserved
                ptr::null_mut(), // lpClass: LPWSTR,
                ptr::null_mut(), // lpcClass: LPDWORD,
                ptr::null_mut(), // lpftLastWriteTime: PFILETIME,
            ) as winapi::DWORD
        } {
            0 => {
                self.index += 1;
                Some(match String::from_utf16(&name[..name_len as usize]) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(RegError{ err: winerror::ERROR_INVALID_BLOCK })
                })
            },
            winerror::ERROR_NO_MORE_ITEMS => None,
            err => {
                Some(Err(RegError{ err: err }))
            }
        }
    }
}

pub struct EnumValues<'key> {
    key: &'key RegKey,
    index: winapi::DWORD,
}

impl<'key> Iterator for EnumValues<'key> {
    type Item = RegResult<(String, RegValue)>;

    fn next(&mut self) -> Option<RegResult<(String, RegValue)>> {
        let mut name_len = 2048;
        let mut name = [0 as winapi::WCHAR; 2048];

        let mut buf_len: winapi::DWORD = 2048;
        let mut buf_type: winapi::DWORD = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
        match unsafe {
            advapi32::RegEnumValueW(
                self.key.hkey,
                self.index,
                name.as_mut_ptr(),
                &mut name_len,
                ptr::null_mut(), // reserved
                &mut buf_type,
                buf.as_mut_ptr() as winapi::LPBYTE,
                &mut buf_len,
            ) as winapi::DWORD
        } {
            0 => {
                self.index += 1;
                let name = String::from_utf16(&name[..name_len as usize]).unwrap();
                unsafe{ buf.set_len(buf_len as usize); }
                // minimal check before transmute to RegType
                if buf_type > winapi::REG_QWORD {
                    return Some(Err(RegError{
                        err: winerror::ERROR_BAD_FILE_TYPE
                    }));
                }
                let t: RegType = unsafe{ transmute(buf_type as u8) };
                let value = RegValue{ bytes: buf, vtype: t };
                Some(Ok((name, value)))
            },
            winerror::ERROR_NO_MORE_ITEMS => None,
            err => {
                Some(Err(RegError{ err: err }))
            }
        }
    }
}

fn to_utf16<P: AsRef<OsStr>>(s: P) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0).into_iter()).collect()
}

fn v16_to_v8(v: &Vec<u16>) -> Vec<u8> {
    let res: Vec<u8> = unsafe {
        slice::from_raw_parts(v.as_ptr() as *const u8, v.len()*2).to_vec()
    };
    res
}

// copycat of rust/src/libstd/sys/windows/os.rs::error_string
// `use std::sys::os::error_string` leads to
// error: function `error_string` is private.
// Get a detailed string description for the given error number
fn error_string(errnum: winapi::DWORD) -> String {
    let mut buf = [0 as winapi::WCHAR; 2048];
    unsafe {
        let res = kernel32::FormatMessageW(winapi::FORMAT_MESSAGE_FROM_SYSTEM |
                                 winapi::FORMAT_MESSAGE_IGNORE_INSERTS,
                                 ptr::null_mut(),
                                 errnum as winapi::DWORD,
                                 0,
                                 buf.as_mut_ptr(),
                                 buf.len() as winapi::DWORD,
                                 ptr::null_mut());
        if res == 0 {
            // Sometimes FormatMessageW can fail e.g. system doesn't like langId,
            // let fm_err = errno();
            return format!("OS Error {} (FormatMessageW() returned error)",
                           errnum);
        }

        let b = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        let msg = String::from_utf16(&buf[..b]);
        match msg {
            Ok(msg) => msg.trim_right().to_string(),
            Err(..) => format!("OS Error {} (FormatMessageW() returned \
                                invalid UTF-16)", errnum),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::types::*;

    #[test]
    fn test_open_subkey_with_flags_query_info() {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let win = hklm.open_subkey_with_flags("Software\\Microsoft\\Windows", KEY_READ).unwrap();
        assert!(win.query_info().is_ok());
        assert!(win.open_subkey_with_flags("CurrentVersion\\", KEY_READ).is_ok());
        assert!(hklm.open_subkey_with_flags("i\\just\\hope\\nobody\\created\\that\\key", KEY_READ).is_err());
    }

    macro_rules! with_key {
        ($k:ident, $path:expr => $b:block) => {{
            let mut path = "Software\\WinRegRsTest".to_string();
            path.push_str($path);
            let $k = RegKey::predef(HKEY_CURRENT_USER)
                .create_subkey(&path).unwrap();
            $b
            RegKey::predef(HKEY_CURRENT_USER)
            .delete_subkey_all(path).unwrap();
        }}
    }

    #[test]
    #[allow(unused_variables)]
    fn test_create_delete_all_subkey() {
        with_key!(key, "CreateDeleteAllSubkey" => {});
    }

    #[test]
    fn test_delete_subkey() {
        let path = "Software\\WinRegRsTestDeleteSubkey";
        RegKey::predef(HKEY_CURRENT_USER).create_subkey(path).unwrap();
        assert!(RegKey::predef(HKEY_CURRENT_USER)
            .delete_subkey(path).is_ok());
    }

    #[test]
    fn test_string_value() {
        with_key!(key, "StringValue" => {
            let name = "RustStringVal";
            let val1 = "Test123 \n$%^&|+-*/\\()".to_string();
            key.set_value(name, &val1).unwrap();
            let val2: String = key.get_value(name).unwrap();
            assert_eq!(val1, val2);
        });
    }

    #[test]
    fn test_u32_value() {
        with_key!(key, "U32Value" => {
            let name = "RustU32Val";
            let val1 = 1234567890u32;
            key.set_value(name, &val1).unwrap();
            let val2: u32 = key.get_value(name).unwrap();
            assert_eq!(val1, val2);
        });
    }

    #[test]
    fn test_u64_value() {
        with_key!(key, "U64Value" => {
            let name = "RustU64Val";
            let val1 = 1234567891011121314u64;
            key.set_value(name, &val1).unwrap();
            let val2: u64 = key.get_value(name).unwrap();
            assert_eq!(val1, val2);
        });
    }

    #[test]
    fn test_delete_value() {
        with_key!(key, "DeleteValue" => {
            let name = "WinregRsTestVal";
            key.set_value(name, &"Qwerty123").unwrap();
            assert!(key.delete_value(name).is_ok());
        });
    }

    #[test]
    fn test_enum_keys() {
        with_key!(key, "EnumKeys" => {
            let mut keys1 = vec!("qwerty", "asdf", "1", "2", "3", "5", "8", "йцукен");
            keys1.sort();
            for i in &keys1 {
                key.create_subkey(i).unwrap();
            }
            let keys2: Vec<_> = key.enum_keys().map(|x| x.unwrap()).collect();
            assert_eq!(keys1, keys2);
        });
    }

    #[test]
    fn test_enum_values() {
        with_key!(key, "EnumValues" => {
            let mut vals1 = vec!("qwerty", "asdf", "1", "2", "3", "5", "8", "йцукен");
            vals1.sort();
            for i in &vals1 {
                key.set_value(i,i).unwrap();
            }
            let mut vals2: Vec<String> = Vec::with_capacity(vals1.len());
            let mut vals3: Vec<String> = Vec::with_capacity(vals1.len());
            for (name, val) in key.enum_values()
                .map(|x| x.unwrap())
            {
                vals2.push(name);
                vals3.push(String::convert_from_bytes(&val).unwrap());
            }
            assert_eq!(vals1, vals2);
            assert_eq!(vals1, vals3);
        });
    }
}
