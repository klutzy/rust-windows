use std::ptr;
use std::vec;
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
}

impl<T: Window> WindowUtil for T {
    pub fn message_box(&self, msg: &str, title: &str) {
        let wmsg = msg.to_utf16();
        wmsg.push(0u16);
        let wtitle = title.to_utf16();
        wtitle.push(0u16);
        unsafe {
            user32::MessageBoxW(self.hwnd(),
            vec::raw::to_ptr(wmsg), vec::raw::to_ptr(wtitle),
            0u32);
        }
    }
}
