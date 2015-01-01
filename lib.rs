// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(globs, phase, macro_rules)]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_name = "rust-windows"]

#[phase(plugin, link)] extern crate log;

extern crate libc;
extern crate collections;

use std::ptr;

use libc::{LONG};
use ll::all::{MSG, POINT};
use ll::types::{HWND, LPARAM, UINT, WPARAM, LRESULT, DWORD};

pub mod ll;

pub mod macros;
pub mod instance;
pub mod resource;
pub mod font;
pub mod wchar;
pub mod window;
pub mod gdi;
pub mod dialog;

pub fn get_last_error() -> DWORD {
    unsafe { ll::all::GetLastError() }
}

pub fn def_window_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    unsafe { ll::all::DefWindowProcW(hwnd, msg, w, l) }
}

pub fn main_window_loop() -> uint {
    let msg = MSG {
        hwnd: ptr::null_mut(),
        message: 0 as UINT,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT { x: 0 as LONG, y: 0 as LONG },
    };
    loop {
        let ret = unsafe {
            ll::all::GetMessageW(&msg as *const MSG, ptr::null_mut(),
                    0 as UINT, 0 as UINT)
        };

        if ret == 0 {
            let exit_code = msg.wParam;
            return exit_code as uint;
        }
        else {
            unsafe {
                ll::all::TranslateMessage(&msg as *const MSG);
                ll::all::DispatchMessageW(&msg as *const MSG);
            }
        }
    }
}
