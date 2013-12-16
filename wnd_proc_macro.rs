#[macro_escape];

macro_rules! wnd_proc_thunk(
    (WM_CREATE) => (
        if msg == 0x0001 { // WM_CREATE
            let cs = unsafe {
                let pcs = std::cast::transmute::<LPARAM, *CREATESTRUCT>(l);
                &(*pcs)
            };
            let ret = self.on_create(cs);
            return if ret { 0 as LRESULT } else { -1 as LRESULT };
        }
    );
    (WM_DESTROY) => (
        if msg == 0x0002 { // WM_DESTROY
            self.on_destroy();
            return 0 as LRESULT;
        }
    );
    (WM_SIZE) => (
        if msg == 0x0005 { // WM_SIZE
            let l = l as u32;
            let width = (l & 0xFFFF) as int;
            let height = (l >> 16) as int;
            self.on_size(width, height);
            return 0 as LRESULT;
        }
    );
    (WM_SETFOCUS) => (
        if msg == 0x0007 { // WM_SETFOCUS
            let w = Window { wnd: w as HWND };
            self.on_focus(w);
            return 0 as LRESULT;
        }
    );
    (WM_PAINT) => (
        if msg == 0x000F { // WM_PAINT
            self.on_paint();
            return 0 as LRESULT;
        }
    );
)

macro_rules! wnd_proc(
    ($wnd:ident, $win:ident, $($msg:ident),+) => (

    impl WindowImpl for $wnd {
        fn wnd<'a>(&'a self) -> &'a Window {
            &self.$win
        }

        fn wnd_mut<'a>(&'a mut self) -> &'a mut Window {
            &mut self.$win
        }

        fn wnd_proc(&self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
            $(
                wnd_proc_thunk!($msg)
            )+
            windows::def_window_proc(self.wnd().wnd, msg, w, l)
        }
    }

    )
)
