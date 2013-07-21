extern mod win32;

use std::ptr;

use win32::window::*;
use win32::ll::*;

struct MainWindow {
    raw: HWND,
    title: ~str,
}

impl win32::window::Window for MainWindow {
    fn hwnd(&self) -> HWND {
        self.raw
    }

    fn set_hwnd(&mut self, hwnd: HWND) {
        self.raw = hwnd;
    }

    fn classname<'s>(&'s self) -> &'s str {
        "MainWindow"
    }

    fn wnd_proc(&mut self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
        if msg == 0x0001 { // WM_CREATE
            return 0 as LRESULT;
        }
        if msg == 0x0002 { // WM_DESTROY
            unsafe {
                user32::PostQuitMessage(0 as c_int);
            }
            return 0 as LRESULT;
        }
        if msg == 0x000F { // WM_PAINT
            let rgb_res: [BYTE, ..32] = [0 as BYTE, ..32];
            let ps = PAINTSTRUCT {
                hdc: ptr::null(),
                fErase: 0 as BOOL,
                rcPaint: RECT {
                    left: 0 as LONG, top: 0 as LONG,
                    right: 0 as LONG, bottom: 0 as LONG
                },
                fRestore: 0 as BOOL,
                fIncUpdate: 0 as BOOL,
                rgbReserved: &rgb_res,
            };
            unsafe {
                let dc = user32::BeginPaint(self.hwnd(), &ps);

                let hello = "hello world";
                let mut hello_p = hello.to_utf16();
                hello_p.push(0u16);
                do std::vec::as_mut_buf(hello_p) |buf, len| {
                    let len = len - 1;
                    gdi32::TextOutW(dc, 0 as c_int, 0 as c_int, buf, len as i32);
                }
                user32::EndPaint(self.hwnd(), &ps);
            }

            return 0 as LRESULT;
        }
        unsafe { user32::DefWindowProcW(self.hwnd(), msg, w, l) }
    }
}

impl MainWindow {
    fn new(instance: HINSTANCE) -> @mut Window {
        let window = @mut MainWindow {
            raw: ptr::null(),
            title: ~"Hello",
        };

        window.create(instance, window.title);
        window as @mut Window
    }
}

fn main() {
    init_window_map();

    let instance = unsafe {
        kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE
    };
    let main: @mut Window = MainWindow::new(instance);

    main.show(1);
    main.update();

    let msg = MSG {
        hwnd: ptr::null(),
        message: 0 as UINT,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT { x: 0 as LONG, y: 0 as LONG },
    };
    loop {
        let ret = unsafe {
            user32::GetMessageW(&msg as *MSG, ptr::null(),
                    0 as UINT, 0 as UINT)
        };

        if ret == 0 {
            let exit_code = msg.wParam;
            std::os::set_exit_status(exit_code as int);
            return;
        }
        else {
            unsafe {
                user32::TranslateMessage(&msg as *MSG);
                user32::DispatchMessageW(&msg as *MSG);
            }
        }
    }
}
