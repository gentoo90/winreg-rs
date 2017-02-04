// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#![cfg(feature = "serialization-rustc")]
use std::io;
use super::{RegKey};
use super::enums::*;
use super::winapi;

macro_rules! read_value{
    ($s:ident) => (
        match mem::replace(&mut $s.f_name, None) {
            Some(ref s) => {
                $s.key.get_value(s)
                    .map_err(DecoderError::IoError)
            },
            None => Err(DecoderError::NoFieldName)
        }
    )
}

macro_rules! parse_string{
    ($s:ident) => ({
        let s: String = try!(read_value!($s));
        s.parse().map_err(|e| DecoderError::ParseError(format!("{:?}", e)))
    })
}

macro_rules! no_impl {
    ($e:expr) => (
        Err(DecoderError::DecodeNotImplemented($e.to_owned()))
    )
}

mod serialization_rustc;

#[derive(Debug)]
pub enum DecoderError{
    DecodeNotImplemented(String),
    IoError(io::Error),
    ParseError(String),
    NoFieldName,
}

pub type DecodeResult<T> = Result<T, DecoderError>;

#[derive(Debug)]
pub struct Decoder {
    key: RegKey,
    f_name: Option<String>,
}

const DECODER_SAM: winapi::DWORD = KEY_QUERY_VALUE;

impl Decoder {
    pub fn from_key(key: &RegKey) -> DecodeResult<Decoder> {
        key.open_subkey_with_flags("", DECODER_SAM)
            .map(Decoder::new)
            .map_err(DecoderError::IoError)
    }

    fn new(key: RegKey) -> Decoder {
        Decoder{
            key: key,
            f_name: None,
        }
    }
}
