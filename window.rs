use std::local_data;
use std::hashmap::HashMap;
use std::ptr;
use std;

use ll::*;

// XXX copy of std::os::win32::as_utf16_p
pub fn as_utf16_p<T>(s: &str, f: &fn(*u16) -> T) -> T {
    let mut t = s.to_utf16();
    // Null terminate before passing on.
    t.push(0u16);
    t.as_imm_buf(|buf, _len| f(buf))
}

pub struct Instance {
    instance: HINSTANCE
}

impl Instance {
    #[fixed_stack_segment]
    pub fn register(&self, classname: &str) -> bool {
        do as_utf16_p(classname) |clsname_p| {
            let wcex = WNDCLASSEX {
                cbSize: std::sys::size_of::<WNDCLASSEX>() as UINT,
                style: 0x0001 | 0x0002, // CS_HREDRAW | CS_VREDRAW
                lpfnWndProc: main_wnd_proc as *u8,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: self.instance,
                hIcon: ptr::mut_null(),
                hCursor: ptr::mut_null(),
                hbrBackground: (5 + 1) as HBRUSH,
                lpszMenuName: ptr::null(),
                lpszClassName: clsname_p,
                hIconSm: ptr::mut_null(),
            };

            let res = unsafe { user32::RegisterClassExW(&wcex) };
            res != 0
        }
    }

    #[fixed_stack_segment]
    pub fn main_instance() -> Instance {
        Instance {
            instance: unsafe { kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE },
        }
    }
}

#[deriving(Eq, IterBytes)]
pub struct Window {
    wnd: HWND,
}

impl Clone for Window {
    fn clone(&self) -> Window {
        Window {
            wnd: self.wnd,
        }
    }
}

impl Window {
    pub fn null() -> Window {
        Window {
            wnd: ptr::mut_null(),
        }
    }

    #[fixed_stack_segment]
    pub fn create(instance: Instance, proc: ~WndProc, classname: &str, title: &str) -> Option<Window> {
        instance.register(classname);
        local_data::set(key_init_wnd, proc);

        let WS_OVERLAPPED = 0x00000000u32;
        let WS_CAPTION = 0x00C00000u32;
        let WS_SYSMENU = 0x00080000u32;
        let WS_THICKFRAME = 0x00040000u32;
        let WS_MINIMIZEBOX = 0x00020000u32;
        let WS_MAXIMIZEBOX = 0x00010000u32;
        let WS_OVERLAPPEDWINDOW = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU |
                WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

        let wnd = unsafe {
            do as_utf16_p(classname) |clsname_p| {
                do as_utf16_p(title) |title_p| {
                    let wnd = user32::CreateWindowExW(
                        0, clsname_p, title_p, WS_OVERLAPPEDWINDOW as DWORD,
                        0 as c_int, 0 as c_int, 400 as c_int, 400 as c_int,
                        ptr::mut_null(), ptr::mut_null(), instance.instance,
                        ptr::null::<*c_void>() as *mut c_void
                    );
                    wnd
                }
            }
        };

        if wnd != ptr::mut_null() {
            Some(Window { wnd: wnd })
        } else {
            None
        }
    }

    #[fixed_stack_segment]
    pub fn show(&self, cmd_show: int) -> bool {
        unsafe { user32::ShowWindow(self.wnd, cmd_show as c_int) == 0 }
    }

    #[fixed_stack_segment]
    pub fn update(&self) -> bool {
        unsafe { user32::UpdateWindow(self.wnd) == 0 }
    }
}

pub trait WndProc {
    fn wnd<'a>(&'a self) -> &'a Window;
    fn wnd_mut<'a>(&'a mut self) -> &'a mut Window;
    fn wnd_proc(&self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT;
}

pub type WindowMap = HashMap<Window, ~WndProc>;

local_data_key!(pub key_win_map: WindowMap)
local_data_key!(pub key_init_wnd: ~WndProc)

pub fn init_window_map() {
    let win_map: WindowMap = HashMap::new();
    local_data::set(key_win_map, win_map);
}

pub extern "stdcall" fn main_wnd_proc(wnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    debug2!("main_wnd_proc: wnd {:?} / msg 0x{:x} / w {:?} / l {:?}", wnd, msg as uint, w, l);
    let win = Window { wnd: wnd };
    let null_proc = local_data::pop(key_init_wnd);
    match null_proc {
        Some(nproc) => {
            // hello newcomer.
            let mut nproc = nproc;
            nproc.wnd_mut().wnd = wnd;
            let mut wmap = local_data::pop(key_win_map).unwrap();
            wmap.insert(win, nproc);
            local_data::set(key_win_map, wmap); // XXX?

        },
        None => {}
    };
    do local_data::get(key_win_map) |wmap| {
        let wmap = wmap.unwrap();
        let proc = wmap.find(&win).unwrap();
        proc.wnd_proc(msg, w, l)
    }
}

pub trait DialogUtil {
    fn message_box(&self, msg: &str, title: &str);
}

impl DialogUtil for Window {
    #[fixed_stack_segment]
    fn message_box(&self, msg: &str, title: &str) {
        do as_utf16_p(msg) |msg_p| {
            do as_utf16_p(title) |title_p| {
                unsafe {
                    user32::MessageBoxW(self.wnd, msg_p, title_p, 0u32);
                }
            }
        }
    }
}

pub trait WindowPaint {
    fn begin_paint(&self) -> (HDC, PAINTSTRUCT);
    fn end_paint(&self, ps: &PAINTSTRUCT);
    fn text_out(&self, dc: HDC, x: int, y: int, s: &str) -> bool;
}

impl<T: WndProc> WindowPaint for T {
    #[fixed_stack_segment]
    fn begin_paint(&self) -> (HDC, PAINTSTRUCT) {
        // TODO params
        let rgb_res: [BYTE, ..32] = [0 as BYTE, ..32];
        let ps = PAINTSTRUCT {
            hdc: ptr::mut_null(),
            fErase: 0 as BOOL,
            rcPaint: RECT {
                left: 0 as LONG, top: 0 as LONG,
                right: 0 as LONG, bottom: 0 as LONG
            },
            fRestore: 0 as BOOL,
            fIncUpdate: 0 as BOOL,
            rgbReserved: &rgb_res,
        };

        let dc = unsafe { user32::BeginPaint(self.wnd().wnd, &ps) };
        (dc, ps)
    }

    #[fixed_stack_segment]
    fn end_paint(&self, ps: &PAINTSTRUCT) {
        unsafe { user32::EndPaint(self.wnd().wnd, ps) };
    }

    #[fixed_stack_segment]
    fn text_out(&self, dc: HDC, x: int, y: int, s: &str) -> bool {
        let mut s16 = s.to_utf16();
        s16.push(0u16);
        do s16.as_mut_buf |buf, len| {
            let len = len - 1;
            let ret = unsafe {
                gdi32::TextOutW(dc, x as c_int, y as c_int, buf, len as i32)
            };
            ret != 0
        }
    }
}

pub trait OnCreate {
    fn on_create(&self, _cs: &CREATESTRUCT) -> bool {
        true
    }
}

pub trait OnDestroy {
    #[fixed_stack_segment]
    fn on_destroy(&self) {
        unsafe {
            user32::PostQuitMessage(0 as c_int);
        }
    }
}

pub trait OnPaint {
    fn on_paint(&self, _dc: HDC) {
    }
}
