pub use core::libc::*;
pub use core::libc::types::os::arch::extra::*;

pub type PVOID = *c_void;
pub type HANDLE = PVOID;
pub type HWND = HANDLE;
pub type HMENU = HANDLE;
pub type HINSTANCE = HANDLE;
pub type UINT = c_uint;

pub extern "stdcall" mod user32 {
    unsafe fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR,
            uType: UINT) -> c_int;
}
