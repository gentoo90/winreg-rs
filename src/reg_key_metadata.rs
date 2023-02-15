// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use winapi::shared::minwindef::{DWORD, FILETIME};
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::timezoneapi::FileTimeToSystemTime;

/// Metadata returned by `RegKey::query_info`
#[derive(Debug, Default)]
pub struct RegKeyMetadata {
    // pub Class: winapi::LPWSTR,
    // pub ClassLen: DWORD,
    pub sub_keys: DWORD,
    pub max_sub_key_len: DWORD,
    pub max_class_len: DWORD,
    pub values: DWORD,
    pub max_value_name_len: DWORD,
    pub max_value_len: DWORD,
    // pub SecurityDescriptor: DWORD,
    pub last_write_time: FILETIME,
}

impl RegKeyMetadata {
    /// Returns `last_write_time` field as `winapi::um::minwinbase::SYSTEMTIME`
    pub fn get_last_write_time_system(&self) -> SYSTEMTIME {
        let mut st: SYSTEMTIME = unsafe { ::std::mem::zeroed() };
        unsafe {
            FileTimeToSystemTime(&self.last_write_time, &mut st);
        }
        st
    }

    /// Returns `last_write_time` field as `chrono::NaiveDateTime`.
    /// Part of `chrono` feature.
    #[cfg(feature = "chrono")]
    pub fn get_last_write_time_chrono(&self) -> chrono::NaiveDateTime {
        let st = self.get_last_write_time_system();

        chrono::NaiveDate::from_ymd(st.wYear.into(), st.wMonth.into(), st.wDay.into()).and_hms(
            st.wHour.into(),
            st.wMinute.into(),
            st.wSecond.into(),
        )
    }
}
