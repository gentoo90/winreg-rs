// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use std::fmt;
use serde::de::*;
use winapi::DWORD;
use super::{DecoderError, DecodeResult, Decoder};
// use super::super::EnumKeys;

impl Error for DecoderError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DecoderError::DeserializerError(format!("{}", msg))
    }
}

impl<'a> Deserializer for &'a mut Decoder {
    type Error = DecoderError;
    fn deserialize<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize")
    }

    fn deserialize_bool<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_bool")
    }

    fn deserialize_u8<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_u8")
    }

    fn deserialize_u16<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_u16")
    }

    fn deserialize_u32<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_u32")
    }

    fn deserialize_u64<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_u64")
    }

    fn deserialize_i8<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_i8")
    }

    fn deserialize_i16<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_i16")
    }

    fn deserialize_i32<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_i32")
    }

    fn deserialize_i64<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_i64")
    }

    fn deserialize_f32<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_f32")
    }

    fn deserialize_f64<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_f64")
    }

    fn deserialize_char<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_char")
    }

    fn deserialize_str<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_str")
    }

    fn deserialize_string<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_string")
    }

    fn deserialize_bytes<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_bytes")
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_byte_buf")
    }

    fn deserialize_option<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_option")
    }

    fn deserialize_unit<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_unit")
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_unit_struct")
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_newtype_struct")
    }

    fn deserialize_seq<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_seq")
    }

    fn deserialize_seq_fixed_size<V>(self, len: usize, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_seq_fixed_size")
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_tuple")
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_tuple_struct")
    }

    fn deserialize_map<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_map")
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        visitor.visit_map(RegMapVisitor::new(self))
        // for f in fields {
        //     println!("{}", f);
        //     self.deserialize_struct_field(visitor);
        // }
        // no_impl!("deserialize_struct")
        // Ok(())
    }

    fn deserialize_struct_field<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        // println!("STRUCT FIELD VISITED");
        // let res = visitor.visit_
        no_impl!("deserialize_struct_field")
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_enum")
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> DecodeResult<V::Value> where V: Visitor {
        no_impl!("deserialize_ignored_any")
    }
}

enum RegMapVisitorState {
    EnumeratingKeys,
    EnumeratingValues,
}

struct RegMapVisitor<'a> {
    dec: &'a mut Decoder,
    state: RegMapVisitorState,
    index: DWORD,
}

impl<'a> RegMapVisitor<'a> {
    fn new(dec: &mut Decoder) -> RegMapVisitor {
        // let iter = dec.key.enum_keys();
        RegMapVisitor {
            dec: dec,
            state: RegMapVisitorState::EnumeratingKeys,
            index: 0,
        }
    }
}

impl<'a> MapVisitor for RegMapVisitor<'a> {
    type Error = DecoderError;
    fn visit_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where K: DeserializeSeed
    {
        // println!("KEY SEED VISITED");
        match self.state {
            RegMapVisitorState::EnumeratingKeys => {
                match self.dec.key.enum_key(self.index) {
                    Some(res) => {
                        self.dec.f_name = Some(res?);
                        self.index += 1;
                        seed.deserialize(&mut *self.dec).map(Some)
                        // match res {
                        //     Ok(name) => { self.dec.f_name = Some(name) },
                        //     Err(err) => {}
                        // }
                    }
                    None => {
                        self.index = 0;
                        self.state = RegMapVisitorState::EnumeratingValues;
                        Ok(None)
                    }
                }
            }
            RegMapVisitorState::EnumeratingValues => {
                let next_value = self.dec.key.enum_value(self.index);
                match next_value {
                    Some(res) => {
                        self.dec.f_name = Some(res?.0);
                        self.index += 1;
                        seed.deserialize(&mut *self.dec).map(Some)
                    }
                    None => Ok(None),
                }
            }
        }
        // seed.deserialize(&mut *self.dec).map(Some)
        // no_impl!("MapVisitor::visit_key_seed")
    }

    fn visit_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where V: DeserializeSeed
    {
        // println!("VALUE SEED VISITED");
        // seed.deserialize(&mut *self.dec)
        no_impl!("MapVisitor::visit_value_seed")
    }
}
