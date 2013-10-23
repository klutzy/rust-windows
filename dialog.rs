use ll::*;
use wchar::*;
use window::*;

pub trait DialogUtil {
    fn message_box(&self, msg: &str, title: &str);
}

impl DialogUtil for Window {
    #[fixed_stack_segment]
    fn message_box(&self, msg: &str, title: &str) {
        do with_utf16_p(msg) |msg_p| {
            do with_utf16_p(title) |title_p| {
                unsafe {
                    MessageBoxW(self.wnd, msg_p, title_p, 0u32);
                }
            }
        }
    }
}

