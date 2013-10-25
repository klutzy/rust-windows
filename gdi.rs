use std::ptr;

use ll::*;
use window::*;

pub trait OnPaint {
    fn on_paint(&self, _dc: HDC) {
    }
}

pub trait PaintChunk {
    fn do_paint(&self, w: WPARAM, l: LPARAM) -> LRESULT;
}

impl<T: WindowImpl + OnPaint> PaintChunk for T {
    #[fixed_stack_segment]
    fn do_paint(&self, _w: WPARAM, _l: LPARAM) -> LRESULT {
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

        self.on_paint(dc);

        unsafe { EndPaint(self.wnd().wnd, &ps) };

        0 as LRESULT
    }
}

pub trait PaintUtil {
    fn text_out(&self, dc: HDC, x: int, y: int, s: &str) -> bool;
}

impl<T: WindowImpl> PaintUtil for T {
    #[fixed_stack_segment]
    fn text_out(&self, dc: HDC, x: int, y: int, s: &str) -> bool {
        let mut s16 = s.to_utf16();
        s16.push(0u16);
        do s16.as_mut_buf |buf, len| {
            let len = len - 1;
            let ret = unsafe {
                TextOutW(dc, x as c_int, y as c_int, buf, len as i32)
            };
            ret != 0
        }
    }
}
