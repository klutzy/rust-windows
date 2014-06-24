#![feature(globs, phase, macro_rules)]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_id = "rust-windows"]

#[phase(plugin, link)] extern crate log;

extern crate libc;
extern crate collections;
extern crate debug;

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
        hwnd: ptr::mut_null(),
        message: 0 as UINT,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT { x: 0 as LONG, y: 0 as LONG },
    };
    loop {
        let ret = unsafe {
            ll::all::GetMessageW(&msg as *MSG, ptr::mut_null(),
                    0 as UINT, 0 as UINT)
        };

        if ret == 0 {
            let exit_code = msg.wParam;
            return exit_code as uint;
        }
        else {
            unsafe {
                ll::all::TranslateMessage(&msg as *MSG);
                ll::all::DispatchMessageW(&msg as *MSG);
            }
        }
    }
}
