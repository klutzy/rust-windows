use std::ptr;

use ll::*;
use font::Font;
use window::WindowImpl;

pub struct Dc {
    raw: HDC,
}

impl Dc {
    pub fn raw(&self) -> HDC {
        self.raw
    }

    pub fn text_out(&self, x: int, y: int, s: &str) -> bool {
        let mut s16 = s.to_utf16();
        let len = s16.len();

        s16.push(0u16);
        let ret = unsafe {
            TextOutW(self.raw, x as c_int, y as c_int, s16.as_mut_ptr(), len as i32)
        };
        ret != 0
    }

    pub fn select_font(&self, font: &Font) -> Option<Font> {
        let res = unsafe { SelectObject(self.raw, font.font as HGDIOBJ) };
        if res.is_null() {
            None
        } else {
            Some(Font { font: res })
        }
    }
}

pub struct PaintDc {
    dc: Dc,
    wnd: HWND,
    ps: PAINTSTRUCT,
}

impl PaintDc {
    pub fn new<W: WindowImpl>(w: &W) -> Option<PaintDc> {
        let mut ps = PAINTSTRUCT {
            hdc: ptr::mut_null(),
            fErase: 0 as BOOL,
            rcPaint: RECT {
                left: 0 as LONG, top: 0 as LONG,
                right: 0 as LONG, bottom: 0 as LONG
            },
            fRestore: 0 as BOOL,
            fIncUpdate: 0 as BOOL,
            rgbReserved: [0 as BYTE, ..32],
        };

        let wnd = w.wnd().wnd;
        let dc = unsafe { BeginPaint(wnd, &mut ps) };
        if dc.is_null() {
            return None;
        }

        let pdc = PaintDc {
            dc: Dc { raw: dc },
            wnd: wnd,
            ps: ps,
        };
        Some(pdc)
    }
}

impl Drop for PaintDc {
    fn drop(&mut self) {
        unsafe { EndPaint(self.wnd, &self.ps) };
    }
}
