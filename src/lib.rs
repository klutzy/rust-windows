// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unstable)]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_name = "rust-windows"]

#[macro_use] extern crate log;
extern crate collections;
extern crate "gdi32-sys" as gdi32;
extern crate "kernel32-sys" as kernel32;
extern crate "user32-sys" as user32;
extern crate winapi;

use std::ptr;
use winapi::{DWORD, HWND, LONG, LPARAM, LPMSG, LRESULT, MSG, POINT, UINT, WPARAM};

#[macro_use] pub mod macros;
pub mod instance;
pub mod resource;
pub mod font;
pub mod wchar;
pub mod window;
pub mod gdi;
pub mod dialog;

pub fn get_last_error() -> DWORD {
    unsafe { kernel32::GetLastError() }
}

pub fn def_window_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    unsafe { user32::DefWindowProcW(hwnd, msg, w, l) }
}

pub fn main_window_loop() -> usize {
    let mut msg = MSG {
        hwnd: ptr::null_mut(),
        message: 0 as UINT,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT { x: 0 as LONG, y: 0 as LONG },
    };
    loop {
        let ret = unsafe {
            user32::GetMessageW(&mut msg as LPMSG, ptr::null_mut(),
                    0 as UINT, 0 as UINT)
        };

        if ret == 0 {
            let exit_code = msg.wParam;
            return exit_code as usize;
        }
        else {
            unsafe {
                user32::TranslateMessage(&msg as *const MSG);
                user32::DispatchMessageW(&msg as *const MSG);
            }
        }
    }
}
