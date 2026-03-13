// Copyright 2026, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use crate::{RegKey, RegValue};
use std::io;
use winapi::shared::minwindef::DWORD;

/// Iterator over values
pub struct EnumValues<'key> {
    pub(crate) key: &'key RegKey,
    pub(crate) index: DWORD,
}

impl<'a> Iterator for EnumValues<'a> {
    type Item = io::Result<(String, RegValue<'static>)>;

    fn next(&mut self) -> Option<io::Result<(String, RegValue<'static>)>> {
        match self.key.enum_value(self.index) {
            v @ Some(_) => {
                self.index += 1;
                v
            }
            e @ None => e,
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n as DWORD;
        self.next()
    }
}
