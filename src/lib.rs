// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
//! Crate for accessing MS Windows registry
extern crate winapi;
extern crate kernel32;
extern crate advapi32;
extern crate ktmw32;
extern crate rustc_serialize;
use std::ptr;
use std::slice;
use std::fmt;
use std::default::Default;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem::transmute;
use std::io;
use winapi::winerror;
use winapi::{HKEY, DWORD, WCHAR};
use enums::*;
use types::{FromRegValue, ToRegValue};
use transaction::Transaction;

macro_rules! werr {
    ($e:expr) => (
        Err(io::Error::from_raw_os_error($e as i32))
    )
}

pub mod enums;
pub mod types;
pub mod serialization;
pub mod transaction;

#[derive(Debug,Default)]
pub struct RegKeyMetadata {
    // Class: winapi::LPWSTR,
    // ClassLen: DWORD,
    sub_keys: DWORD,
    max_sub_key_len: DWORD,
    max_class_len: DWORD,
    values: DWORD,
    max_value_name_len: DWORD,
    max_value_len: DWORD,
    // SecurityDescriptor: DWORD,
    // LastWriteTime: winapi::PFILETIME,
}

/// Raw registry value
#[derive(PartialEq)]
pub struct RegValue {
    pub bytes: Vec<u8>,
    pub vtype: RegType,
}

impl fmt::Debug for RegValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_val = match self.vtype {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                format!("{:?}", String::from_reg_value(self).unwrap())
            },
            REG_DWORD => {
                let dword_val = u32::from_reg_value(self).unwrap();
                format!("{:?}", dword_val)
            },
            REG_QWORD => {
                let dword_val = u64::from_reg_value(self).unwrap();
                format!("{:?}", dword_val)
            },
            _ => format!("{:?}", self.bytes) //TODO: implement more types
        };
        write!(f, "RegValue({:?}: {})", self.vtype, f_val)
    }
}

/// Handle of opened registry key
#[derive(Debug)]
pub struct RegKey {
    hkey: HKEY,
}

impl RegKey {
    /// Open one of predefined keys:
    ///
    /// * `HKEY_CLASSES_ROOT`
    /// * `HKEY_CURRENT_USER`
    /// * `HKEY_LOCAL_MACHINE`
    /// * `HKEY_USERS`
    /// * `HKEY_PERFORMANCE_DATA`
    /// * `HKEY_PERFORMANCE_TEXT`
    /// * `HKEY_PERFORMANCE_NLSTEXT`
    /// * `HKEY_CURRENT_CONFIG`
    /// * `HKEY_DYN_DATA`
    /// * `HKEY_CURRENT_USER_LOCAL_SETTINGS`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    /// ```
    pub fn predef(hkey: HKEY) -> RegKey {
        RegKey{ hkey: hkey }
    }

    /// Open subkey with `KEY_ALL_ACCESS` permissions.
    /// Will open another handle to itself if `path` is an empty string.
    /// To open with different permissions use `open_subkey_with_flags`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// let soft = RegKey::predef(HKEY_CURRENT_USER)
    ///     .open_subkey("Software").unwrap();
    /// ```
    pub fn open_subkey<P: AsRef<OsStr>>(&self, path: P) -> io::Result<RegKey> {
        self.open_subkey_with_flags(path, winapi::KEY_ALL_ACCESS)
    }

    /// Open subkey with desired permissions.
    /// Will open another handle to itself if `path` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    /// hklm.open_subkey_with_flags("SOFTWARE\\Microsoft", KEY_READ).unwrap();
    /// ```
    pub fn open_subkey_with_flags<P: AsRef<OsStr>>(&self, path: P, perms: winapi::REGSAM) -> io::Result<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = ptr::null_mut();
        match unsafe {
            advapi32::RegOpenKeyExW(
                self.hkey,
                c_path.as_ptr(),
                0,
                perms,
                &mut new_hkey,
            ) as DWORD
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => werr!(err)
        }
    }

    pub fn open_subkey_transacted<P: AsRef<OsStr>>(&self, path: P, t: &Transaction) -> io::Result<RegKey> {
        self.open_subkey_transacted_with_flags(path, t, winapi::KEY_ALL_ACCESS)
    }

    pub fn open_subkey_transacted_with_flags<P: AsRef<OsStr>>(&self, path: P, t: &Transaction, perms: winapi::REGSAM)
        -> io::Result<RegKey>
    {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = ptr::null_mut();
        match unsafe {
            advapi32::RegOpenKeyTransactedW(
                self.hkey,
                c_path.as_ptr(),
                0,
                perms,
                &mut new_hkey,
                t.handle,
                ptr::null_mut(),
            ) as DWORD
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => werr!(err)
        }
    }

    /// Create subkey (and all missing parent keys)
    /// and open it with `KEY_ALL_ACCESS` permissions.
    /// Will just open key if it already exists.
    /// Will open another handle to itself if `path` is an empty string.
    /// To create with different permissions use `create_subkey_with_flags`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.create_subkey("Software\\MyProduct\\Settings").unwrap();
    /// ```
    pub fn create_subkey<P: AsRef<OsStr>>(&self, path: P) -> io::Result<RegKey> {
        self.create_subkey_with_flags(path, winapi::KEY_ALL_ACCESS)
    }

    pub fn create_subkey_with_flags<P: AsRef<OsStr>>(&self, path: P, perms: winapi::REGSAM) -> io::Result<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = ptr::null_mut();
        let mut disp: DWORD = 0;
        match unsafe {
            advapi32::RegCreateKeyExW(
                self.hkey,
                c_path.as_ptr(),
                0,
                ptr::null_mut(),
                winapi::REG_OPTION_NON_VOLATILE,
                perms,
                ptr::null_mut(),
                &mut new_hkey,
                &mut disp // TODO: return this somehow
            ) as DWORD
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => werr!(err)
        }
    }

    pub fn create_subkey_transacted<P: AsRef<OsStr>>(&self, path: P, t: &Transaction) -> io::Result<RegKey> {
        self.create_subkey_transacted_with_flags(path, t, winapi::KEY_ALL_ACCESS)
    }

    pub fn create_subkey_transacted_with_flags<P: AsRef<OsStr>>(&self, path: P, t: &Transaction, perms: winapi::REGSAM)
        -> io::Result<RegKey>
    {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = ptr::null_mut();
        let mut disp: DWORD = 0;
        match unsafe {
            advapi32::RegCreateKeyTransactedW(
                self.hkey,
                c_path.as_ptr(),
                0,
                ptr::null_mut(),
                winapi::REG_OPTION_NON_VOLATILE,
                perms,
                ptr::null_mut(),
                &mut new_hkey,
                &mut disp, // TODO: return this somehow
                t.handle,
                ptr::null_mut(),
            ) as DWORD
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => werr!(err)
        }
    }

    pub fn query_info(&self) -> io::Result<RegKeyMetadata> {
        let mut info: RegKeyMetadata = Default::default();
        match unsafe {
            advapi32::RegQueryInfoKeyW(
                self.hkey,
                ptr::null_mut(), // Class: winapi::LPWSTR,
                ptr::null_mut(), // ClassLen: DWORD,
                ptr::null_mut(), // Reserved
                &mut info.sub_keys,
                &mut info.max_sub_key_len,
                &mut info.max_class_len,
                &mut info.values,
                &mut info.max_value_name_len,
                &mut info.max_value_len,
                ptr::null_mut(), // lpcbSecurityDescriptor: winapi::LPDWORD,
                ptr::null_mut(), // lpftLastWriteTime: winapi::PFILETIME,
            ) as DWORD
        } {
            0 => Ok(info),
            err => werr!(err)
        }
    }

    /// Return an iterator over subkeys names.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// println!("File extensions, registered in this system:");
    /// for i in RegKey::predef(HKEY_CLASSES_ROOT)
    ///     .enum_keys().map(|x| x.unwrap())
    ///     .filter(|x| x.starts_with("."))
    /// {
    ///     println!("{}", i);
    /// }
    /// ```
    pub fn enum_keys<'a>(&'a self) -> EnumKeys<'a> {
        EnumKeys{key: self, index: 0}
    }

    /// Return an iterator over values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// let system = RegKey::predef(HKEY_LOCAL_MACHINE)
    ///     .open_subkey_with_flags("HARDWARE\\DESCRIPTION\\System", KEY_READ)
    ///     .unwrap();
    /// for (name, value) in system.enum_values().map(|x| x.unwrap()) {
    ///     println!("{} = {:?}", name, value);
    /// }
    /// ```
    pub fn enum_values<'a>(&'a self) -> EnumValues<'a> {
        EnumValues{key: self, index: 0}
    }

    /// Delete key. Cannot delete if it has subkeys.
    /// Will delete itself if `path` is an empty string.
    /// Use `delete_subkey_all` for that.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// RegKey::predef(HKEY_CURRENT_USER)
    ///     .delete_subkey(r"Software\MyProduct\History").unwrap();
    /// ```
    pub fn delete_subkey<P: AsRef<OsStr>>(&self, path: P) -> io::Result<()> {
        let c_path = to_utf16(path);
        match unsafe {
            advapi32::RegDeleteKeyW(
                self.hkey,
                c_path.as_ptr(),
            ) as DWORD
        } {
            0 => Ok(()),
            err => werr!(err)
        }
    }

    pub fn delete_subkey_transacted<P: AsRef<OsStr>>(&self, path: P, t: &Transaction) -> io::Result<()> {
        let c_path = to_utf16(path);
        match unsafe {
            advapi32::RegDeleteKeyTransactedW(
                self.hkey,
                c_path.as_ptr(),
                0,
                0,
                t.handle,
                ptr::null_mut(),
            ) as DWORD
        } {
            0 => Ok(()),
            err => werr!(err)
        }
    }

    /// Recursively delete subkey with all its subkeys and values.
    /// Will delete itself if `path` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// RegKey::predef(HKEY_CURRENT_USER)
    ///     .delete_subkey_all("Software\\MyProduct").unwrap();
    /// ```
    pub fn delete_subkey_all<P: AsRef<OsStr>>(&self, path: P) -> io::Result<()> {
        let c_path = to_utf16(path);
        match unsafe{
            advapi32::RegDeleteTreeW(
                self.hkey,
                c_path.as_ptr(),
            ) as DWORD
        } {
            0 => Ok(()),
            err => werr!(err)
        }
    }

    /// Get a value from registry and seamlessly convert it to the specified rust type
    /// with `FromRegValue` implemented (currently `String`, `u32` and `u64`).
    /// Will get the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings").unwrap();
    /// let server: String = settings.get_value("server").unwrap();
    /// let port: u32 = settings.get_value("port").unwrap();
    /// ```
    pub fn get_value<T: FromRegValue, N: AsRef<OsStr>>(&self, name: N) -> io::Result<T> {
        match self.get_raw_value(name) {
            Ok(ref val) => FromRegValue::from_reg_value(val),
            Err(err) => Err(err)
        }
    }

    /// Get raw bytes from registry value.
    /// Will get the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings").unwrap();
    /// let data = settings.get_raw_value("data").unwrap();
    /// println!("Bytes: {:?}", data.bytes);
    /// ```
    pub fn get_raw_value<N: AsRef<OsStr>>(&self, name: N) -> io::Result<RegValue> {
        let c_name = to_utf16(name);
        let mut buf_len: DWORD = 2048;
        let mut buf_type: DWORD = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
        loop {
            match unsafe {
                advapi32::RegQueryValueExW(
                    self.hkey,
                    c_name.as_ptr() as *const u16,
                    ptr::null_mut(),
                    &mut buf_type,
                    buf.as_mut_ptr() as winapi::LPBYTE,
                    &mut buf_len
                ) as DWORD
            } {
                0 => {
                    unsafe{ buf.set_len(buf_len as usize); }
                    // minimal check before transmute to RegType
                    if buf_type > winapi::REG_QWORD {
                        return werr!(winerror::ERROR_BAD_FILE_TYPE);
                    }
                    let t: RegType = unsafe{ transmute(buf_type as u8) };
                    return Ok(RegValue{ bytes: buf, vtype: t })
                },
                winerror::ERROR_MORE_DATA => {
                    buf.reserve(buf_len as usize);
                },
                err => return werr!(err),
            }
        }
    }

    /// Seamlessly convert a value from a rust type and write it to the registry value
    /// with `ToRegValue` trait implemented (currently `String`, `&str`, `u32` and `u64`).
    /// Will set the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.create_subkey("Software\\MyProduct\\Settings").unwrap();
    /// settings.set_value("server", &"www.example.com").unwrap();
    /// settings.set_value("port", &8080u32).unwrap();
    /// ```
    pub fn set_value<T: ToRegValue, N: AsRef<OsStr>>(&self, name: N, value: &T) -> io::Result<()> {
        self.set_raw_value(name, &value.to_reg_value())
    }

    /// Write raw bytes from `RegValue` struct to a registry value.
    /// Will set the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winreg::{RegKey, RegValue};
    /// use winreg::enums::*;
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings").unwrap();
    /// let bytes: Vec<u8> = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    /// let data = RegValue{ vtype: REG_BINARY, bytes: bytes};
    /// settings.set_raw_value("data", &data).unwrap();
    /// println!("Bytes: {:?}", data.bytes)
    /// ```
    pub fn set_raw_value<N: AsRef<OsStr>>(&self, name: N, value: &RegValue) -> io::Result<()> {
        let c_name = to_utf16(name);
        let t = value.vtype.clone() as DWORD;
        match unsafe{
            advapi32::RegSetValueExW(
                self.hkey,
                c_name.as_ptr(),
                0,
                t,
                value.bytes.as_ptr() as *const winapi::BYTE,
                value.bytes.len() as u32
            ) as DWORD
        } {
            0 => Ok(()),
            err => werr!(err)
        }
    }

    /// Delete specified value from registry.
    /// Will delete the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings").unwrap();
    /// settings.delete_value("data").unwrap();
    /// ```
    pub fn delete_value<N: AsRef<OsStr>>(&self, name: N) -> io::Result<()> {
        let c_name = to_utf16(name);
        match unsafe {
            advapi32::RegDeleteValueW(
                self.hkey,
                c_name.as_ptr(),
            ) as DWORD
        } {
            0 => Ok(()),
            err => werr!(err)
        }
    }

    /// Save `Encodable` type to a registry key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// extern crate rustc_serialize;
    /// extern crate winreg;
    /// # fn main() {
    /// use winreg::RegKey;
    /// use winreg::enums::*;
    /// use rustc_serialize::Encodable;
    ///
    /// #[derive(RustcEncodable)]
    /// struct Rectangle{
    ///     x: u32,
    ///     y: u32,
    ///     w: u32,
    ///     h: u32,
    /// }
    ///
    /// #[derive(RustcEncodable)]
    /// struct Settings{
    ///     current_dir: String,
    ///     window_pos: Rectangle,
    ///     show_in_tray: bool,
    /// }
    ///
    /// let s: Settings = Settings{
    ///     current_dir: "C:\\".to_string(),
    ///     window_pos: Rectangle{ x:200, y: 100, w: 800, h: 500 },
    ///     show_in_tray: false,
    /// };
    /// let s_key = RegKey::predef(HKEY_CURRENT_USER)
    ///     .open_subkey("Software\\MyProduct\\Settings").unwrap();
    /// s_key.encode(&s).unwrap();
    /// # }
    /// ```
    pub fn encode<T: rustc_serialize::Encodable>(&self, value: &T)
        -> serialization::EncodeResult<()>
    {
        let mut encoder = try!(
            serialization::Encoder::from_key(&self)
        );
        try!(value.encode(&mut encoder));
        encoder.commit()
    }

    /// Load `Decodable` type from a registry key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// extern crate rustc_serialize;
    /// extern crate winreg;
    /// # fn main() {
    /// use winreg::RegKey;
    /// use winreg::enums::*;
    /// use rustc_serialize::Decodable;
    ///
    /// #[derive(RustcDecodable)]
    /// struct Rectangle{
    ///     x: u32,
    ///     y: u32,
    ///     w: u32,
    ///     h: u32,
    /// }
    ///
    /// #[derive(RustcDecodable)]
    /// struct Settings{
    ///     current_dir: String,
    ///     window_pos: Rectangle,
    ///     show_in_tray: bool,
    /// }
    ///
    /// let s_key = RegKey::predef(HKEY_CURRENT_USER)
    ///     .open_subkey("Software\\MyProduct\\Settings").unwrap();
    /// let s: Settings = s_key.decode().unwrap();
    /// # }
    /// ```
    pub fn decode<T: rustc_serialize::Decodable>(&self)
        -> serialization::DecodeResult<T>
    {
        let mut decoder = try!(
            serialization::Decoder::from_key(&self)
        );
        T::decode(&mut decoder)
    }

    fn close_(&mut self) -> io::Result<()> {
        // don't try to close predefined keys
        if self.hkey >= winapi::HKEY_CLASSES_ROOT { return Ok(()) };
        match unsafe {
            advapi32::RegCloseKey(self.hkey) as DWORD
        } {
            0 => Ok(()),
            err => werr!(err)
        }
    }
}

impl Drop for RegKey {
    fn drop(&mut self) {
        self.close_().unwrap();
    }
}

/// Iterator over subkeys names
pub struct EnumKeys<'key> {
    key: &'key RegKey,
    index: DWORD,
}

impl<'key> Iterator for EnumKeys<'key> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        let mut name_len = 2048;
        let mut name = [0 as WCHAR; 2048];
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
            ) as DWORD
        } {
            0 => {
                self.index += 1;
                Some(match String::from_utf16(&name[..name_len as usize]) {
                    Ok(s) => Ok(s),
                    Err(_) => werr!(winerror::ERROR_INVALID_BLOCK)
                })
            },
            winerror::ERROR_NO_MORE_ITEMS => None,
            err => {
                Some(werr!(err))
            }
        }
    }
}

/// Iterator over values
pub struct EnumValues<'key> {
    key: &'key RegKey,
    index: DWORD,
}

impl<'key> Iterator for EnumValues<'key> {
    type Item = io::Result<(String, RegValue)>;

    fn next(&mut self) -> Option<io::Result<(String, RegValue)>> {
        let mut name_len = 2048;
        let mut name = [0 as WCHAR; 2048];

        let mut buf_len: DWORD = 2048;
        let mut buf_type: DWORD = 0;
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
            ) as DWORD
        } {
            0 => {
                self.index += 1;
                let name = String::from_utf16(&name[..name_len as usize]).unwrap();
                unsafe{ buf.set_len(buf_len as usize); }
                // minimal check before transmute to RegType
                if buf_type > winapi::REG_QWORD {
                    return Some(werr!(winerror::ERROR_BAD_FILE_TYPE));
                }
                let t: RegType = unsafe{ transmute(buf_type as u8) };
                let value = RegValue{ bytes: buf, vtype: t };
                Some(Ok((name, value)))
            },
            winerror::ERROR_NO_MORE_ITEMS => None,
            err => {
                Some(werr!(err))
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


#[cfg(test)]
mod test {
    extern crate rand;
    use super::*;
    use super::enums::*;
    use super::types::*;
    use std::collections::HashMap;
    use rustc_serialize::{Encodable,Decodable};

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
    fn test_long_value() {
        with_key!(key, "LongValue" => {
            let name = "RustLongVal";
            let val1 = RegValue { vtype: REG_BINARY, bytes: (0..6000).map(|_| rand::random::<u8>()).collect() };
            key.set_raw_value(name, &val1).unwrap();
            let val2 = key.get_raw_value(name).unwrap();
            assert_eq!(val1, val2);
        });
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
                vals3.push(String::from_reg_value(&val).unwrap());
            }
            assert_eq!(vals1, vals2);
            assert_eq!(vals1, vals3);
        });
    }

    #[test]
    fn test_serialization() {
        #[derive(Debug,RustcEncodable,RustcDecodable,PartialEq)]
        struct Rectangle{
            x: u32,
            y: u32,
            w: u32,
            h: u32,
        }

        #[derive(Debug,RustcEncodable,RustcDecodable,PartialEq)]
        struct Test {
            t_bool: bool,
            t_u8: u8,
            t_u16: u16,
            t_u32: u32,
            t_u64: u64,
            t_usize: usize,
            t_struct: Rectangle,
            t_string: String,
            t_i8: i8,
            t_i16: i16,
            t_i32: i32,
            t_i64: i64,
            t_isize: isize,
            // t_f64: f64,
            // t_f32: f32,
        }

        let v1 = Test{
            t_bool: false,
            t_u8: 127,
            t_u16: 32768,
            t_u32: 123456789,
            t_u64: 123456789101112,
            t_usize: 123456789101112,
            t_struct: Rectangle{ x: 55, y: 77, w: 500, h: 300 },
            t_string: "Test123 \n$%^&|+-*/\\()".to_string(),
            t_i8: -123,
            t_i16: -2049,
            t_i32: 20100,
            t_i64: -12345678910,
            t_isize: -1234567890,
            // t_f64: -0.01,
            // t_f32: 3.14,
        };

        with_key!(key, "Serialization" => {
            key.encode(&v1).unwrap();
            let v2: Test = key.decode().unwrap();
            assert_eq!(v1, v2);
        });
    }
}
