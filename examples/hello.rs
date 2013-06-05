extern mod win32;

fn main() {
    let non = win32::window::Window::null();
    non.message_box("Hello World", "hello world");
}
