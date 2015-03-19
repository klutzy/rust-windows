// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(core, exit_status)]

#[macro_use]
extern crate log;

extern crate winapi;

#[macro_use]
extern crate "rust-windows" as windows;

use std::ptr;
use std::cell::RefCell;
use std::default::Default;
use std::env;

use winapi::{UINT, HBRUSH, CREATESTRUCTW};

use windows::main_window_loop;
use windows::instance::Instance;
use windows::resource::*;
use windows::window::{WindowImpl, Window, WndClass, WindowParams};
use windows::window::{OnCreate, OnSize, OnDestroy, OnPaint, OnFocus};
use windows::window;
use windows::gdi::PaintDc;
use windows::font::Font;
use windows::font;

// TODO duplicate of hello.rc
static IDI_ICON: isize = 0x101;
static MENU_MAIN: isize = 0x201;
//static MENU_NEW: isize = 0x202;
//static MENU_EXIT: isize = 0x203;

struct MainFrame {
    win: Window,
    title: String,
    text_height: isize,
    edit: RefCell<Option<Window>>,
    font: RefCell<Option<Font>>,
}

wnd_proc!(MainFrame, win, WM_CREATE, WM_DESTROY, WM_SIZE, WM_SETFOCUS, WM_PAINT);

impl OnCreate for MainFrame {
    fn on_create(&self, _cs: &CREATESTRUCTW) -> bool {
        let rect = self.win.client_rect().unwrap();
        let params = WindowParams {
            window_name: "Hello World".to_string(),
            style: window::WS_CHILD | window::WS_VISIBLE | window::WS_BORDER | window::WS_VSCROLL |
                window::ES_AUTOVSCROLL | window::ES_MULTILINE | window::ES_NOHIDESEL,
            x: 0,
            y: self.text_height,
            width: rect.right as isize,
            height: rect.bottom as isize - self.text_height,
            parent: self.win,
            menu: ptr::null_mut(),
            ex_style: 0,
        };
        let edit = Window::new(Instance::main_instance(), None, "EDIT", &params);
        match edit {
            None => false,
            Some(e) => {
                let font_attr = Default::default();
                let font = font::Font::new(&font_attr);
                match font {
                    None => false,
                    Some(f) => {
                        static WM_SETFONT: UINT = 0x0030;
                        unsafe {
                            e.send_message(WM_SETFONT, std::mem::transmute(f.font), 0);
                        }
                        *self.edit.borrow_mut() = Some(e);
                        *self.font.borrow_mut() = Some(f);
                        true
                    }
                }
            }
        }
    }
}

impl OnSize for MainFrame {
    fn on_size(&self, width: isize, height: isize) {
        // SWP_NOOWNERZORDER | SWP_NOZORDER
        let h = self.text_height;
        self.edit.borrow().expect("edit is empty")
            .set_window_pos(0, h, width, height - h, 0x200 | 0x4);
    }
}

impl OnDestroy for MainFrame {}

impl OnPaint for MainFrame {
    fn on_paint(&self) {
        let font = self.font.borrow();
        let pdc = PaintDc::new(self).expect("Paint DC");
        pdc.dc.select_font(&font.expect("font is empty"));
        pdc.dc.text_out(0, 0, self.title.as_slice());
    }
}

impl OnFocus for MainFrame {
    fn on_focus(&self, _w: Window) {
        self.edit.borrow().expect("edit is empty").set_focus();
    }
}

impl MainFrame {
    fn new(instance: Instance, title: String, text_height: isize) -> Option<Window> {
        let icon = Image::load_resource(instance, IDI_ICON, ImageType::IMAGE_ICON, 0, 0);
        let wnd_class = WndClass {
            classname: "MainFrame".to_string(),
            style: 0x0001 | 0x0002, // CS_HREDRAW | CS_VREDRAW
            icon: icon,
            icon_small: None,
            cursor: Image::load_cursor_resource(32514), // hourglass
            background: (5 + 1) as HBRUSH,
            menu: MenuResource::MenuId(MENU_MAIN),
            cls_extra: 0,
            wnd_extra: 0,
        };
        let res = wnd_class.register(instance);
        if !res {
            return None;
        }

        let wproc = Box::new(MainFrame {
            win: Window::null(),
            title: title.clone(),
            text_height: text_height,
            edit: RefCell::new(None),
            font: RefCell::new(None),
        });

        let win_params = WindowParams {
            window_name: title,
            style: window::WS_OVERLAPPEDWINDOW,
            x: 0,
            y: 0,
            width: 400,
            height: 400,
            parent: Window::null(),
            menu: ptr::null_mut(),
            ex_style: 0,
        };

        Window::new(instance, Some(wproc as Box<WindowImpl + 'static>),
                    wnd_class.classname.as_slice(), &win_params)
    }
}

fn main() {
    let instance = Instance::main_instance();
    let main = MainFrame::new(instance, "Hello Rust".to_string(), 20);
    let main = main.unwrap();

    main.show(1);
    main.update();

    let exit_code = main_window_loop();
    env::set_exit_status(exit_code as i32);
}
