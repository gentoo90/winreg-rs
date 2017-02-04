// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use std::mem;
use rustc_serialize;
use super::{Decoder, DecoderError, DecodeResult, DECODER_SAM};

impl rustc_serialize::Decoder for Decoder {
    type Error = DecoderError;
    fn read_nil(&mut self) -> DecodeResult<()> {
        no_impl!("nil")
    }

    fn read_usize(&mut self) -> DecodeResult<usize> {
        self.read_u64().map(|v| v as usize)
    }

    fn read_u64(&mut self) -> DecodeResult<u64> {
        read_value!(self)
    }

    fn read_u32(&mut self) -> DecodeResult<u32> {
        read_value!(self)
    }

    fn read_u16(&mut self) -> DecodeResult<u16> {
        self.read_u32().map(|v| v as u16)
    }

    fn read_u8(&mut self) -> DecodeResult<u8> {
        self.read_u32().map(|v| v as u8)
    }

    fn read_isize(&mut self) -> DecodeResult<isize> {
        self.read_i64().map(|v| v as isize)
    }

    fn read_i64(&mut self) -> DecodeResult<i64> {
        parse_string!(self)
    }

    fn read_i32(&mut self) -> DecodeResult<i32> {
        parse_string!(self)
    }

    fn read_i16(&mut self) -> DecodeResult<i16> {
        parse_string!(self)
    }

    fn read_i8(&mut self) -> DecodeResult<i8> {
        parse_string!(self)
    }

    fn read_bool(&mut self) -> DecodeResult<bool> {
        self.read_u32().map(|v| v > 0)
    }

    fn read_f64(&mut self) -> DecodeResult<f64> {
        parse_string!(self)
    }

    fn read_f32(&mut self) -> DecodeResult<f32> {
        parse_string!(self)
    }

    fn read_char(&mut self) -> DecodeResult<char> {
        no_impl!("char")
    }

    fn read_str(&mut self) -> DecodeResult<String> {
        read_value!(self)
    }

    fn read_enum<T, F>(&mut self, _name: &str, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("enum")
    }

    fn read_enum_variant<T, F>(&mut self, _names: &[&str], _f: F) -> DecodeResult<T>
        where F: FnMut(&mut Self, usize) -> DecodeResult<T>
    {
        no_impl!("enum_variant")
    }

    fn read_enum_variant_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("enum_variant_arg")
    }

    fn read_enum_struct_variant<T, F>(&mut self, _names: &[&str], _f: F) -> DecodeResult<T>
        where F: FnMut(&mut Self, usize) -> DecodeResult<T>
    {
        no_impl!("enum_struct_variant")
    }

    fn read_enum_struct_variant_field<T, F>(&mut self,
                                            _f_name: &str,
                                            _f_idx: usize,
                                            _f: F)
                                            -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("enum_struct_variant_field")
    }

    fn read_struct<T, F>(&mut self, _s_name: &str, _len: usize, f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        match mem::replace(&mut self.f_name, None) {
            None => {
                // root structure
                f(self)
            }
            Some(ref s) => {
                // nested structure
                match self.key.open_subkey_with_flags(&s, DECODER_SAM) {
                    Ok(subkey) => {
                        let mut nested = Decoder::new(subkey);
                        f(&mut nested)
                    }
                    Err(err) => Err(DecoderError::IoError(err)),
                }
            }
        }
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, _f_idx: usize, f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        self.f_name = Some(f_name.to_owned());
        f(self)
    }

    fn read_tuple<T, F>(&mut self, _len: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("tuple")
    }

    fn read_tuple_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("tuple_arg")
    }

    fn read_tuple_struct<T, F>(&mut self, _s_name: &str, _len: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("tuple_struct")
    }

    fn read_tuple_struct_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("tuple_struct_arg")
    }

    fn read_option<T, F>(&mut self, _f: F) -> DecodeResult<T>
        where F: FnMut(&mut Self, bool) -> DecodeResult<T>
    {
        no_impl!("option")
    }

    fn read_seq<T, F>(&mut self, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self, usize) -> DecodeResult<T>
    {
        no_impl!("seq")
    }

    fn read_seq_elt<T, F>(&mut self, _idx: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("seq_elt")
    }

    fn read_map<T, F>(&mut self, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self, usize) -> DecodeResult<T>
    {
        no_impl!("map")
    }

    fn read_map_elt_key<T, F>(&mut self, _idx: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("map_elt_key")
    }

    fn read_map_elt_val<T, F>(&mut self, _idx: usize, _f: F) -> DecodeResult<T>
        where F: FnOnce(&mut Self) -> DecodeResult<T>
    {
        no_impl!("map_elt_val")
    }

    fn error(&mut self, err: &str) -> Self::Error {
        DecoderError::ParseError(err.to_owned())
    }
}

