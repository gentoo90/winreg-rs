// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
extern crate winreg;
use winreg::RegKey;
use winreg::types::*;

fn main() {
    println!("File extensions, registered in system:");
    for i in RegKey::predef(HKEY_CLASSES_ROOT)
        .enum_keys().map(|x| x.unwrap())
        .filter(|x| x.starts_with("."))
    {
        println!("{}", i);
    }
}
