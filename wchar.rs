// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::fmt;
use std::vec::Vec;

// Helper struct for *u16 manipulation.
#[allow(missing_copy_implementations)]
pub struct CU16String {
    buf: *const u16,
    /// length of buffer, including null
    len: usize,
}

impl CU16String {
    /// Create a CU16String from a pointer.
    pub unsafe fn new(buf: *const u16) -> CU16String {
        CU16String { 
            buf: buf, 
            len: {
                let mut length_counter = 0;
                loop {
                    if *buf.offset(length_counter)==0 {
                        break;
                    }
                    length_counter += 1;
                }
                length_counter as usize
            }
        }
    }

    /// Converts the CU16String into a `&[u16]` without copying.
    /// NULL is not included.
    ///
    /// # Failure
    ///
    /// Fails if the CU16String is null.
    #[inline]
    pub fn as_u16_vec<'a>(&'a self) -> &'a [u16] {
        if self.buf.is_null() { panic!("CU16String is null!"); }
        unsafe {
            mem::transmute((self.buf, self.len - 1))
        }
    }
}

impl fmt::Show for CU16String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = if self.buf.is_null() {
            "".to_string()
        } else {
            String::from_utf16_lossy(self.as_u16_vec())
        };
        s.fmt(f)
    }
}

/// Parses a C utf-16 "multistring".
/// See `std::c_str::from_c_multistring` for detailed explanation.
pub unsafe fn from_c_u16_multistring<F>(buf: *const u16, count: Option<usize>, f: F) -> usize 
    where F: Fn(&[u16]) {
    let mut curr_ptr: usize = buf as usize;
    let mut ctr = 0;
    let (limited_count, limit) = match count {
        Some(limit) => (true, limit),
        None => (false, 0)
    };
    while ((limited_count && ctr < limit) || !limited_count)
          && *(curr_ptr as *const u16) != 0 as u16 {
        let cstr = CU16String::new(curr_ptr as *const u16);
        f(cstr.as_u16_vec());
        curr_ptr += cstr.len * 2;
        ctr += 1;
    }
    return ctr;
}

/// A generic trait for converting a value to a `CU16String`, like `ToCStr`.
pub trait ToCU16Str {
    fn to_c_u16(&self) -> Vec<u16>;
}

impl<'a> ToCU16Str for &'a str {
    fn to_c_u16(&self) -> Vec<u16> {
        let mut t : Vec<u16> = self.utf16_units().collect();
        t.push(0u16);
        t
    }
}

impl ToCU16Str for String {
    fn to_c_u16(&self) -> Vec<u16> {
        self.as_slice().to_c_u16()
    }
}


impl<S: Str> ToCU16Str for Option<S> {
    fn to_c_u16(&self) -> Vec<u16> {
        match self {
            &None => Vec::new(),
            &Some(ref s) => s.as_slice().to_c_u16(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CU16String;
    use super::from_c_u16_multistring;
    use std::str::from_utf16;

    #[test]
    fn test_as_u16_vec() {
        let u16s: &[u16] = [
            0xac00, 0x20, 0xac00, 0x00,
        ];

        let cu = unsafe { CU16String::new(u16s.as_ptr()) };
        let v = cu.as_u16_vec();
        assert_eq!(v, u16s.slice_to(u16s.len() - 1));
    }

    #[test]
    fn test_from_c_u16_multistring() {
        let test: &[u16] = [
            0xac00, 0x00,
            0xac00, 0xac00, 0x00,
            0xac00, 0xac00, 0xac00, 0x00,
            0xac00, 0xac00, 0xac00, 0xac02, 0x00,
            0x00,
        ];
        let compare = [
            "가",
            "가가",
            "가가가",
            "가가가갂",
        ];
        let mut i = 0;
        let buf = test.as_ptr();
        unsafe {
            from_c_u16_multistring(buf, None, |p| {
                let b = from_utf16(p).expect("invalid utf-16 sequence");
                assert_eq!(b.char_len(), i + 1);
                assert_eq!(b, compare[i].to_owned());
                i += 1;
            });
        }
        assert_eq!(i, 4);
    }
}
