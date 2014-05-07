use std::ptr;

use ll::types::HINSTANCE;

pub struct Instance {
    pub instance: HINSTANCE
}

impl Instance {
    pub fn main_instance() -> Instance {
        Instance {
            instance: unsafe { super::ll::all::GetModuleHandleW(ptr::null()) as HINSTANCE },
        }
    }
}
