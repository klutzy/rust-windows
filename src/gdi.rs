// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

use gdi32;
use user32;
use winapi::{
    BOOL, BYTE, COLORREF, DWORD, HANDLE, HBITMAP, HBRUSH, HDC, HFONT, HGDIOBJ, HWND, LONG,
    PAINTSTRUCT, RECT, c_int,
};

use font::Font;
use window::WindowImpl;

#[derive(Copy)]
pub struct Dc {
    pub raw: HDC,
}

impl Dc {
    pub fn raw(&self) -> HDC {
        self.raw
    }

    pub fn text_out(&self, x: isize, y: isize, s: &str) -> bool {
        let mut s16 : Vec<u16> = s.utf16_units().collect();
        let len = s16.len();

        s16.push(0u16);
        let ret = unsafe {
            gdi32::TextOutW(self.raw, x as c_int, y as c_int, s16.as_mut_ptr(), len as i32)
        };
        ret != 0
    }

    pub fn select_object(&self, handle: HANDLE) -> HANDLE {
        unsafe { gdi32::SelectObject(self.raw, handle as HGDIOBJ) }
    }

    pub fn select_font(&self, font: &Font) -> Option<Font> {
        let res = self.select_object(font.font as HANDLE);
        if res.is_null() {
            None
        } else {
            Some(Font { font: res as HFONT })
        }
    }

    pub fn set_text_color(&self, color: COLORREF) -> COLORREF {
        unsafe { gdi32::SetTextColor(self.raw, color) }
    }

    pub fn set_background_color(&self, color: COLORREF) -> COLORREF {
        unsafe { gdi32::SetBkColor(self.raw, color) }
    }

    pub fn create_compatible_bitmap(&self, width: isize, height: isize) -> Bitmap {
        let raw = unsafe {
            gdi32::CreateCompatibleBitmap(self.raw, width as c_int, height as c_int)
        };
        Bitmap { raw: raw }
    }

    pub fn bit_blt(&self, pos: (isize, isize), size: (isize, isize), src: &Dc,
                   src_pos: (isize, isize), flag: DWORD) -> bool {
        let res = unsafe {
            let (px, py) = pos;
            let (w, h) = size;
            let (sx, sy) = src_pos;
            gdi32::BitBlt(self.raw, px as c_int, py as c_int, w as c_int, h as c_int,
                        src.raw, sx as c_int, sy as c_int, flag)
        };
        return res != 0;
    }

    pub fn fill_rect(&self, left_top: (isize, isize), right_bottom: (isize, isize), brush: HBRUSH) -> bool {
        let (left, top) = left_top;
        let (right, bottom) = right_bottom;
        let rect = RECT {
            left: left as LONG, top: top as LONG,
            right: right as LONG, bottom: bottom as LONG
        };
        let res = unsafe {
            gdi32::FillRect(self.raw, &rect, brush)
        };
        return res != 0;
    }

    pub fn rect(&self, left_top: (isize, isize), right_bottom: (isize, isize)) -> bool {
        let (left, top) = left_top;
        let (right, bottom) = right_bottom;
        let res = unsafe {
            gdi32::Rectangle(self.raw, left as c_int, top as c_int, right as c_int, bottom as c_int)
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
            hdc: ptr::null_mut(),
            fErase: 0 as BOOL,
            rcPaint: RECT {
                left: 0 as LONG, top: 0 as LONG,
                right: 0 as LONG, bottom: 0 as LONG
            },
            fRestore: 0 as BOOL,
            fIncUpdate: 0 as BOOL,
            rgbReserved: [0 as BYTE; 32],
        };

        let wnd = w.wnd().wnd;
        let dc = unsafe { user32::BeginPaint(wnd, &mut ps) };
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
        unsafe { user32::EndPaint(self.wnd, &self.ps) };
    }
}

pub struct MemoryDc {
    pub dc: Dc,
}

impl MemoryDc {
    pub fn new(dc: &Dc) -> Option<MemoryDc> {
        let hdc = unsafe { gdi32::CreateCompatibleDC(dc.raw) };
        if hdc.is_null() {
            return None;
        }

        Some(MemoryDc { dc: Dc { raw: hdc } })
    }
}

impl Drop for MemoryDc {
    fn drop(&mut self) {
        unsafe { gdi32::DeleteDC(self.dc.raw) };
    }
}

pub struct Bitmap {
    raw: HBITMAP,
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { gdi32::DeleteObject(self.raw as HGDIOBJ) };
    }
}
