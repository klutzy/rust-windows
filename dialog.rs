use ll::*;
use wchar::*;
use window::*;

pub trait DialogUtil {
    fn message_box(&self, msg: &str, title: &str);
}

impl DialogUtil for Window {
    fn message_box(&self, msg: &str, title: &str) {
        msg.with_c_u16_str(|msg_p| {
            title.with_c_u16_str(|title_p| {
                unsafe {
                    MessageBoxW(self.wnd, msg_p, title_p, 0u32);
                }
            })
        })
    }
}

