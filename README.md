winreg
======

Rust bindings to MS Windows Registry API. Work in progress.

Currently it can:
* open registry key
* create registry key
* delete registry key
* delete registry key recursively
* read `String` from `REG_SZ`, `REG_EXPAND_SZ` or `REG_MULTI_SZ` value
* read `u32` from `REG_DWORD` value
* read `u64` from `REG_QWORD` value
* write `String` and `&str` into `REG_SZ` value
* write `u32` into `REG_DWORD` value
* write `u64` into `REG_QWORD` value

## Usage

```rust
extern crate winreg;
use std::path::Path;
use winreg::RegKey;
use winreg::types::*;

fn main() {
    println!("Reading some system info...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion",
        KEY_READ).unwrap();
    let pf: String = cur_ver.get_value("ProgramFilesDir").unwrap();
    let dp: String = cur_ver.get_value("DevicePath").unwrap();
    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);

    println!("And now lets write something...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("WinregRsExample1");
    let key = hkcu.create_subkey(&path).unwrap();

    key.set_value("TestSZ", &"written by Rust").unwrap();
    let sz_val: String = key.get_value("TestSZ").unwrap();
    key.delete_value("TestSZ").unwrap();
    println!("TestSZ = {}", sz_val);

    key.set_value("TestDWORD", &1234567890u32).unwrap();
    let dword_val: u32 = key.get_value("TestDWORD").unwrap();
    println!("TestDWORD = {}", dword_val);

    key.set_value("TestQWORD", &1234567891011121314u64).unwrap();
    let qword_val: u64 = key.get_value("TestQWORD").unwrap();
    println!("TestQWORD = {}", qword_val);

    key.create_subkey("sub\\key").unwrap();
    hkcu.delete_subkey_all(&path).unwrap();

    println!("Trying to open nonexisting key...");
    println!("{:?}", hkcu.open_subkey(&path).unwrap_err());
}
```
