#![feature(collections)]
extern crate winreg;
use std::path::Path;
use winreg::types::*;

fn main() {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey(Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion"), KEY_READ).unwrap();
    let program_files: String = cur_ver.get_value(Path::new("ProgramFilesDir")).unwrap();
    let common_files: String = cur_ver.get_value(Path::new("CommonFilesDir")).unwrap();
    println!("ProgramFiles = {}\nCommonFiles = {}", program_files, common_files);

    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let test_key = hkcu.create_subkey(Path::new("Software\\WinregRsExample1\\many\\sub\\keys"), KEY_ALL_ACCESS).unwrap();
    test_key.set_value(Path::new("Test123"), &String::from_str("written by Rust")).unwrap();
    hkcu.delete_subkey_all(Path::new("Software\\WinregRsExample1")).unwrap();

    println!("Trying to open nonexisting key...");
    println!("{:?}", hkcu.open_subkey(Path::new("Software\\WinregRsExample1\\MissingKey"), KEY_READ).unwrap_err());
}
