// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! wnd_proc_thunk(
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_CREATE) => (
        if $msg == 0x0001 { // WM_CREATE
            let cs = unsafe {
                let pcs = ::std::mem::transmute::<::winapi::LPARAM,
                                                   *const ::winapi::CREATESTRUCTW>($l);
                &(*pcs)
            };
            let ret = $self_.on_create(cs);
            if ret {
                return 0 as ::winapi::LRESULT;
            } else {
                return -1 as ::winapi::LRESULT;
            }
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_DESTROY) => (
        if $msg == 0x0002 { // WM_DESTROY
            $self_.on_destroy();
            return 0 as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_SIZE) => (
        if $msg == 0x0005 { // WM_SIZE
            let l = $l as u32;
            let width = (l & 0xFFFF) as isize;
            let height = (l >> 16) as isize;
            $self_.on_size(width, height);
            return 0 as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_SETFOCUS) => (
        if $msg == 0x0007 { // WM_SETFOCUS
            let w = ::windows::window::Window { wnd: $w as ::winapi::HWND };
            $self_.on_focus(w);
            return 0 as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_PAINT) => (
        if $msg == 0x000F { // WM_PAINT
            $self_.on_paint();
            return 0 as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_LBUTTONDOWN) => (
        if $msg == 0x0201 { // WM_LBUTTONDOWN
            let l = $l as u32;
            let x = (l & 0xFFFF) as isize;
            let y = (l >> 16) as isize;
            let flags = $w as u32;
            $self_.on_left_button_down(x, y, flags);
            return 0 as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_LBUTTONUP) => (
        if $msg == 0x0202 { // WM_LBUTTONUP
            let l = $l as u32;
            let x = (l & 0xFFFF) as isize;
            let y = (l >> 16) as isize;
            let flags = $w as u32;
            $self_.on_left_button_up(x, y, flags);
            return 0 as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_KEYDOWN) => (
        if $msg == 0x0100 { // WM_KEYDOWN
            return $self_.on_key_down($w as u8, $l as u32) as ::winapi::::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_KEYUP) => (
        if $msg == 0x0101 { // WM_KEYUP
            return $self_.on_key_up($w as u8, $l as u32) as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, WM_ERASEBKGND) => (
        if $msg == 0x0014 { // WM_ERASEBKGND
            // Returning 1 means that the background no longer needs erasing.
            return $self_.on_erase_background() as ::winapi::LRESULT;
        }
    );
    ($self_:ident, $msg:ident, $w:ident, $l:ident, ANY) => (
        if let Some(result) = $self_.on_message($msg, $w, $l) {
            return result;
        }
    );
);

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

        fn wnd_proc(&self, msg: ::winapi::UINT, w: ::winapi::WPARAM,
                    l: ::winapi::LPARAM) -> ::winapi::LRESULT {
            $(
                wnd_proc_thunk!(self, msg, w, l, $msg);
            )+
            ::windows::def_window_proc(self.wnd().wnd, msg, w, l)
        }
    }

    )
);
