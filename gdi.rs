use std::ptr;

use libc::c_int;
use libc::{LONG, BOOL, BYTE, HANDLE, DWORD};
use ll::all::PAINTSTRUCT;
use ll::windef::{HDC, HWND, RECT, HBITMAP, HGDIOBJ};
use ll::gdi;
use font::Font;
use window::WindowImpl;

pub struct Dc {
    pub raw: HDC,
}

impl Dc {
    pub fn raw(&self) -> HDC {
        self.raw
    }

    pub fn text_out(&self, x: int, y: int, s: &str) -> bool {
        #![allow(deprecated_owned_vector)]
        let mut s16 = s.to_utf16();
        let len = s16.len();

        s16.push(0u16);
        let ret = unsafe {
            gdi::TextOutW(self.raw, x as c_int, y as c_int, s16.as_mut_ptr(), len as i32)
        };
        ret != 0
    }

    pub fn select_object(&self, handle: HANDLE) -> HANDLE {
        unsafe { gdi::SelectObject(self.raw, handle as HGDIOBJ) }
    }

    pub fn select_font(&self, font: &Font) -> Option<Font> {
        let res = self.select_object(font.font);
        if res.is_null() {
            None
        } else {
            Some(Font { font: res })
        }
    }

    pub fn create_compatible_bitmap(&self, width: int, height: int) -> Bitmap {
        let raw = unsafe {
            gdi::CreateCompatibleBitmap(self.raw, width as c_int, height as c_int)
        };
        Bitmap { raw: raw }
    }

    pub fn bit_blt(&self, pos: (int, int), size: (int, int), src: &Dc,
                   src_pos: (int, int), flag: DWORD) -> bool {
        let res = unsafe {
            let (px, py) = pos;
            let (w, h) = size;
            let (sx, sy) = src_pos;
            gdi::BitBlt(self.raw, px as c_int, py as c_int, w as c_int, h as c_int,
                        src.raw, sx as c_int, sy as c_int, flag)
        };
        return res != 0;
    }
}

pub struct PaintDc {
    pub dc: Dc,
    pub wnd: HWND,
    pub ps: PAINTSTRUCT,
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
        let dc = unsafe { super::ll::all::BeginPaint(wnd, &mut ps) };
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
        unsafe { super::ll::all::EndPaint(self.wnd, &self.ps) };
    }
}

pub struct MemoryDc {
    pub dc: Dc,
}

impl MemoryDc {
    pub fn new(dc: &Dc) -> Option<MemoryDc> {
        let hdc = unsafe { gdi::CreateCompatibleDC(dc.raw) };
        if hdc.is_null() {
            return None;
        }

        Some(MemoryDc { dc: Dc { raw: hdc } })
    }
}

impl Drop for MemoryDc {
    fn drop(&mut self) {
        unsafe { gdi::DeleteDC(self.dc.raw) };
    }
}

pub struct Bitmap {
    raw: HBITMAP,
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { gdi::DeleteObject(self.raw) };
    }
}
