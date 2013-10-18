#[feature(globs)];
#[crate_type = "lib"];
#[link(name = "win32")];

#[link_args = "-lgdi32"];

use std::ptr;

use ll::*;

pub mod ll {
    pub use ll::platform::*;
    pub use ll::windef::*;
    pub use ll::all::*;

    pub mod platform;
    pub mod windef;
    pub mod all;
}

pub mod window;

#[fixed_stack_segment]
pub fn def_window_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, w, l) }
}

#[fixed_stack_segment]
pub fn main_window_loop() -> u32 {
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
            return exit_code;
        }
        else {
            unsafe {
                TranslateMessage(&msg as *MSG);
                DispatchMessageW(&msg as *MSG);
            }
        }
    }
}
