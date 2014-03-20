#[feature(globs, macro_rules, phase)];

#[phase(syntax, link)] extern crate log;

extern crate windows = "rust-windows";

use std::ptr;
use std::cell::RefCell;
use std::default::Default;

use windows::main_window_loop;
use windows::ll::*;
use windows::instance::*;
use windows::resource::*;
use windows::window::*;
use windows::gdi::PaintDc;
use windows::font::Font;
use windows::font;

#[path = "../wnd_proc_macro.rs"]
mod macro;

// TODO duplicate of hello.rc
static IDI_ICON: int = 0x101;
static MENU_MAIN: int = 0x201;
//static MENU_NEW: int = 0x202;
//static MENU_EXIT: int = 0x203;

struct MainFrame {
    win: Window,
    title: ~str,
    text_height: int,
    edit: RefCell<Option<Window>>,
    font: RefCell<Option<Font>>,
}

wnd_proc!(MainFrame, win, WM_CREATE, WM_DESTROY, WM_SIZE, WM_SETFOCUS, WM_PAINT)

impl OnCreate for MainFrame {
    fn on_create(&self, _cs: &CREATESTRUCT) -> bool {
        let rect = self.win.client_rect().unwrap();
        let params = WindowParams {
            window_name: ~"Hello World",
            style: WS_CHILD | WS_VISIBLE | WS_BORDER | WS_VSCROLL |
                ES_AUTOVSCROLL | ES_MULTILINE | ES_NOHIDESEL,
            x: 0,
            y: self.text_height,
            width: rect.right as int,
            height: rect.bottom as int - self.text_height,
            parent: self.win,
            menu: ptr::mut_null(),
            ex_style: 0,
        };
        let edit = Window::new(Instance::main_instance(), None, "EDIT", &params);
        match edit {
            None => false,
            Some(e) => {
                let font_attr = Default::default();
                let font = font::Font::new(&font_attr);
                debug!("font: {:?}", font);
                match font {
                    None => false,
                    Some(f) => {
                        static WM_SETFONT: UINT = 0x0030;
                        unsafe {
                            e.send_message(WM_SETFONT, std::cast::transmute(f.font), 0);
                        }
                        self.edit.with_mut(|r| {*r = Some(e);});
                        self.font.with_mut(|r| {*r = Some(f);});
                        true
                    }
                }
            }
        }
    }
}

impl OnSize for MainFrame {
    fn on_size(&self, width: int, height: int) {
        self.edit.with(|edit| {
            // SWP_NOOWNERZORDER | SWP_NOZORDER
            let h = self.text_height;
            edit.unwrap().set_window_pos(0, h, width, height - h, 0x200 | 0x4);
        })
    }
}

impl OnDestroy for MainFrame {}

impl OnPaint for MainFrame {
    fn on_paint(&self) {
        self.font.with(|font| {
            let pdc = PaintDc::new(self).expect("Paint DC");
            pdc.dc.select_font(&font.unwrap());
            pdc.dc.text_out(0, 0, self.title);
        })
    }
}

impl OnFocus for MainFrame {
    fn on_focus(&self, _w: Window) {
        self.edit.with(|edit| {
            edit.unwrap().set_focus();
        })
    }
}

impl MainFrame {
    fn new(instance: Instance, title: ~str, text_height: int) -> Option<Window> {
        let icon = Image::load_resource(instance, IDI_ICON, IMAGE_ICON, 0, 0);
        let wnd_class = WndClass {
            classname: ~"MainFrame",
            style: 0x0001 | 0x0002, // CS_HREDRAW | CS_VREDRAW
            icon: icon,
            icon_small: None,
            cursor: Image::load_cursor_resource(32514), // hourglass
            background: (5 + 1) as HBRUSH,
            menu: MenuId(MENU_MAIN),
            cls_extra: 0,
            wnd_extra: 0,
        };
        let res = wnd_class.register(instance);
        if !res {
            return None;
        }

        let wproc = ~MainFrame {
            win: Window::null(),
            title: title.clone(),
            text_height: text_height,
            edit: RefCell::new(None),
            font: RefCell::new(None),
        };

        let win_params = WindowParams {
            window_name: title,
            style: WS_OVERLAPPEDWINDOW,
            x: 0,
            y: 0,
            width: 400,
            height: 400,
            parent: Window::null(),
            menu: ptr::mut_null(),
            ex_style: 0,
        };

        Window::new(instance, Some(wproc as ~WindowImpl), wnd_class.classname, &win_params)
    }
}

fn main() {
    init_window_map();

    let instance = Instance::main_instance();
    let main = MainFrame::new(instance, ~"Hello Rust", 20);
    let main = main.unwrap();

    main.show(1);
    main.update();

    let exit_code = main_window_loop();
    std::os::set_exit_status(exit_code as int);
}
