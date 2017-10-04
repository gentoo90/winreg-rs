// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#[macro_use]
extern crate serde_derive;
extern crate winreg;

use std::collections::HashMap;
use winreg::enums::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Server {
    addr: String,
    port: u16
}

impl Server {
    fn new(addr: &str, port: u16) -> Server {
        Server {
            addr: addr.to_owned(),
            port
        }
    }
}

fn main() {
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.create_subkey("Software\\RustEncode").unwrap();

    let mut v1 = HashMap::new();
    v1.insert("google", Server::new("www.google.com", 80));
    v1.insert("github", Server::new("github.com", 80));

    key.encode(&v1).unwrap();

    let v2: HashMap<String, Server> = key.decode().unwrap();
    println!("Decoded {:?}", v2);

    // println!("Equal to encoded: {:?}", v1 == v2);
}
