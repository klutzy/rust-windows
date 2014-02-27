use ll::*;
use wchar::ToCU16Str;
use window::*;

pub trait DialogUtil {
    fn message_box(&self, msg: &str, title: &str);
}

impl DialogUtil for Window {
    fn message_box(&self, msg: &str, title: &str) {
        let msg_u = msg.to_c_u16();
        let title_u = title.to_c_u16();
        unsafe {
            MessageBoxW(self.wnd, msg_u.as_ptr(), title_u.as_ptr(), 0u32);
        }
    }
}
