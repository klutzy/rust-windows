use std::ptr;

use ll::*;
use font::Font;
use window::WindowImpl;

// TODO: allocated DC (CreateDC/ReleaseDC)
pub struct Dc {
    dc: HDC,
}

impl Dc {
    #[fixed_stack_segment]
    pub fn text_out(&self, x: int, y: int, s: &str) -> bool {
        let mut s16 = s.to_utf16();
        s16.push(0u16);
        do s16.as_mut_buf |buf, len| {
            let len = len - 1;
            let ret = unsafe {
                TextOutW(self.dc, x as c_int, y as c_int, buf, len as i32)
            };
            ret != 0
        }
    }

    #[fixed_stack_segment]
    pub fn select_font(&self, font: &Font) -> Option<Font> {
        let res = unsafe { SelectObject(self.dc, font.font as HGDIOBJ) };
        if res.is_null() {
            None
        } else {
            Some(Font { font: res })
        }
    }
}

// TODO better name
pub trait WindowPaint {
    fn with_paint_dc<T>(&self, f: &fn(Dc) -> T) -> T;
}

impl<T: WindowImpl> WindowPaint for T {
    #[fixed_stack_segment]
    fn with_paint_dc<T>(&self, f: &fn(Dc) -> T) -> T {
        let rgb_res: [BYTE, ..32] = [0 as BYTE, ..32];
        let ps = PAINTSTRUCT {
            hdc: ptr::mut_null(),
            fErase: 0 as BOOL,
            rcPaint: RECT {
                left: 0 as LONG, top: 0 as LONG,
                right: 0 as LONG, bottom: 0 as LONG
            },
            fRestore: 0 as BOOL,
            fIncUpdate: 0 as BOOL,
            rgbReserved: &rgb_res,
        };

        let dc = unsafe { BeginPaint(self.wnd().wnd, &ps) };
        let dc = Dc { dc: dc };

        let ret = f(dc);

        unsafe { EndPaint(self.wnd().wnd, &ps) };
        ret
    }
}
