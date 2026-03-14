// Copyright 2026, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use crate::{RegKey, RegValue};
use std::io;
use winapi::shared::{minwindef::DWORD, winerror};

/// Iterator over values
pub struct EnumValues<'key> {
    pub(crate) key: &'key RegKey,
    pub(crate) index: DWORD,
}

impl<'a> Iterator for EnumValues<'a> {
    type Item = io::Result<(String, RegValue<'static>)>;

    fn next(&mut self) -> Option<io::Result<(String, RegValue<'static>)>> {
        match self.key.enum_value(self.index) {
            None => None,
            Some(Err(err)) => {
                self.index += 1;
                Some(Err(err))
            }
            Some(Ok((name_os_string, value))) => {
                self.index += 1;
                match name_os_string.into_string() {
                    Ok(name_string) => Some(Ok((name_string, value))),
                    Err(_) => Some(werr!(winerror::ERROR_INVALID_DATA)),
                }
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n as DWORD;
        self.next()
    }
}
