// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
extern crate rustc_serialize;
extern crate winreg;
use std::collections::HashMap;
use winreg::enums::*;

fn main() {
    let mut m = HashMap::new();

    m.insert("", 100u32);
    m.insert("key2", 200u32);
    m.insert("key3", 300u32);

    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.create_subkey("Software\\RustEncodeMap").unwrap();
    key.encode(&m).unwrap();
}
