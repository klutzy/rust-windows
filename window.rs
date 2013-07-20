use std::local_data;
use std::hashmap::HashMap;
use std::ptr;
use std::os::win32::as_utf16_p;
use ll::*;

pub trait Window {
    fn hwnd(&self) -> HWND;
    fn set_hwnd(&mut self, HWND);
    fn wnd_proc(&mut self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT;
}

pub struct EmptyWindow;

impl Window for EmptyWindow {
    fn hwnd(&self) -> HWND {
        ptr::null()
    }

    fn set_hwnd(&mut self, _hwnd: HWND) {
        fail!("set_hwnd on EmptyWindow");
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
        (*self).hwnd()
    }

    fn set_hwnd(&mut self, hwnd: HWND) {
        (*self).set_hwnd(hwnd)
    }

    fn wnd_proc(&mut self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
        (*self).wnd_proc(msg, w, l)
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
    pub fn show(&self, cmd_show: int) -> bool;

    pub fn update(&self) -> bool;
}

impl<T: Window> WindowUtil for T {
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
