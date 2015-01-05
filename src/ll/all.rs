// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]

use ll::types::*;

// extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT
pub type WNDPROC = *const c_void;

#[repr(C)]
#[derive(Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: LPVOID,
    pub bInheritHandle: BOOL,
}

#[repr(C)]
#[derive(Copy)]
pub struct PROCESS_INFORMATION {
    pub hProcess: HANDLE,
    pub hThread: HANDLE,
    pub dwProcessId: DWORD,
    pub dwThreadId: DWORD,
}

#[repr(C)]
#[derive(Copy)]
pub struct STARTUPINFO {
    pub cb: DWORD,
    pub lpReserved: LPWSTR,
    pub lpDesktop: LPWSTR,
    pub lpTitle: LPWSTR,
    pub dwX: DWORD,
    pub dwY: DWORD,
    pub dwXSize: DWORD,
    pub dwYSize: DWORD,
    pub dwXCountChars: DWORD,
    pub dwYCountChars: DWORD,
    pub dwFillAttribute: DWORD,
    pub dwFlags: DWORD,
    pub wShowWindow: WORD,
    pub cbReserved2: WORD,
    pub lpReserved2: LPBYTE,
    pub hStdInput: HANDLE,
    pub hStdOutput: HANDLE,
    pub hStdError: HANDLE,
}

#[repr(C)]
#[derive(Copy)]
pub struct WNDCLASSEX {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
    pub hIconSm: HICON,
}

#[repr(C)]
#[derive(Copy)]
pub struct CREATESTRUCT {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCWSTR,
    pub lpszClass: LPCWSTR,
    pub dwExStyle: DWORD,
}

#[repr(C)]
#[derive(Copy)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
#[derive(Copy)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

#[derive(Copy)]
#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}

// kernel32
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    pub fn GetLastError() -> DWORD;

    pub fn CreateProcessW(
        lpApplicationName: LPCWSTR, lpCommandLine: LPWSTR,
        lpProcessAttributes: *const SECURITY_ATTRIBUTES,
        lpThreadAttributes: *const SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: *mut STARTUPINFO,
        lpProcessInformation: *mut PROCESS_INFORMATION
    ) -> BOOL;
}

// user32
extern "system" {
    pub fn CreateWindowExW(extrastyle: DWORD, classname: LPCWSTR,
            windowname: LPCWSTR, style: DWORD,
            x: c_int, y: c_int, width: c_int, height: c_int,
            parent: HWND, menu: HMENU, instance: HINSTANCE, param: LPVOID
    ) -> HWND;

    pub fn ShowWindow(hwnd: HWND, nCmdShow: c_int) -> BOOL;

    pub fn ShowWindowAsync(hwnd: HWND, nCmdShow: c_int) -> BOOL;

    pub fn UpdateWindow(hwnd: HWND) -> BOOL;

    pub fn BeginPaint(hwnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HDC;

    pub fn EndPaint(hwnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;

    pub fn MessageBoxW(
            hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
    ) -> c_int;

    pub fn RegisterClassExW(lpwcx: *const WNDCLASSEX) -> ATOM;

    pub fn DefWindowProcW(
            hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM
    ) -> LRESULT;

    pub fn GetMessageW(
            lpMsg: *const MSG, hWnd: HWND,
            wMsgFilterMin: UINT, wMsgFilterMAx: UINT
    ) -> BOOL;

    pub fn PeekMessageW(
            lpMsg: *const MSG, hWnd: HWND,
            wMsgFilterMin: UINT, wMsgFilterMAx: UINT, wRemoveMsg: UINT
    ) -> BOOL;

    pub fn PostMessageW(
            hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM
    ) -> BOOL;

    pub fn PostQuitMessage(nExitCode: c_int);

    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

    #[cfg(target_arch = "x86")]
    pub fn GetClassLongW(hwnd: HWND, nIndex: c_int) -> DWORD;

    #[cfg(target_arch = "x86")]
    pub fn SetClassLongW(
            hwnd: HWND, nIndex: c_int, dwNewLong: LONG
    ) -> DWORD;

    pub fn LoadImageW(
        hinst: HINSTANCE, name: LPCWSTR, type_: UINT,
        xDesired: c_int, yDesired: c_int, load: UINT
    ) -> HANDLE;

    pub fn GetSysColor(nIndex: c_int) -> DWORD;

    pub fn GetClientRect(hwnd: HWND, rect: LPRECT) -> BOOL;

    pub fn InvalidateRect(hwnd: HWND, lpRect: *const RECT, erase: BOOL) -> BOOL;

    pub fn SetWindowPos(
        hwnd: HWND, hwndInsertAfter: HWND, x: c_int, y: c_int,
        cx: c_int, cy: c_int, flags: UINT
    ) -> BOOL;

    pub fn SetFocus(hwnd: HWND) -> HWND;

    pub fn SendMessageW(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
}
