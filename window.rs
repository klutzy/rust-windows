use core::ptr;
use ll::*;

pub struct Window {
    raw: HWND,
}

impl Window {
    pub fn null() -> Window {
        unsafe {
            Window {
                raw: ptr::null(),
            }
        }
    }

    pub fn message_box(&self, msg: &str, title: &str) {
        let mut wmsg = str::to_utf16(msg);
        wmsg.push(0u16);
        let mut wtitle = str::to_utf16(title);
        wtitle.push(0u16);
        unsafe {
            user32::MessageBoxW(self.raw,
            vec::raw::to_ptr(wmsg), vec::raw::to_ptr(wtitle),
            0u32);
        }
    }
}
