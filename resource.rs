use std::ptr;
use std;

use libc::c_int;
use libc::HANDLE;
use ll::types::UINT;

use wchar::ToCU16Str;
use instance::Instance;

pub trait ToHandle {
    fn to_handle(&self) -> HANDLE;
}

impl<T: ToHandle> ToHandle for Option<T> {
    fn to_handle(&self) -> HANDLE {
        match *self {
            None => ptr::mut_null(),
            Some(ref s) => s.to_handle(),
        }
    }
}

#[allow(non_camel_case_types)]
pub enum ImageType {
    IMAGE_BITMAP = 0,
    IMAGE_ICON = 1,
    IMAGE_CURSOR = 2,
}

pub struct Image {
    pub image: HANDLE,
}

impl Image {
    pub fn load_resource(instance: Instance, id: int, img_type: ImageType, width: int, height: int) -> Option<Image> {
        let img = unsafe {
            super::ll::all::LoadImageW(
                instance.instance, std::mem::transmute(id), img_type as UINT,
                width as c_int, height as c_int, 0x8000
            )
        };

        if img == ptr::mut_null() {
            None
        } else {
            Some(Image { image: img })
        }
    }

    pub fn load_cursor_resource(id: int) -> Option<Image> {
        let null_instance = Instance { instance: ptr::mut_null() };
        Image::load_resource(null_instance, id, IMAGE_CURSOR, 0, 0)
    }
}

impl ToHandle for Image {
    fn to_handle(&self) -> HANDLE {
        self.image
    }
}

pub enum MenuResource {
    MenuName(String),
    MenuId(int),
}

impl MenuResource {
    pub fn with_menu_p<T>(&self, f: |*u16| -> T) -> T {
        match *self {
            MenuName(ref s) => {
                let u = s.as_slice().to_c_u16();
                f(u.as_ptr())
            }
            MenuId(id) => unsafe { f(std::mem::transmute(id)) },
        }
    }

    pub fn null() -> MenuResource {
        MenuId(0)
    }
}
