pub use core::libc::*;
pub use core::libc::types::os::arch::extra::*;

pub type ATOM = WORD;
pub type UINT = c_uint;

// 32-bit specific
pub type UINT_PTR = c_uint;
pub type LONG_PTR = c_long;

pub type HANDLE = PVOID;

pub type HBRUSH = HANDLE;
pub type HCURSOR = HICON;
pub type HMENU = HANDLE;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HWND = HANDLE;

pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;

pub type PVOID = *c_void;

// extern fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT
pub type WNDPROC = *u8;

pub struct WNDCLASSEX {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
    hIconSm: HICON,
}

pub extern "stdcall" mod user32 {
    unsafe fn CreateWindowExW(extrastyle: DWORD, classname: LPCWSTR,
            windowname: LPCWSTR, style: DWORD,
            x: c_int, y: c_int, width: c_int, height: c_int,
            parent: HWND, menu: HMENU, instance: HINSTANCE, param: LPVOID
    ) -> HWND;

    unsafe fn MessageBoxW(
            hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
    ) -> c_int;

    unsafe fn RegisterClassExW(lpwcx: *WNDCLASSEX) -> ATOM;

    unsafe fn DefWindowProcW(
            hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM
    ) -> LRESULT;
}
