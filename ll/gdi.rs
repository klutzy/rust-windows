use std::libc::c_int;
use ll::windef::{HDC, BOOL, HBITMAP, HGDIOBJ, DWORD};

#[link(name = "gdi32")]
extern "system" {
    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;

    pub fn DeleteDC(hdc: HDC) -> BOOL;

    pub fn CreateCompatibleBitmap(hdc: HDC, nWidth: c_int, nHeight: c_int) -> HBITMAP;

    pub fn SelectObject(hdc: HDC, hgdiobj: HGDIOBJ) -> HGDIOBJ;

    pub fn DeleteObject(hObject: HGDIOBJ) -> BOOL;

    pub fn BitBlt(hdcDest: HDC, nXDest: c_int, nYDest: c_int, nWidth: c_int, nHeight: c_int,
                  hdcSrc: HDC, nXSrc: c_int, nYSrc: c_int, dwRop: DWORD) -> BOOL;
}
