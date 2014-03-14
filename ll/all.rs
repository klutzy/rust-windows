use ll::platform::*;
use ll::windef::*;

// extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT
pub type WNDPROC = *c_void;

pub struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL,
}

pub struct PROCESS_INFORMATION {
    hProcess: HANDLE,
    hTread: HANDLE,
    dwProcessId: DWORD,
    dwThreadId: DWORD,
}

pub struct STARTUPINFO {
    cb: DWORD,
    lpReserved: LPWSTR,
    lpDesktop: LPWSTR,
    lpTitle: LPWSTR,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: WORD,
    cbReserved2: WORD,
    lpReserved2: LPBYTE,
    hStdInput: HANDLE,
    hStdOutput: HANDLE,
    hStdError: HANDLE,
}

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

pub struct CREATESTRUCT {
    lpCreateParams: LPVOID,
    hInstance: HINSTANCE,
    hMenu: HMENU,
    hwndParent: HWND,
    cy: c_int,
    cx: c_int,
    y: c_int,
    x: c_int,
    style: LONG,
    lpszName: LPCWSTR,
    lpszClass: LPCWSTR,
    dwExStyle: DWORD,
}

pub struct POINT {
    x: LONG,
    y: LONG,
}

pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}

pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [BYTE, ..32],
}

// kernel32
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    pub fn GetLastError() -> DWORD;

    pub fn CreateProcessW(
        lpApplicationName: LPCWSTR, lpCommandLine: LPWSTR,
        lpProcessAttributes: *SECURITY_ATTRIBUTES,
        lpThreadAttributes: *SECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFO,
        lpProcessInformation: LPPROCESS_INFORMATION
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

    pub fn EndPaint(hwnd: HWND, lpPaint: *PAINTSTRUCT) -> BOOL;

    pub fn MessageBoxW(
            hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
    ) -> c_int;

    pub fn RegisterClassExW(lpwcx: *WNDCLASSEX) -> ATOM;

    pub fn DefWindowProcW(
            hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM
    ) -> LRESULT;

    pub fn GetMessageW(
            lpMsg: *MSG, hWnd: HWND,
            wMsgFilterMin: UINT, wMsgFilterMAx: UINT
    ) -> BOOL;

    pub fn PeekMessageW(
            lpMsg: *MSG, hWnd: HWND,
            wMsgFilterMin: UINT, wMsgFilterMAx: UINT, wRemoveMsg: UINT
    ) -> BOOL;

    pub fn PostMessageW(
            hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM
    ) -> BOOL;

    pub fn PostQuitMessage(nExitCode: c_int);

    pub fn TranslateMessage(lpMsg: *MSG) -> BOOL;

    pub fn DispatchMessageW(lpMsg: *MSG) -> LRESULT;

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

    pub fn GetClientRect(hwnd: HWND, rect: LPRECT) -> BOOL;

    pub fn SetWindowPos(
        hwnd: HWND, hwndInsertAfter: HWND, x: c_int, y: c_int,
        cx: c_int, cy: c_int, flags: UINT
    ) -> BOOL;

    pub fn SetFocus(hwnd: HWND) -> HWND;

    pub fn SendMessageW(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
}

// gdi32
#[link(name = "gdi32")]
extern "system" {
    pub fn SelectObject(hdc: HDC, hgdiobj: HGDIOBJ) -> HGDIOBJ;
}
