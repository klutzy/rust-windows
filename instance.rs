use std::ptr;

use ll::*;

pub struct Instance {
    instance: HINSTANCE
}

impl Instance {
    pub fn main_instance() -> Instance {
        Instance {
            instance: unsafe { GetModuleHandleW(ptr::null()) as HINSTANCE },
        }
    }
}
