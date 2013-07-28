pub use std::libc::*;
pub use std::libc::types::os::arch::extra::*;

// small portion from winnt.h
pub type UINT = c_uint;
pub type LONG = c_long;
pub type SHORT = c_short;

pub type PVOID = *c_void;

// 32-bit specific
pub type UINT_PTR = c_uint;
pub type LONG_PTR = c_long;
pub type ULONG_PTR = c_ulong;
