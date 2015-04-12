//! Crate for accessing MS Windows registry
extern crate winapi;
extern crate advapi32;
use std::path::Path;
use std::ptr;
use std::os::windows::ffi::OsStrExt;
use types::FromReg;

pub mod types;

#[derive(Debug)]
pub struct RegError {
    err: winapi::LONG,
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

    pub fn open(&self, path: &Path, perms: winapi::REGSAM) -> RegResult<RegKey> {
        let mut new_hkey: winapi::HKEY = ptr::null_mut();
        let c_path = to_utf16(path);
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
                unsafe{ buf.set_len((buf_len >> 1) as usize); }
                Ok(FromReg::convert_from_bytes(buf))
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

fn to_utf16(s: &Path) -> Vec<u16> {
    s.as_os_str().encode_wide().chain(Some(0).into_iter()).collect()
}

#[test]
fn test_key_open() {
    let hklm = RegKey::predef(winapi::HKEY_LOCAL_MACHINE);
    let win = hklm.open(Path::new("Software\\Microsoft\\Windows"), winapi::KEY_READ);
    assert!(win.is_ok());
    assert!(win.unwrap().open(Path::new("CurrentVersion\\"), winapi::KEY_READ).is_ok());
    assert!(hklm.open(Path::new("i\\just\\hope\\nobody\\created\\that\\key"), winapi::KEY_READ).is_err());
}
