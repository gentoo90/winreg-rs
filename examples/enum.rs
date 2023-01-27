// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
extern crate winreg;
use std::io;
use winreg::enums::{HKEY_CLASSES_ROOT, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("File extensions, registered in system:");
    for i in RegKey::predef(HKEY_CLASSES_ROOT)
        .enum_keys()
        .map(std::result::Result::unwrap)
        .filter(|x| x.starts_with('.'))
    {
        println!("{i}");
    }

    let system = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("HARDWARE\\DESCRIPTION\\System")?;
    for (name, value) in system.enum_values().map(std::result::Result::unwrap) {
        println!("{name} = {value:?}");
    }

    Ok(())
}
