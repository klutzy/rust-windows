// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use std;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use user32;
use winapi::{
    BOOL, CREATESTRUCTW, HBRUSH, HCURSOR, HICON, HMENU, HWND, INT, LPARAM, LRESULT, RECT, UINT,
    WNDCLASSEXW, WPARAM, c_int,
};

use wchar::{FromCU16Str,ToCU16Str};
use instance::Instance;
use resource::*;

pub struct WndClass {
    pub classname: String,
    pub style: usize,
    pub icon: Option<Image>,
    pub icon_small: Option<Image>,
    pub cursor: Option<Image>,
    pub background: HBRUSH,
    pub menu: MenuResource,
    pub cls_extra: isize,
    pub wnd_extra: isize,
}

impl WndClass {
    pub fn register(&self, instance: Instance) -> bool {
        self.menu.with_menu_p(|menu_p| {
            let clsname_u = self.classname.to_c_u16();
            let wcex = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
                style: self.style as UINT,
                lpfnWndProc: Some(main_wnd_proc),
                cbClsExtra: self.cls_extra as INT,
                cbWndExtra: self.wnd_extra as INT,
                hInstance: instance.instance,
                hIcon: self.icon.to_handle() as HICON,
                hCursor: self.cursor.to_handle() as HCURSOR,
                hbrBackground: self.background,
                lpszMenuName: menu_p,
                lpszClassName: clsname_u.as_ptr(),
                hIconSm: self.icon_small.to_handle() as HICON,
            };

            let res = unsafe { user32::RegisterClassExW(&wcex) };
            res != 0
        })
    }
}

pub struct WindowParams {
    pub window_name: String,
    pub style: u32,
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
    pub parent: Window,
    pub menu: HMENU,
    pub ex_style: u32,
}

/// Thin wrapper of `HWND` type
#[derive(PartialEq, Eq, Hash, Copy)]
pub struct Window {
    pub wnd: HWND,
}

// Sending across threads allows, for example, a worker thread to communicate
// with a UI thread via PostMessage.
unsafe impl Send for Window {}

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
            wnd: ptr::null_mut(),
        }
    }

    pub fn from_handle(handle: HWND) -> Window {
        Window {
            wnd: handle
        }
    }

    pub fn new(
        instance: Instance, wproc: Option<Box<WindowImpl + 'static>>, classname: &str, params: &WindowParams
    ) -> Option<Window> {
        KEY_INIT_WND.with(move |f| *f.borrow_mut() = wproc);

        let wnd = unsafe {
            let clsname_u = classname.to_c_u16();
            let title_u = params.window_name.to_c_u16();
            let wnd = user32::CreateWindowExW(
                params.ex_style, clsname_u.as_ptr(), title_u.as_ptr(), params.style,
                params.x as c_int, params.y as c_int,
                params.width as c_int, params.height as c_int,
                params.parent.wnd, params.menu, instance.instance,
                ptr::null_mut()
            );
            wnd
        };

        if wnd != ptr::null_mut() {
            Some(Window { wnd: wnd })
        } else {
            None
        }
    }

    pub fn show(&self, cmd_show: isize) -> bool {
        unsafe { user32::ShowWindow(self.wnd, cmd_show as c_int) == 0 }
    }

    pub fn show_async(&self, cmd_show: isize) -> bool {
        unsafe { user32::ShowWindowAsync(self.wnd, cmd_show as c_int) == 0 }
    }

    pub fn update(&self) -> bool {
        unsafe { user32::UpdateWindow(self.wnd) == 0 }
    }

    pub fn client_rect(&self) -> Option<RECT> {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let res = unsafe {
            user32::GetClientRect(self.wnd, &mut rect as *mut RECT)
        } != 0;
        match res {
            true => Some(rect),
            false => None,
        }
    }

    pub fn set_window_pos(
        &self, x: isize, y: isize, width: isize, height: isize, flags: UINT
    ) -> bool {
        // TODO: hwndInsertAfter
        unsafe {
            user32::SetWindowPos(
                self.wnd, ptr::null_mut(), x as c_int, y as c_int,
                width as c_int, height as c_int, flags
            ) != 0
        }
    }

    pub fn set_focus(&self) -> Window {
        unsafe {
            Window {
                wnd: user32::SetFocus(self.wnd)
            }
        }
    }

    pub fn send_message(&self, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            user32::SendMessageW(self.wnd, msg, wparam, lparam)
        }
    }

    pub fn post_message(&self, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> bool {
        1 == unsafe {
            user32::PostMessageW(self.wnd, msg, wparam, lparam)
        }
    }

    pub fn invalidate_rect(&self, rect: RECT, erase: bool) -> bool {
        1 == unsafe {
            user32::InvalidateRect(self.wnd, &rect, erase as BOOL)
        }
    }

    pub fn invalidate(&self, erase: bool) -> bool {
        1 == unsafe {
            user32::InvalidateRect(self.wnd, ptr::null(), erase as BOOL)
        }
    }

    pub fn get_window_text(&self) -> String {
        unsafe {
            let len = user32::GetWindowTextLengthW(self.wnd);
            let mut buf = vec![ 0u16; (len+1) as usize ];

            let read = user32::GetWindowTextW(self.wnd, buf.as_mut_ptr(), (len+1) );
            if read == len {
                match String::from_c_u16(&buf) {
                    None => String::new(),
                    Some(s) => s
                }
            } else {
                String::new()
            }
        }
    }

    pub fn set_window_text(&self, text: &str ) -> bool {
        let text_u = text.to_c_u16();
        1 == unsafe {
            user32::SetWindowTextW(self.wnd, text_u.as_ptr())
        }
    }
}

pub trait WindowImpl {
    fn wnd<'a>(&'a self) -> &'a Window;
    fn wnd_mut<'a>(&'a mut self) -> &'a mut Window;
    fn wnd_proc(&self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT;
}

/// A thread-local global map from windows to the above WindowImpl trait object
/// RefCell is necessary for mutability.
/// Rc is necessary so multiple main_wnd_procs on the same stack can reference it at once.
/// Box is necessary because WindowImpl is unsized, so can't be Rc'ed directly.
thread_local!(static KEY_WIN_MAP: RefCell<HashMap<Window, Rc<Box<WindowImpl + 'static>>>> = RefCell::new(HashMap::new()));

/// A thread-local global pointing to the initial window
thread_local!(static KEY_INIT_WND: RefCell<Option<Box<WindowImpl + 'static>>> = RefCell::new(None));

fn associate_window_impl(win: Window, wnd_impl: Box<WindowImpl + 'static>) {
    KEY_WIN_MAP.with(move |wmap_cell| {
        let mut wmap = wmap_cell.borrow_mut();
        wmap.insert(win, Rc::new(wnd_impl));
    });
}

fn promote_init_wnd(win: Window) {
    KEY_INIT_WND.with(move |maybe_initial_cell| {
        let mut maybe_initial = maybe_initial_cell.borrow_mut();
        if let Some(mut wnd_impl) = maybe_initial.take() {
            wnd_impl.wnd_mut().wnd = win.wnd;
            associate_window_impl(win, wnd_impl);
        }
    });
}

fn lookup_wnd_impl(wnd: HWND) -> Option<Rc<Box<WindowImpl + 'static>>> {
    KEY_WIN_MAP.with(|wmap_cell| {
        if let Some(wnd_impl) = wmap_cell.borrow().get(&Window{wnd: wnd}) {
            return Some(wnd_impl.clone());
        }
        return None;
    })
}

pub unsafe extern "system" fn main_wnd_proc(wnd: HWND,
                                            msg: UINT,
                                            w: WPARAM,
                                            l: LPARAM) -> LRESULT {
    promote_init_wnd(Window { wnd: wnd });

    if let Some(wnd_impl) = lookup_wnd_impl(wnd) {
        wnd_impl.wnd_proc(msg, w, l)
    } else {
        super::def_window_proc(wnd, msg, w, l)
    }
}


pub trait OnCreate {
    #[inline(always)]
    fn wm_create(&self, _wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let cs = unsafe {
            let pcs = std::mem::transmute::<LPARAM, *const CREATESTRUCTW>(lparam);
            &(*pcs)
        };
        let ret = self.on_create(cs);
        if ret {
            return 0 as LRESULT;
        } else {
            return -1 as LRESULT;
        }
    }

    fn on_create(&self, _cs: &CREATESTRUCTW) -> bool {
        true
    }
}

pub trait OnDestroy {
    #[inline(always)]
    fn wm_destroy(&self, _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        0 as LRESULT
    }

    fn on_destroy(&self) {
        unsafe {
            user32::PostQuitMessage(0 as c_int);
        }
    }
}

pub trait OnPaint {
    #[inline(always)]
    fn wm_paint(&self, _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        self.on_paint();
        0 as LRESULT
    }

    fn on_paint(&self) {
    }
}

pub trait OnSize {
    #[inline(always)]
    fn wm_size(&self, _wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let l = lparam as u32;
        let width = (l & 0xFFFF) as isize;
        let height = (l >> 16) as isize;
        self.on_size(width, height);
        0 as LRESULT
    }

    fn on_size(&self, _width: isize, _height: isize) {
    }
}

pub trait OnSetFocus {
    #[inline(always)]
    fn wm_set_focus(&self, wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        let w = Window::from_handle(wparam as HWND);
        self.on_set_focus(w);
        0 as LRESULT
    }

    fn on_set_focus(&self, _prev: Window) {
    }
}

pub trait OnLeftButtonDown {
    #[inline(always)]
    fn wm_left_button_down(&self, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let l = lparam as u32;
        let x = (l & 0xFFFF) as isize;
        let y = (l >> 16) as isize;
        let flags = wparam as u32;
        self.on_left_button_down(x, y, flags);
        0 as LRESULT
    }

    fn on_left_button_down(&self, _x: isize, _y: isize, _flags: u32) {
    }
}

pub trait OnLeftButtonUp {
    #[inline(always)]
    fn wm_left_button_up(&self, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let l = lparam as u32;
        let x = (l & 0xFFFF) as isize;
        let y = (l >> 16) as isize;
        let flags = wparam as u32;
        self.on_left_button_up(x, y, flags);
        0 as LRESULT
    }

    fn on_left_button_up(&self, _x: isize, _y: isize, _flags: u32) {
    }
}

pub trait OnKeyDown {
    #[inline(always)]
    fn wm_key_down(&self, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let keycode = wparam as u8;
        let flags = lparam as u32;
        return self.on_key_down(keycode, flags) as LRESULT;
    }

    fn on_key_down(&self, _keycode: u8, _flags: u32) -> bool {
        return false;
    }
}

pub trait OnKeyUp {
    #[inline(always)]
    fn wm_key_up(&self, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let keycode = wparam as u8;
        let flags = lparam as u32;
        return self.on_key_up(keycode, flags) as LRESULT;
    }

    fn on_key_up(&self, _keycode: u8, _flags: u32) -> bool {
        return false;
    }
}

pub trait OnEraseBackground {
    #[inline(always)]
    fn wm_erase_background(&self, _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        return self.on_erase_background() as LRESULT;
    }

    /// return true iff background does not need erasing.
    fn on_erase_background(&self) -> bool {
        false
    }
}

pub trait OnMessage {
    fn on_message(&self, _message: UINT, _wparam: WPARAM, _lparam: LPARAM) -> Option<LRESULT> {
        None
    }
}
