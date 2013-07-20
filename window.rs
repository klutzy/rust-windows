use std::ptr;
use std::os::win32::as_utf16_p;
use ll::*;

pub trait Window {
    fn hwnd(&self) -> HWND;
}

pub struct EmptyWindow {
    raw: HWND,
}

impl Window for EmptyWindow {
    fn hwnd(&self) -> HWND {
        self.raw
    }
}

pub fn null() -> EmptyWindow {
    EmptyWindow {
        raw: ptr::null(),
    }
}

pub trait WindowUtil {
    pub fn message_box(&self, msg: &str, title: &str);

    pub fn show(&self, cmd_show: int) -> bool;

    pub fn update(&self) -> bool;
}

impl<T: Window> WindowUtil for T {
    pub fn message_box(&self, msg: &str, title: &str) {
        do as_utf16_p(msg) |msg_p| {
            do as_utf16_p(title) |title_p| {
                unsafe {
                    user32::MessageBoxW(self.hwnd(), msg_p, title_p, 0u32);
                }
            }
        }
    }

    pub fn show(&self, cmd_show: int) -> bool {
        unsafe { user32::ShowWindow(self.hwnd(), cmd_show as c_int) as bool }
    }

    pub fn update(&self) -> bool {
        unsafe { user32::UpdateWindow(self.hwnd()) as bool }
    }
}
