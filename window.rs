use std::ptr;
use std::owned::Box;
use std;
use std::cell::RefCell;

use collections::HashMap;

use libc::{c_int, c_void};
use ll::all::{WNDCLASSEX, CREATESTRUCT};
use ll::types::{HWND, INT, RECT, LPARAM, UINT, WPARAM, LRESULT, HMENU, HBRUSH};

use wchar::ToCU16Str;
use instance::Instance;
use resource::*;

pub struct WndClass {
    pub classname: String,
    pub style: uint,
    pub icon: Option<Image>,
    pub icon_small: Option<Image>,
    pub cursor: Option<Image>,
    pub background: HBRUSH,
    pub menu: MenuResource,
    pub cls_extra: int,
    pub wnd_extra: int,
}

impl WndClass {
    pub fn register(&self, instance: Instance) -> bool {
        self.menu.with_menu_p(|menu_p| {
            let clsname_u = self.classname.to_c_u16();
            let wcex = WNDCLASSEX {
                cbSize: std::mem::size_of::<WNDCLASSEX>() as UINT,
                style: self.style as UINT,
                lpfnWndProc: main_wnd_proc as *c_void,
                cbClsExtra: self.cls_extra as INT,
                cbWndExtra: self.wnd_extra as INT,
                hInstance: instance.instance,
                hIcon: self.icon.to_handle(),
                hCursor: self.cursor.to_handle(),
                hbrBackground: self.background,
                lpszMenuName: menu_p,
                lpszClassName: clsname_u.as_ptr(),
                hIconSm: self.icon_small.to_handle(),
            };

            let res = unsafe { super::ll::all::RegisterClassExW(&wcex) };
            res != 0
        })
    }
}

pub static WS_BORDER: u32 = 0x800000;
pub static WS_CAPTION: u32 = 0xc00000;
pub static WS_CHILD: u32 = 0x40000000;
pub static WS_CHILDWINDOW: u32 = 0x40000000;
pub static WS_CLIPCHILDREN: u32 = 0x2000000;
pub static WS_CLIPSIBLINGS: u32 = 0x4000000;
pub static WS_DISABLED: u32 = 0x8000000;
pub static WS_DLGFRAME: u32 = 0x400000;
pub static WS_GROUP: u32 = 0x20000;
pub static WS_HSCROLL: u32 = 0x100000;
pub static WS_ICONIC: u32 = 0x20000000;
pub static WS_MAXIMIZE: u32 = 0x1000000;
pub static WS_MAXIMIZEBOX: u32 = 0x10000;
pub static WS_MINIMIZE: u32 = 0x20000000;
pub static WS_MINIMIZEBOX: u32 = 0x20000;
pub static WS_OVERLAPPED: u32 = 0;
pub static WS_OVERLAPPEDWINDOW: u32 = 0xcf0000;
pub static WS_POPUP: u32 = 0x80000000;
pub static WS_POPUPWINDOW: u32 = 0x80880000;
pub static WS_SIZEBOX: u32 = 0x40000;
pub static WS_SYSMENU: u32 = 0x80000;
pub static WS_TABSTOP: u32 = 0x10000;
pub static WS_THICKFRAME: u32 = 0x40000;
pub static WS_TILED: u32 = 0;
pub static WS_TILEDWINDOW: u32 = 0xcf0000;
pub static WS_VISIBLE: u32 = 0x10000000;
pub static WS_VSCROLL: u32 = 0x200000;

pub static ES_AUTOHSCROLL: u32 = 128;
pub static ES_AUTOVSCROLL: u32 = 64;
pub static ES_CENTER: u32 = 1;
pub static ES_LEFT: u32 = 0;
pub static ES_LOWERCASE: u32 = 16;
pub static ES_MULTILINE: u32 = 4;
pub static ES_NOHIDESEL: u32 = 256;
pub static ES_NUMBER: u32 = 0x2000;
pub static ES_OEMCONVERT: u32 = 0x400;
pub static ES_PASSWORD: u32 = 32;
pub static ES_READONLY: u32 = 0x800;
pub static ES_RIGHT: u32 = 2;
pub static ES_UPPERCASE: u32 = 8;
pub static ES_WANTRETURN: u32 = 4096;

pub struct WindowParams {
    pub window_name: String,
    pub style: u32,
    pub x: int,
    pub y: int,
    pub width: int,
    pub height: int,
    pub parent: Window,
    pub menu: HMENU,
    pub ex_style: u32,
}

#[deriving(Eq, TotalEq, Hash)]
pub struct Window {
    pub wnd: HWND,
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

    pub fn new(
        instance: Instance, wproc: Option<Box<WindowImpl:Send>>, classname: &str, params: &WindowParams
    ) -> Option<Window> {
        key_init_wnd.replace(wproc);

        let wnd = unsafe {
            let clsname_u = classname.to_c_u16();
            let title_u = params.window_name.to_c_u16();
            let wnd = super::ll::all::CreateWindowExW(
                params.ex_style, clsname_u.as_ptr(), title_u.as_ptr(), params.style,
                params.x as c_int, params.y as c_int,
                params.width as c_int, params.height as c_int,
                params.parent.wnd, params.menu, instance.instance,
                ptr::mut_null()
            );
            wnd
        };

        if wnd != ptr::mut_null() {
            Some(Window { wnd: wnd })
        } else {
            None
        }
    }

    pub fn show(&self, cmd_show: int) -> bool {
        unsafe { super::ll::all::ShowWindow(self.wnd, cmd_show as c_int) == 0 }
    }

    pub fn show_async(&self, cmd_show: int) -> bool {
        unsafe { super::ll::all::ShowWindowAsync(self.wnd, cmd_show as c_int) == 0 }
    }

    pub fn update(&self) -> bool {
        unsafe { super::ll::all::UpdateWindow(self.wnd) == 0 }
    }

    pub fn client_rect(&self) -> Option<RECT> {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let res = unsafe {
            super::ll::all::GetClientRect(self.wnd, &mut rect as *mut RECT)
        } != 0;
        match res {
            true => Some(rect),
            false => None,
        }
    }

    pub fn set_window_pos(
        &self, x: int, y: int, width: int, height: int, flags: UINT
    ) -> bool {
        // TODO: hwndInsertAfter
        unsafe {
            super::ll::all::SetWindowPos(
                self.wnd, ptr::mut_null(), x as c_int, y as c_int,
                width as c_int, height as c_int, flags
            ) != 0
        }
    }

    pub fn set_focus(&self) -> Window {
        unsafe {
            Window {
                wnd: super::ll::all::SetFocus(self.wnd)
            }
        }
    }

    pub fn send_message(&self, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            super::ll::all::SendMessageW(self.wnd, msg, wparam, lparam)
        }
    }
}

pub trait WindowImpl {
    fn wnd<'a>(&'a self) -> &'a Window;
    fn wnd_mut<'a>(&'a mut self) -> &'a mut Window;
    fn wnd_proc(&self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT;
}

local_data_key!(pub key_win_map: RefCell<HashMap<Window, Box<WindowImpl:Send>>>)
local_data_key!(pub key_init_wnd: Box<WindowImpl:Send>)

pub fn init_window_map() {
    let win_map = HashMap::new();
    let _old_map = key_win_map.replace(Some(RefCell::new(win_map)));
    debug_assert!(_old_map.is_none());
}

pub extern "system" fn main_wnd_proc(wnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    debug!("main_wnd_proc: wnd {:?} / msg 0x{:x} / w {:?} / l {:?}", wnd, msg as uint, w, l);
    let win = Window { wnd: wnd };
    let null_proc = key_init_wnd.replace(None);
    match null_proc {
        Some(mut nproc) => {
            // hello newcomer.
            nproc.wnd_mut().wnd = wnd;
            let wmap = key_win_map.get().expect("key_win_map null");
            (*wmap).borrow_mut().insert(win, nproc);
        },
        None => {}
    };

    let wmap = key_win_map.get().expect("key_win_map null");
    let wmap = wmap.borrow();
    let wproc = {
        wmap.find(&win).unwrap()
    };
    wproc.wnd_proc(msg, w, l)
}

pub trait OnCreate {
    fn on_create(&self, _cs: &CREATESTRUCT) -> bool {
        true
    }
}

pub trait OnDestroy {
    fn on_destroy(&self) {
        unsafe {
            super::ll::all::PostQuitMessage(0 as c_int);
        }
    }
}

pub trait OnPaint {
    fn on_paint(&self) {
    }
}

pub trait OnSize {
    fn on_size(&self, _width: int, _height: int) {
    }
}

pub trait OnFocus {
    fn on_focus(&self, _prev: Window) {
    }
}
