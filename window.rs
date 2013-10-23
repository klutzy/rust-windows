use std::local_data;
use std::hashmap::HashMap;
use std::ptr;
use std;

use ll::*;
use wchar::*;
use instance::Instance;
use resource::*;

pub struct WndClass {
    classname: ~str,
    style: uint,
    icon: Option<Image>,
    icon_small: Option<Image>,
    cursor: Option<Image>,
    background: HBRUSH,
    menu: MenuResource,
    cls_extra: int,
    wnd_extra: int,
}

impl WndClass {
    #[fixed_stack_segment]
    pub fn register(&self, instance: Instance) -> bool {
        do self.menu.with_menu_p |menu_p| {
            do with_utf16_p(self.classname) |clsname_p| {
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
                    lpszClassName: clsname_p,
                    hIconSm: self.icon_small.to_handle(),
                };

                let res = unsafe { RegisterClassExW(&wcex) };
                res != 0
            }
        }
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
    window_name: ~str,
    style: u32,
    x: int,
    y: int,
    width: int,
    height: int,
    parent: Window,
    menu: HMENU,
    ex_style: u32,
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
    pub fn new(
        instance: Instance, proc: Option<~WindowImpl>, classname: &str, params: &WindowParams
    ) -> Option<Window> {
        match proc {
            Some(proc) => {
                local_data::set(key_init_wnd, proc);
            },
            None => {},
        }

        let wnd = unsafe {
            do with_utf16_p(classname) |clsname_p| {
                do with_utf16_p(params.window_name) |title_p| {
                    let wnd = CreateWindowExW(
                        params.ex_style, clsname_p, title_p, params.style,
                        params.x as c_int, params.y as c_int,
                        params.width as c_int, params.height as c_int,
                        params.parent.wnd, params.menu, instance.instance,
                        ptr::mut_null()
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
        unsafe { ShowWindow(self.wnd, cmd_show as c_int) == 0 }
    }

    #[fixed_stack_segment]
    pub fn update(&self) -> bool {
        unsafe { UpdateWindow(self.wnd) == 0 }
    }

    #[fixed_stack_segment]
    pub fn client_rect(&self) -> Option<RECT> {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let res = unsafe {
            GetClientRect(self.wnd, &mut rect as *mut RECT)
        } != 0;
        match res {
            true => Some(rect),
            false => None,
        }
    }

    #[fixed_stack_segment]
    pub fn set_window_pos(
        &self, x: int, y: int, width: int, height: int, flags: UINT
    ) -> bool {
        // TODO: hwndInsertAfter
        unsafe {
            SetWindowPos(
                self.wnd, ptr::mut_null(), x as c_int, y as c_int,
                width as c_int, height as c_int, flags
            ) != 0
        }
    }

    #[fixed_stack_segment]
    pub fn set_focus(&self) -> Window {
        unsafe {
            Window {
                wnd: SetFocus(self.wnd)
            }
        }
    }

    #[fixed_stack_segment]
    pub fn send_message(&self, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            SendMessageW(self.wnd, msg, wparam, lparam)
        }
    }
}

pub trait WindowImpl {
    fn wnd<'a>(&'a self) -> &'a Window;
    fn wnd_mut<'a>(&'a mut self) -> &'a mut Window;
    fn wnd_proc(&self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT;
}

pub type WindowMap = HashMap<Window, ~WindowImpl>;

local_data_key!(pub key_win_map: WindowMap)
local_data_key!(pub key_init_wnd: ~WindowImpl)

pub fn init_window_map() {
    let win_map: WindowMap = HashMap::new();
    local_data::set(key_win_map, win_map);
}

pub extern "stdcall" fn main_wnd_proc(wnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    debug!("main_wnd_proc: wnd {:?} / msg 0x{:x} / w {:?} / l {:?}", wnd, msg as uint, w, l);
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
        do with_utf16_p(msg) |msg_p| {
            do with_utf16_p(title) |title_p| {
                unsafe {
                    MessageBoxW(self.wnd, msg_p, title_p, 0u32);
                }
            }
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
            PostQuitMessage(0 as c_int);
        }
    }
}

pub trait OnPaint {
    fn on_paint(&self, _dc: HDC) {
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
