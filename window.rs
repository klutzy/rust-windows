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

pub fn as_utf16_p_or_null<T>(s: &Option<~str>, f: &fn(*u16) -> T) -> T {
    match s {
        &None => f(ptr::null()),
        &Some(ref s) => as_utf16_p(*s, f),
    }
}

pub struct Instance {
    instance: HINSTANCE
}

pub struct WndClass {
    classname: ~str,
    style: uint,
    icon: HICON,
    icon_small: HICON,
    cursor: Cursor,
    background: HBRUSH,
    menu_name: Option<~str>,
    cls_extra: int,
    wnd_extra: int,
}

impl WndClass {
    #[fixed_stack_segment]
    pub fn register(&self, instance: Instance) -> bool {
        do as_utf16_p_or_null(&self.menu_name) |menuname_p| {
            do as_utf16_p(self.classname) |clsname_p| {
                let wcex = WNDCLASSEX {
                    cbSize: std::sys::size_of::<WNDCLASSEX>() as UINT,
                    style: self.style as UINT,
                    lpfnWndProc: main_wnd_proc as *c_void,
                    cbClsExtra: self.cls_extra as INT,
                    cbWndExtra: self.wnd_extra as INT,
                    hInstance: instance.instance,
                    hIcon: self.icon,
                    hCursor: self.cursor.cursor,
                    hbrBackground: self.background,
                    lpszMenuName: menuname_p,
                    lpszClassName: clsname_p,
                    hIconSm: self.icon_small,
                };

                let res = unsafe { RegisterClassExW(&wcex) };
                res != 0
            }
        }
    }
}

impl Instance {
    #[fixed_stack_segment]
    pub fn main_instance() -> Instance {
        Instance {
            instance: unsafe { GetModuleHandleW(ptr::null()) as HINSTANCE },
        }
    }
}

pub struct Cursor {
    cursor: HCURSOR,
}

impl Cursor {
    pub fn null() -> Cursor {
        Cursor {
            cursor: ptr::mut_null(),
        }
    }

    #[fixed_stack_segment]
    pub fn load_resource(id: int) -> Cursor {
        let c = unsafe {
            LoadImageW(ptr::mut_null(), std::cast::transmute(id), 2, 0, 0, 0x8000)
        };

        Cursor {
            cursor: c,
        }
    }
}

pub struct WindowParams {
    class: WndClass,
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
    pub fn create(instance: Instance, proc: ~WndProc, params: &WindowParams) -> Option<Window> {
        let res = params.class.register(instance);
        if res {
            local_data::set(key_init_wnd, proc);
        } else {
            return None;
        }

        let wnd = unsafe {
            do as_utf16_p(params.class.classname) |clsname_p| {
                do as_utf16_p(params.window_name) |title_p| {
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
                    MessageBoxW(self.wnd, msg_p, title_p, 0u32);
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

        let dc = unsafe { BeginPaint(self.wnd().wnd, &ps) };
        (dc, ps)
    }

    #[fixed_stack_segment]
    fn end_paint(&self, ps: &PAINTSTRUCT) {
        unsafe { EndPaint(self.wnd().wnd, ps) };
    }

    #[fixed_stack_segment]
    fn text_out(&self, dc: HDC, x: int, y: int, s: &str) -> bool {
        let mut s16 = s.to_utf16();
        s16.push(0u16);
        do s16.as_mut_buf |buf, len| {
            let len = len - 1;
            let ret = unsafe {
                TextOutW(dc, x as c_int, y as c_int, buf, len as i32)
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
            PostQuitMessage(0 as c_int);
        }
    }
}

pub trait OnPaint {
    fn on_paint(&self, _dc: HDC) {
    }
}
