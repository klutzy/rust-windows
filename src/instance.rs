// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

use kernel32;
use winapi::{HINSTANCE};

#[derive(Copy)]
pub struct Instance {
    pub instance: HINSTANCE
}

impl Instance {
    pub fn main_instance() -> Instance {
        Instance {
            instance: unsafe { kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE },
        }
    }
}
