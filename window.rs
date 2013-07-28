use std::local_data;
use std::hashmap::HashMap;
use std::ptr;
use std::os::win32::as_utf16_p;
use std;

use ll::*;

pub trait Window {
    fn hwnd(&self) -> HWND;
    fn set_hwnd(&mut self, HWND);
    fn classname<'s>(&'s self) -> &'s str;
    fn wnd_proc(&mut self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT;
}

pub struct EmptyWindow;

impl Window for EmptyWindow {
    fn hwnd(&self) -> HWND {
        ptr::mut_null()
    }

    fn set_hwnd(&mut self, _hwnd: HWND) {
        fail!("set_hwnd on EmptyWindow");
    }

    fn classname<'s>(&'s self) -> &'s str {
        "EmptyWindow"
    }

    fn wnd_proc(&mut self, _msg: UINT, _w: WPARAM, _l: LPARAM) -> LRESULT {
        fail!("wnd_proc on EmptyWindow");
    }
}

pub fn null() -> EmptyWindow {
    EmptyWindow
}

impl Window for @mut Window {
    fn hwnd(&self) -> HWND {
        self.hwnd()
    }

    fn set_hwnd(&mut self, hwnd: HWND) {
        self.set_hwnd(hwnd)
    }

    fn classname<'s>(&'s self) -> &'s str {
        self.classname()
    }

    fn wnd_proc(&mut self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
        self.wnd_proc(msg, w, l)
    }
}

// Use local_data for HWND -> Window matching.
// For unassigned one during CreateWindow, we use special key: the NULL.
// TODO is this safe? no two Windows cannot be unassigned simultaneously?
pub type WindowMap = HashMap<HWND, @mut Window>;

fn window_key(_x: @@mut WindowMap) {
}

pub fn init_window_map() {
    let win_map = @@mut HashMap::new::<HWND, @mut Window>();
    unsafe { local_data::local_data_set(window_key, win_map); }
}

pub fn get_window_map() -> @mut WindowMap {
    let ret = unsafe { local_data::local_data_get(window_key) };
    match ret {
        Some(@wmap) => wmap,
        None => fail!("local_data() returned no map"),
    }
}

pub extern "stdcall" fn main_wnd_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    debug!("main_wnd_proc: hwnd %? / msg 0x%x / w %? / l %?", hwnd, msg as uint, w, l);
    let wmap = get_window_map();

    let mut window = {
        let null_hwnd = ptr::null::<*c_void>() as HWND;
        let null_win = wmap.pop(&null_hwnd);
        match null_win {
            Some(window) => {
                // hello newcomer.
                wmap.insert(hwnd, window);
                let mut window = window;
                window.set_hwnd(hwnd);
                window
            },
            None => {
                let win = wmap.find(&hwnd);
                match win {
                    Some(&window) => {
                        window
                    },
                    None => fail!("main_wnd_proc called for unknown hwnd"),
                }
            },
        }

    };

    window.wnd_proc(msg, w, l)
}

pub trait WindowUtil {
    pub fn register(&self, instance: HINSTANCE) -> bool;

    fn create(@mut self, instance: HINSTANCE, title: &str) -> bool;

    pub fn show(&self, cmd_show: int) -> bool;

    pub fn update(&self) -> bool;
}

impl<T: Window + 'static> WindowUtil for T {
    fn register(&self, instance: HINSTANCE) -> bool {
        do as_utf16_p(self.classname()) |clsname_p| {
            let wcex = WNDCLASSEX {
                cbSize: std::sys::size_of::<WNDCLASSEX>() as UINT,
                style: 0x0001 | 0x0002, // CS_HREDRAW | CS_VREDRAW
                lpfnWndProc: main_wnd_proc,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
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

    fn create(@mut self, instance: HINSTANCE, title: &str) -> bool {
        (&*self).register(instance);
        get_window_map().insert(self.hwnd(), self as @mut Window);

        let WS_OVERLAPPED = 0x00000000u32;
        let WS_CAPTION = 0x00C00000u32;
        let WS_SYSMENU = 0x00080000u32;
        let WS_THICKFRAME = 0x00040000u32;
        let WS_MINIMIZEBOX = 0x00020000u32;
        let WS_MAXIMIZEBOX = 0x00010000u32;
        let WS_OVERLAPPEDWINDOW = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU |
                WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

        let hwnd = unsafe {
            do as_utf16_p(self.classname()) |clsname_p| {
                do as_utf16_p(title) |title_p| {
                    let hwnd = user32::CreateWindowExW(
                        0, clsname_p, title_p, WS_OVERLAPPEDWINDOW as DWORD,
                        0 as c_int, 0 as c_int, 400 as c_int, 400 as c_int,
                        ptr::mut_null(), ptr::mut_null(), instance,
                        ptr::null::<*c_void>() as *mut c_void
                    );
                    hwnd
                }
            }
        };
        hwnd != ptr::mut_null()
    }

    pub fn show(&self, cmd_show: int) -> bool {
        unsafe { user32::ShowWindow(self.hwnd(), cmd_show as c_int) as bool }
    }

    pub fn update(&self) -> bool {
        unsafe { user32::UpdateWindow(self.hwnd()) as bool }
    }
}

pub trait DialogUtil {
    pub fn message_box(&self, msg: &str, title: &str);
}

impl<T: Window> DialogUtil for T {
    pub fn message_box(&self, msg: &str, title: &str) {
        do as_utf16_p(msg) |msg_p| {
            do as_utf16_p(title) |title_p| {
                unsafe {
                    user32::MessageBoxW(self.hwnd(), msg_p, title_p, 0u32);
                }
            }
        }
    }
}

pub trait OnCreate {
    fn on_create(&mut self, cs: &CREATESTRUCT) -> bool;
}

pub trait OnDestroy {
    fn on_destroy(&mut self);
}

pub trait OnPaint {
    fn on_paint(&mut self, dc: HDC);
}
