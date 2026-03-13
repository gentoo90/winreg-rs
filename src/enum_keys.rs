// Copyright 2026, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use crate::RegKey;
use std::io;
use winapi::shared::minwindef::DWORD;

/// Iterator over subkeys names
pub struct EnumKeys<'key> {
    pub(crate) key: &'key RegKey,
    pub(crate) index: DWORD,
}

impl Iterator for EnumKeys<'_> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        match self.key.enum_key(self.index) {
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
