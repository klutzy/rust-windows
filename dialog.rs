use ll::*;
use wchar::*;
use window::*;

pub trait DialogUtil {
    fn message_box(&self, msg: &str, title: &str);
}

impl DialogUtil for Window {
    #[fixed_stack_segment]
    fn message_box(&self, msg: &str, title: &str) {
        do msg.with_c_u16_str |msg_p| {
            do title.with_c_u16_str |title_p| {
                unsafe {
                    MessageBoxW(self.wnd, msg_p, title_p, 0u32);
                }
            }
        }
    }
}

