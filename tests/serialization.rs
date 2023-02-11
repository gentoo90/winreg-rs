// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#![cfg(feature = "serialization-serde")]
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

mod common;

#[test]
fn test_serialization() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Rectangle {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Test {
        t_bool: bool,
        t_u8: u8,
        t_u16: u16,
        t_u32: u32,
        t_u64: u64,
        t_usize: usize,
        t_struct: Rectangle,
        t_string: String,
        t_map: HashMap<String, HashMap<String, u32>>,
        t_i8: i8,
        t_i16: i16,
        t_i32: i32,
        t_i64: i64,
        t_isize: isize,
        t_f64: f64,
        t_f32: f32,
        t_char: char,
    }

    let mut k1 = HashMap::new();
    k1.insert("val1".to_owned(), 32);
    k1.insert("val2".to_owned(), 64);
    k1.insert("val3".to_owned(), 128);

    let mut k2 = HashMap::new();
    k2.insert("val1".to_owned(), 256);
    k2.insert("val2".to_owned(), 512);
    k2.insert("val3".to_owned(), 1024);

    let mut map = HashMap::new();
    map.insert("key1".to_owned(), k1);
    map.insert("key2".to_owned(), k2);

    let v1 = Test {
        t_bool: false,
        t_u8: 127,
        t_u16: 32768,
        t_u32: 123_456_789,
        t_u64: 123_456_789_101_112,
        t_usize: 1_234_567_891,
        t_struct: Rectangle {
            x: 55,
            y: 77,
            w: 500,
            h: 300,
        },
        t_map: map,
        t_string: "Test123 \n$%^&|+-*/\\()".to_owned(),
        t_i8: -123,
        t_i16: -2049,
        t_i32: 20100,
        t_i64: -12_345_678_910,
        t_isize: -1_234_567_890,
        t_f64: -0.01,
        t_f32: 3.15,
        t_char: 'a',
    };

    with_key!(key, "Serialization" => {
        key.encode(&v1).unwrap();
        let v2: Test = key.decode().unwrap();
        assert_eq!(v1, v2);
    });
}
