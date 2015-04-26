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

    let metrics = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Control Panel\\Desktop\\WindowMetrics")
        .unwrap();
    for (name, value) in metrics.enum_values().map(|x| x.unwrap()) {
        println!("{} = {:?}", name, value);
    }
    let lol = metrics.get_raw_value("MenuHeight");
    println!("{:?}", lol);
}
