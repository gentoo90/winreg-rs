winreg [![Crates.io](https://img.shields.io/crates/v/winreg.svg)](https://crates.io/crates/winreg)
======

Rust bindings to MS Windows Registry API. Work in progress.

Currently it can:
* open registry key
* create key
* query key metadata
* delete key
* delete key recursively
* read `String` from `REG_SZ`, `REG_EXPAND_SZ` or `REG_MULTI_SZ` value
* read `u32` from `REG_DWORD` value
* read `u64` from `REG_QWORD` value
* read raw value of any type to `RegValue` structure
* write `String` and `&str` into `REG_SZ` value
* write `u32` into `REG_DWORD` value
* write `u64` into `REG_QWORD` value
* write raw value of any type from `RegValue` structure
* iterate through subkey names
* iterate through values

## Usage

Basic usage:
```rust
extern crate winreg;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

fn main() {
    println!("Reading some system info...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion",
        KEY_READ).unwrap();
    let pf: String = cur_ver.get_value("ProgramFilesDir").unwrap();
    let dp: String = cur_ver.get_value("DevicePath").unwrap();
    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);
    let info = cur_ver.query_info().unwrap();
    println!("info = {:?}", info);

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

Iterators:
```rust
extern crate winreg;
use winreg::RegKey;
use winreg::enums::*;

fn main() {
    println!("File extensions, registered in system:");
    for i in RegKey::predef(HKEY_CLASSES_ROOT)
        .enum_keys().map(|x| x.unwrap())
        .filter(|x| x.starts_with("."))
    {
        println!("{}", i);
    }

    let system = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags("HARDWARE\\DESCRIPTION\\System", KEY_READ)
        .unwrap();
    for (name, value) in system.enum_values().map(|x| x.unwrap()) {
        println!("{} = {:?}", name, value);
    }
}
```
