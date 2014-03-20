#[feature(globs, phase)];
#[crate_type = "lib"];
#[crate_id = "rust-windows"];

#[phase(syntax, link)] extern crate log;

extern crate collections;

use std::ptr;
use std::vec_ng::Vec;

use ll::*;

pub mod ll {
    #[allow(non_camel_case_types)];

    pub use ll::platform::*;
    pub use ll::windef::*;
    pub use ll::all::*;
    pub use ll::font::*;

    pub mod platform;
    pub mod windef;
    pub mod all;
    pub mod font;
    pub mod gdi;
}

pub mod instance;
pub mod resource;
pub mod font;
pub mod wchar;
pub mod window;
pub mod gdi;
pub mod dialog;

/// Returns a vector of (variable, value) pairs for all the environment
/// variables of the current process.
/// This is unicode-version of `std::os::env()`.
pub fn env() -> Vec<(~str, ~str)> {
    unsafe {
        unsafe fn get_env_pairs() -> Vec<~str> {
            extern "system" {
                fn GetEnvironmentStringsW() -> *u16;
                fn FreeEnvironmentStringsW(ch: *u16) -> std::libc::BOOL;
            }

            let ch = GetEnvironmentStringsW();
            if ch.is_null() {
                fail!("os::env() failure getting env string from OS: {}",
                       std::os::last_os_error());
            }
            let mut result = Vec::new();
            wchar::from_c_u16_multistring(ch as *u16, None, |cstr| {
                result.push(cstr.to_str());
            });
            FreeEnvironmentStringsW(ch);
            result
        }

        fn env_convert(input: Vec<~str>) -> Vec<(~str, ~str)> {
            let mut pairs = Vec::new();
            for p in input.iter() {
                let vs: Vec<&str> = p.splitn('=', 1).collect();
                debug!("splitting: vs: {:?} len: {}", vs, vs.len());
                assert_eq!(vs.len(), 2);
                pairs.push((vs.get(0).to_owned(), vs.get(1).to_owned()));
            }
            pairs
        }
        let unparsed_environ = get_env_pairs();
        debug!("unp: {:?}", unparsed_environ);
        env_convert(unparsed_environ)
    }
}

pub fn get_last_error() -> DWORD {
    unsafe { GetLastError() }
}

pub fn def_window_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, w, l) }
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
            GetMessageW(&msg as *MSG, ptr::mut_null(),
                    0 as UINT, 0 as UINT)
        };

        if ret == 0 {
            let exit_code = msg.wParam;
            return exit_code as uint;
        }
        else {
            unsafe {
                TranslateMessage(&msg as *MSG);
                DispatchMessageW(&msg as *MSG);
            }
        }
    }
}
