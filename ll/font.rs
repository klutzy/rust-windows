use ll::platform::*;
use ll::windef::*;

extern "stdcall" {
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
