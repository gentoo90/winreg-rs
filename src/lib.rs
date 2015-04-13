//! Crate for accessing MS Windows registry
#![feature(std_misc)]
#![cfg_attr(test, feature(collections))]
extern crate winapi;
extern crate kernel32;
extern crate advapi32;
use std::path::Path;
use std::ptr;
use std::fmt;
use std::ffi::AsOsStr;
use std::os::windows::ffi::OsStrExt;
use types::{FromReg, ToReg};

pub mod types;

pub struct RegError {
    err: winapi::LONG,
}

impl fmt::Debug for RegError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegError {{ err: {:?}, message: {:?} }}",
                   self.err, error_string(self.err))
    }
}

pub type RegResult<T> = std::result::Result<T, RegError>;

#[derive(Debug)]
pub struct RegKey {
    hkey: winapi::HKEY,
}

impl RegKey {
    pub fn predef(hkey: winapi::HKEY) -> RegKey {
        RegKey{ hkey: hkey }
    }

    pub fn open_subkey(&self, path: &Path, perms: winapi::REGSAM) -> RegResult<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: winapi::HKEY = ptr::null_mut();
        match unsafe{
            advapi32::RegOpenKeyExW(
                self.hkey,
                c_path.as_ptr(),
                0,
                perms,
                &mut new_hkey,
            )
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => Err(RegError{ err: err })
        }
    }

    /// Will also create all missing parent keys. Will open key if it already exists.
    pub fn create_subkey(&self, path: &Path, perms: winapi::REGSAM) -> RegResult<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: winapi::HKEY = ptr::null_mut();
        let mut disp: winapi::DWORD = 0;
        match unsafe{
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
            )
        } {
            0 => Ok(RegKey{ hkey: new_hkey }),
            err => Err(RegError{ err: err })
        }
    }

    pub fn delete_subkey(&self, path: &Path) -> RegResult<()> {
        let c_path = to_utf16(path);
        match unsafe{
            advapi32::RegDeleteKeyW(
                self.hkey,
                c_path.as_ptr(),
            )
        } {
            0 => Ok(()),
            err => Err(RegError{ err: err })
        }
    }

    pub fn delete_subkey_all(&self, path: &Path) -> RegResult<()> {
        let c_path = to_utf16(path);
        match unsafe{
            advapi32::RegDeleteTreeW(
                self.hkey,
                c_path.as_ptr(),
            )
        } {
            0 => Ok(()),
            err => Err(RegError{ err: err })
        }
    }

    pub fn get_value<T: FromReg>(&self, name: &Path) -> RegResult<T> {
        let c_name = to_utf16(name);
        let mut buf_len: winapi::DWORD = winapi::MAX_PATH as winapi::DWORD;
        let mut buf: Vec<u16> = Vec::with_capacity(buf_len as usize);
        match unsafe{
            advapi32::RegQueryValueExW(
                self.hkey,
                c_name.as_ptr() as *const u16,
                ptr::null_mut(),
                ptr::null_mut(),
                buf.as_mut_ptr() as winapi::LPBYTE,
                &mut buf_len
            )
        } {
            0 => {
                // set length to wchars count - 1 (trailing \0)
                unsafe{ buf.set_len(((buf_len >> 1) - 1) as usize); }
                Ok(FromReg::convert_from_bytes(buf))
            },
            err => Err(RegError{ err: err })
        }
    }

    pub fn set_value<T: ToReg>(&self, name: &Path, value: &T) -> RegResult<()> {
        let c_name = to_utf16(name);
        let c_value = value.convert_to_bytes();
        let v_type = value.get_val_type();
        match unsafe{
            advapi32::RegSetValueExW(
                self.hkey,
                c_name.as_ptr(),
                0,
                v_type,
                c_value.as_ptr() as *const winapi::BYTE,
                (c_value.len()*2) as u32
            )
        } {
            0 => {
                Ok(())
            },
            err => Err(RegError{ err: err })
        }
    }

    fn close_(&mut self) -> RegResult<()> {
        match unsafe{
            advapi32::RegCloseKey(self.hkey)
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

fn to_utf16<T: AsOsStr>(s: T) -> Vec<u16> {
    s.as_os_str().encode_wide().chain(Some(0).into_iter()).collect()
}

// copycat of rust/src/libstd/sys/windows/os.rs::error_string
// `use std::sys::os::error_string` leads to
// error: function `error_string` is private.
// Get a detailed string description for the given error number
fn error_string(errnum: winapi::LONG) -> String {
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
            Ok(msg) => msg,
            Err(..) => format!("OS Error {} (FormatMessageW() returned \
                                invalid UTF-16)", errnum),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::types::*;
    use std::path::Path;

    #[test]
    fn test_key_open() {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let win = hklm.open_subkey(Path::new("Software\\Microsoft\\Windows"), KEY_READ);
        assert!(win.is_ok());
        assert!(win.unwrap().open_subkey(Path::new("CurrentVersion\\"), KEY_READ).is_ok());
        assert!(hklm.open_subkey(Path::new("i\\just\\hope\\nobody\\created\\that\\key"), KEY_READ).is_err());
    }

    #[test]
    fn test_string_value() {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new("Software\\WinregRsTestKey");
        let name = Path::new("WinregRsTestVal");
        let val1 = String::from_str("Test123 $%^&|+-*/\\()");

        let sw = hkcu.create_subkey(path, KEY_ALL_ACCESS).unwrap();
        assert!(sw.set_value(name, &val1).is_ok());
        let val2: String = sw.get_value(name).unwrap();
        assert_eq!(val1, val2);
        assert!(hkcu.delete_subkey(path).is_ok());
    }

    #[test]
    fn test_delete_subkey_all() {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new("Software\\WinregRsTestKey2");
        {
            hkcu.create_subkey(&path.join("with\\sub\\keys"),
                               KEY_READ).unwrap();
        }
        assert!(hkcu.delete_subkey_all(path).is_ok());
    }
}
