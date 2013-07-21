#[crate_type = "lib"];
#[link(name = "win32")];

use std::ptr;

use ll::*;

pub mod ll;
pub mod window;

/// returns main HINSTANCE which can be obtained from WinMain().
pub fn get_main_instance() -> HINSTANCE {
    unsafe { kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE }
}

pub fn main_window_loop() -> u32 {
    let msg = MSG {
        hwnd: ptr::null(),
        message: 0 as UINT,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT { x: 0 as LONG, y: 0 as LONG },
    };
    loop {
        let ret = unsafe {
            user32::GetMessageW(&msg as *MSG, ptr::null(),
                    0 as UINT, 0 as UINT)
        };

        if ret == 0 {
            let exit_code = msg.wParam;
            return exit_code;
        }
        else {
            unsafe {
                user32::TranslateMessage(&msg as *MSG);
                user32::DispatchMessageW(&msg as *MSG);
            }
        }
    }
}
