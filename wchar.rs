use std::cast;
use std::ptr;
use std::str;
use std::fmt;
use std::vec::Vec;

// Helper struct for *u16 manipulation.
pub struct CU16String {
    priv buf: *u16,
    /// length of buffer, including null
    priv len: uint,
}

impl CU16String {
    /// Create a CU16String from a pointer.
    pub unsafe fn new(buf: *u16) -> CU16String {
        let len = ptr::position(buf, |c| *c == 0) + 1;
        CU16String { buf: buf, len: len }
    }

    /// Converts the CU16String into a `&[u8]` without copying.
    /// NULL is not included.
    ///
    /// # Failure
    ///
    /// Fails if the CU16String is null.
    #[inline]
    pub fn as_u16_vec<'a>(&'a self) -> &'a [u16] {
        if self.buf.is_null() { fail!("CU16String is null!"); }
        unsafe {
            cast::transmute((self.buf, self.len - 1))
        }
    }
}

impl fmt::Show for CU16String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = if self.buf.is_null() {
            ~""
        } else {
            str::from_utf16_lossy(self.as_u16_vec())
        };
        s.fmt(f)
    }
}

/// Parses a C utf-16 "multistring".
/// See `std::c_str::from_c_multistring` for detailed explanation.
pub unsafe fn from_c_u16_multistring(buf: *u16, count: Option<uint>, f: |&[u16]|) -> uint {
    let mut curr_ptr: uint = buf as uint;
    let mut ctr = 0;
    let (limited_count, limit) = match count {
        Some(limit) => (true, limit),
        None => (false, 0)
    };
    while ((limited_count && ctr < limit) || !limited_count)
          && *(curr_ptr as *u16) != 0 as u16 {
        let cstr = CU16String::new(curr_ptr as *u16);
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
        #[allow(deprecated_owned_vector)];
        let mut t = Vec::from_slice(self.to_utf16());
        t.push(0u16);
        t
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
