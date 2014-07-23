#![macro_escape]

#[macro_export]
macro_rules! wnd_proc_thunk(
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_CREATE) => (
        if $msg == 0x0001 { // WM_CREATE
            let cs = unsafe {
                let pcs = ::std::mem::transmute::<::windows::ll::types::LPARAM,
                                                   *const ::windows::ll::all::CREATESTRUCT>($l);
                &(*pcs)
            };
            let ret = $self_.on_create(cs);
            if ret {
                return 0 as ::windows::ll::types::LRESULT;
            } else {
                return -1 as ::windows::ll::types::LRESULT;
            }
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_DESTROY) => (
        if $msg == 0x0002 { // WM_DESTROY
            $self_.on_destroy();
            return 0 as ::windows::ll::types::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_SIZE) => (
        if $msg == 0x0005 { // WM_SIZE
            let l = $l as u32;
            let width = (l & 0xFFFF) as int;
            let height = (l >> 16) as int;
            $self_.on_size(width, height);
            return 0 as ::windows::ll::types::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_SETFOCUS) => (
        if $msg == 0x0007 { // WM_SETFOCUS
            let w = ::windows::window::Window { wnd: $w as ::windows::ll::types::HWND };
            $self_.on_focus(w);
            return 0 as ::windows::ll::types::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_PAINT) => (
        if $msg == 0x000F { // WM_PAINT
            $self_.on_paint();
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
                wnd_proc_thunk!(self, msg, w, l, $msg)
            )+
            ::windows::def_window_proc(self.wnd().wnd, msg, w, l)
        }
    }

    )
)
