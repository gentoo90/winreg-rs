// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use std::mem;
use rustc_serialize;
use super::{Encoder, EncoderError, EncodeResult, ENCODER_SAM};
use super::EncoderState::*;

impl rustc_serialize::Encoder for Encoder {
    type Error = EncoderError;

    fn emit_nil(&mut self) -> EncodeResult<()> {
        no_impl!("nil")
    }

    fn emit_usize(&mut self, v: usize) -> EncodeResult<()> {
        self.emit_u64(v as u64)
    }

    fn emit_u64(&mut self, v: u64) -> EncodeResult<()> {
        emit_value!(self, v)
    }

    fn emit_u32(&mut self, v: u32) -> EncodeResult<()> {
        emit_value!(self, v)
    }

    fn emit_u16(&mut self, v: u16) -> EncodeResult<()> {
        self.emit_u32(v as u32)
    }

    fn emit_u8(&mut self, v: u8) -> EncodeResult<()> {
        self.emit_u32(v as u32)
    }

    fn emit_isize(&mut self, v: isize) -> EncodeResult<()> {
        self.emit_i64(v as i64)
    }

    fn emit_i64(&mut self, v: i64) -> EncodeResult<()> {
        let s = v.to_string();
        emit_value!(self, s)
    }

    fn emit_i32(&mut self, v: i32) -> EncodeResult<()> {
        self.emit_i64(v as i64)
    }

    fn emit_i16(&mut self, v: i16) -> EncodeResult<()> {
        self.emit_i64(v as i64)
    }

    fn emit_i8(&mut self, v: i8) -> EncodeResult<()> {
        self.emit_i64(v as i64)
    }

    fn emit_bool(&mut self, v: bool) -> EncodeResult<()> {
        self.emit_u32(v as u32)
    }

    fn emit_f64(&mut self, v: f64) -> EncodeResult<()> {
        let s = v.to_string();
        emit_value!(self, s)
    }

    fn emit_f32(&mut self, v: f32) -> EncodeResult<()> {
        let s = v.to_string();
        emit_value!(self, s)
    }

    fn emit_char(&mut self, _v: char) -> EncodeResult<()> {
        no_impl!("char")
    }

    fn emit_str(&mut self, v: &str) -> EncodeResult<()> {
        emit_value!(self, v)
    }

    fn emit_enum<F>(&mut self, _name: &str, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("enum")
    }

    fn emit_enum_variant<F>(&mut self,
                            _name: &str,
                            _id: usize,
                            _cnt: usize,
                            _f: F)
                            -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("enum_variant")
    }

    fn emit_enum_variant_arg<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("enum_variant_arg")
    }

    fn emit_enum_struct_variant<F>(&mut self,
                                   _name: &str,
                                   _id: usize,
                                   _cnt: usize,
                                   _f: F)
                                   -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("enum_struct_variant")
    }

    fn emit_enum_struct_variant_field<F>(&mut self,
                                         _name: &str,
                                         _: usize,
                                         _f: F)
                                         -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("enum_struct_variant_field")
    }

    fn emit_struct<F>(&mut self, _name: &str, _len: usize, f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        match mem::replace(&mut self.state, Start) {
            Start => {
                // root structure
                f(self)
            }
            NextKey(ref s) => {
                // nested structure
                match self.keys[self.keys.len() - 1]
                    .create_subkey_transacted_with_flags(&s, &self.tr, ENCODER_SAM) {
                    Ok(subkey) => {
                        self.keys.push(subkey);
                        let res = f(self);
                        self.keys.pop();
                        res
                    }
                    Err(err) => Err(EncoderError::IoError(err)),
                }
            }
        }
    }

    fn emit_struct_field<F>(&mut self, f_name: &str, _f_idx: usize, f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        self.state = NextKey(f_name.to_owned());
        f(self)
    }

    fn emit_tuple<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("tuple")
    }

    fn emit_tuple_arg<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("tuple_arg")
    }

    fn emit_tuple_struct<F>(&mut self, _: &str, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("tuple_struct")
    }

    fn emit_tuple_struct_arg<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("tuple_struct_arg")
    }

    fn emit_option<F>(&mut self, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("Option")
    }

    fn emit_option_none(&mut self) -> EncodeResult<()> {
        no_impl!("Option::None")
    }

    fn emit_option_some<F>(&mut self, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("Option::Some")
    }

    fn emit_seq<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("seq")
    }

    fn emit_seq_elt<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("seq_elt")
    }

    fn emit_map<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("map")
    }

    fn emit_map_elt_key<F>(&mut self, _: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("map_elt_key")
    }

    fn emit_map_elt_val<F>(&mut self, _idx: usize, _f: F) -> EncodeResult<()>
        where F: FnOnce(&mut Self) -> EncodeResult<()>
    {
        no_impl!("map_elt_val")
    }
}

