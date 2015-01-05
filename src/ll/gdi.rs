// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::c_int;
use libc::types::os::arch::extra::{LPCWSTR, LPWSTR};
use ll::types::{HDC, BOOL, HBITMAP, HGDIOBJ, DWORD, HFONT, COLORREF, RECT, HBRUSH};

#[link(name = "gdi32")]
extern "system" {
    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;

    pub fn DeleteDC(hdc: HDC) -> BOOL;

    pub fn CreateCompatibleBitmap(hdc: HDC, nWidth: c_int, nHeight: c_int) -> HBITMAP;

    pub fn SelectObject(hdc: HDC, hgdiobj: HGDIOBJ) -> HGDIOBJ;

    pub fn GetStockObject(fnObject: c_int) -> HGDIOBJ;

    pub fn DeleteObject(hObject: HGDIOBJ) -> BOOL;

    pub fn SetDCBrushColor(hdc: HDC, crColor: COLORREF) -> COLORREF;

    pub fn BitBlt(hdcDest: HDC, nXDest: c_int, nYDest: c_int, nWidth: c_int, nHeight: c_int,
                  hdcSrc: HDC, nXSrc: c_int, nYSrc: c_int, dwRop: DWORD) -> BOOL;

    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;

    pub fn Rectangle(hDC: HDC, nLeftRect: c_int, nTopRect: c_int, nRightRect: c_int, nBottomRect: c_int) -> BOOL;

    pub fn CreateFontW(
        height: c_int, width: c_int, escapement: c_int, orientation: c_int,
        weight: c_int, italic: DWORD, underline: DWORD, strikeOut: DWORD,
        charSet: DWORD, outputPrecision: DWORD, clipPrecision: DWORD,
        quality: DWORD, pitchAndFamily: DWORD, face: LPCWSTR
    ) -> HFONT;

    pub fn TextOutW(
        hdc: HDC, nXStart: c_int, nYStart: c_int, lpString: LPWSTR, cchString: c_int
    ) -> BOOL;

    pub fn SetTextColor(hdc: HDC, color: COLORREF) -> COLORREF;

    pub fn SetBkColor(hdc: HDC, color: COLORREF) -> COLORREF;
}
