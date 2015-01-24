// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::wchar::ToCU16Str;
use super::window::Window;

use user32;

pub trait DialogUtil {
    fn message_box(&self, msg: &str, title: &str);
}

impl DialogUtil for Window {
    fn message_box(&self, msg: &str, title: &str) {
        let msg_u = msg.to_c_u16();
        let title_u = title.to_c_u16();
        unsafe {
            user32::MessageBoxW(self.wnd, msg_u.as_ptr(), title_u.as_ptr(), 0u32);
        }
    }
}
