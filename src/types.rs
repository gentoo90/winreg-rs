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

/// A trait for types that can be loaded from registry values.
pub trait FromReg {
    fn convert_from_bytes(buf: Vec<u16>) -> Self;
}

impl FromReg for String {
    fn convert_from_bytes(buf: Vec<u16>) -> String {
        String::from_utf16(&buf).unwrap()
    }
}
