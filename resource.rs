use std::ptr;
use std;

use ll::*;
use wchar;
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

pub enum ImageType {
    IMAGE_BITMAP = 0,
    IMAGE_CURSOR = 1,
    IMAGE_ICON = 2,
}

pub struct Image {
    image: HANDLE,
}

impl Image {
    #[fixed_stack_segment]
    pub fn load_resource(instance: Instance, id: int, img_type: ImageType, width: int, height: int) -> Option<Image> {
        let img = unsafe {
            LoadImageW(
                instance.instance, std::cast::transmute(id), img_type as UINT,
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
    MenuName(~str),
    MenuId(int),
}

impl MenuResource {
    pub fn with_menu_p<T>(&self, f: &fn(*u16) -> T) -> T {
        match *self {
            MenuName(ref s) => wchar::with_utf16_p(*s, f),
            MenuId(id) => unsafe { f(std::cast::transmute(id)) },
        }
    }

    pub fn null() -> MenuResource {
        MenuId(0)
    }
}
