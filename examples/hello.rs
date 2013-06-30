extern mod win32;

use win32::window::*;
use win32::ll::*;

struct MainWindow {
    raw: HWND,
}

impl win32::window::Window for MainWindow {
    fn hwnd(&self) -> HWND {
        self.raw
    }
}

fn main() {
    let non = win32::window::null();
    non.message_box("Hello World", "hello world");
}
