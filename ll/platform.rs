pub use libc::*;
pub use libc::types::os::arch::extra::*;

// small portion from winnt.h
pub type LONG = c_long;
pub type ULONG = c_ulong;
pub type SHORT = c_short;

pub type PVOID = *c_void;

#[cfg(target_arch = "x86")] pub type INT_PTR = c_int;
#[cfg(target_arch = "x86")] pub type UINT_PTR = c_uint;
#[cfg(target_arch = "x86")] pub type LONG_PTR = c_long;
#[cfg(target_arch = "x86")] pub type ULONG_PTR = c_ulong;

#[cfg(target_arch = "x86_64")] pub type INT_PTR = i64;
#[cfg(target_arch = "x86_64")] pub type UINT_PTR = u64;
#[cfg(target_arch = "x86_64")] pub type LONG_PTR = i64;
#[cfg(target_arch = "x86_64")] pub type ULONG_PTR = u64;
