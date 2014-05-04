use libc::c_int;
use libc::types::os::arch::extra::{LPCWSTR, LPWSTR};
use ll::windef::{HDC, BOOL, HBITMAP, HGDIOBJ, DWORD, HFONT};

#[link(name = "gdi32")]
extern "system" {
    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;

    pub fn DeleteDC(hdc: HDC) -> BOOL;

    pub fn CreateCompatibleBitmap(hdc: HDC, nWidth: c_int, nHeight: c_int) -> HBITMAP;

    pub fn SelectObject(hdc: HDC, hgdiobj: HGDIOBJ) -> HGDIOBJ;

    pub fn DeleteObject(hObject: HGDIOBJ) -> BOOL;

    pub fn BitBlt(hdcDest: HDC, nXDest: c_int, nYDest: c_int, nWidth: c_int, nHeight: c_int,
                  hdcSrc: HDC, nXSrc: c_int, nYSrc: c_int, dwRop: DWORD) -> BOOL;

    pub fn CreateFontW(
        height: c_int, width: c_int, escapement: c_int, orientation: c_int,
        weight: c_int, italic: DWORD, underline: DWORD, strikeOut: DWORD,
        charSet: DWORD, outputPrecision: DWORD, clipPrecision: DWORD,
        quality: DWORD, pitchAndFamily: DWORD, face: LPCWSTR
    ) -> HFONT;

    pub fn TextOutW(
        hdc: HDC, nXStart: c_int, nYStart: c_int, lpString: LPWSTR, cchString: c_int
    ) -> BOOL;
}
