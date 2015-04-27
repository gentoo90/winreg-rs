// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
extern crate winapi;
macro_rules! winapi_enum{
    ($t:ident => [$($v:ident),*]) => (
        #[allow(non_camel_case_types)]
        #[derive(Debug,Clone,PartialEq)]
        pub enum $t {
            $( $v = winapi::$v as isize ),*
        }
    )
}

winapi_enum!(RegType => [
REG_NONE,
REG_SZ,
REG_EXPAND_SZ,
REG_BINARY,
REG_DWORD,
REG_DWORD_BIG_ENDIAN,
REG_LINK,
REG_MULTI_SZ,
REG_RESOURCE_LIST,
REG_FULL_RESOURCE_DESCRIPTOR,
REG_RESOURCE_REQUIREMENTS_LIST,
REG_QWORD
]);
pub use self::RegType::*;
