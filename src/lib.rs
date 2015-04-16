//! Crate for accessing MS Windows registry
extern crate winapi;
extern crate kernel32;
extern crate advapi32;
use std::ptr;
use std::fmt;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use types::{FromReg, ToReg};

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
        let c_name = to_utf16(name);
        let mut buf_len: winapi::DWORD = 2048 as winapi::DWORD;
        let mut buf_type: winapi::DWORD = 0;
        let mut buf: Vec<u16> = Vec::with_capacity(buf_len as usize);
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
                unsafe{ buf.set_len((buf_len >> 1) as usize); }
                FromReg::convert_from_bytes(buf, buf_type)
            },
            err => Err(RegError{ err: err })
        }
    }

    /// Set the `Default` value if `name` is an empty string
    pub fn set_value<T: ToReg, P: AsRef<OsStr>>(&self, name: P, value: &T) -> RegResult<()> {
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

fn to_utf16<P: AsRef<OsStr>>(s: P) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0).into_iter()).collect()
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

    #[test]
    fn test_open_subkey() {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let win = hklm.open_subkey("Software\\Microsoft\\Windows");
        assert!(win.is_ok());
        assert!(win.unwrap().open_subkey_with_flags("CurrentVersion\\", KEY_READ).is_ok());
        assert!(hklm.open_subkey("i\\just\\hope\\nobody\\created\\that\\key").is_err());
    }

    fn create_test_key(path: &str) -> RegKey {
        let mut full_path = "Software\\WinRegRsTest".to_string();
        full_path.push_str(path);
        RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(full_path).unwrap()
    }

    fn delete_test_key(path: &str) {
        let mut full_path = "Software\\WinRegRsTest".to_string();
        full_path.push_str(path);
        RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey(full_path).unwrap();
    }

    #[test]
    fn test_create_delete_subkey() {
        let path = "CreateDeleteSubkey";
        create_test_key(path);
        delete_test_key(path);
    }

    #[test]
    fn test_delete_subkey_all() {
        let path = "DeleteSubkeyAll";
        let key = create_test_key(path);
        key.create_subkey_with_flags("with\\sub\\keys", KEY_READ).unwrap();
        assert!(RegKey::predef(HKEY_CURRENT_USER)
            .delete_subkey_all("Software\\WinRegRsTestDeleteSubkeyAll").is_ok());
    }

    #[test]
    fn test_string_value() {
        let path = "StringValue";
        let key = create_test_key(path);
        let name = "RustStringVal";
        let val1 = "Test123 \n$%^&|+-*/\\()".to_string();

        key.set_value(name, &val1).unwrap();
        let val2: String = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
        delete_test_key(path);
    }

    #[test]
    fn test_u32_value() {
        let path = "U32Value";
        let key = create_test_key(path);
        let name = "RustU32Val";
        let val1 = 1234567890u32;

        key.set_value(name, &val1).unwrap();
        let val2: u32 = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
        delete_test_key(path);
    }

    #[test]
    fn test_delete_value() {
        let path = "StringValue";
        let key = create_test_key(path);
        let name = "WinregRsTestVal";
        key.set_value(name, &"Qwerty123").unwrap();
        assert!(key.delete_value(name).is_ok());
        delete_test_key(path);
    }
}
