#![macro_escape]

#[macro_export]
macro_rules! wnd_proc_thunk(
    (WM_CREATE) => (
        if msg == 0x0001 { // WM_CREATE
            let cs = unsafe {
                let pcs = ::std::mem::transmute::<::windows::ll::types::LPARAM,
                                                   *::windows::ll::all::CREATESTRUCT>(l);
                &(*pcs)
            };
            let ret = self.on_create(cs);
            if ret {
                return 0 as ::windows::ll::types::LRESULT;
            } else {
                return -1 as ::windows::ll::types::LRESULT;
            }
        }
    );
    (WM_DESTROY) => (
        if msg == 0x0002 { // WM_DESTROY
            self.on_destroy();
            return 0 as ::windows::ll::types::LRESULT;
        }
    );
    (WM_SIZE) => (
        if msg == 0x0005 { // WM_SIZE
            let l = l as u32;
            let width = (l & 0xFFFF) as int;
            let height = (l >> 16) as int;
            self.on_size(width, height);
            return 0 as ::windows::ll::types::LRESULT;
        }
    );
    (WM_SETFOCUS) => (
        if msg == 0x0007 { // WM_SETFOCUS
            let w = ::windows::window::Window { wnd: w as ::windows::ll::types::HWND };
            self.on_focus(w);
            return 0 as ::windows::ll::types::LRESULT;
        }
    );
    (WM_PAINT) => (
        if msg == 0x000F { // WM_PAINT
            self.on_paint();
            return 0 as ::windows::ll::types::LRESULT;
        }
    );
)

#[macro_export]
macro_rules! wnd_proc(
    ($wnd:ident, $win:ident, $($msg:ident),+) => (

    impl ::windows::window::WindowImpl for $wnd {
        fn wnd<'a>(&'a self) -> &'a ::windows::window::Window {
            &self.$win
        }

        fn wnd_mut<'a>(&'a mut self) -> &'a mut ::windows::window::Window {
            &mut self.$win
        }

        fn wnd_proc(&self, msg: ::windows::ll::types::UINT, w: ::windows::ll::types::WPARAM,
                    l: ::windows::ll::types::LPARAM) -> ::windows::ll::types::LRESULT {
            $(
                wnd_proc_thunk!($msg)
            )+
            ::windows::def_window_proc(self.wnd().wnd, msg, w, l)
        }
    }

    )
)

#[macro_export]
macro_rules! guid(
    ($d1:expr, $d2:expr, $d3:expr,
     $d41:expr, $d42:expr, $d43:expr, $d44:expr, $d45:expr, $d46:expr, $d47:expr, $d48:expr) => (
        ::windows::com::GUID {
            data1: $d1,
            data2: $d2,
            data3: $d3,
            data4: [$d41, $d42, $d43, $d44, $d45, $d46, $d47, $d48],
        }
    )
)
