use std::cast;
use std::libc;
use std::ptr;
use std::str;

// CU16String is a cursed version of `std::c_str::CString` for
// null-terminated utf-16 strings.

/// The representation of a C UTF-16 string.
pub struct CU16String {
    priv buf: *u16,
    priv owns_buffer_: bool,
}

// TODO: iter
impl CU16String {
    /// Create a CU16String from a pointer.
    pub unsafe fn new(buf: *u16, owns_buffer: bool) -> CU16String {
        CU16String { buf: buf, owns_buffer_: owns_buffer }
    }

    /// Unwraps the wrapped `*u16` from the `CU16String` wrapper.
    /// Any ownership of the buffer by the `CU16String` wrapper is forgotten.
    pub unsafe fn unwrap(self) -> *u16 {
        let mut c_u16_str = self;
        c_u16_str.owns_buffer_ = false;
        c_u16_str.buf
    }

    /// Calls a closure with a reference to the underlying `*u16`.
    ///
    /// # Failure
    ///
    /// Fails if the CU16String is null.
    pub fn with_ref<T>(&self, f: |*u16| -> T) -> T {
        if self.buf.is_null() { fail!("CU16String is null!"); }
        f(self.buf)
    }

    /// Calls a closure with a mutable reference to the underlying `*u16`.
    ///
    /// # Failure
    ///
    /// Fails if the CU16String is null.
    pub fn with_mut_ref<T>(&mut self, f: |*mut u16| -> T) -> T {
        if self.buf.is_null() { fail!("CU16String is null!"); }
        f(unsafe { cast::transmute_mut_unsafe(self.buf) })
    }

    /// Returns true if the CU16String is a null.
    pub fn is_null(&self) -> bool {
        self.buf.is_null()
    }

    /// Returns true if the CU16String is not null.
    pub fn is_not_null(&self) -> bool {
        self.buf.is_not_null()
    }

    /// Returns whether or not the `CU16String` owns the buffer.
    pub fn owns_buffer(&self) -> bool {
        self.owns_buffer_
    }
}

impl CU16String {
    /// Converts the CU16String into a `&[u8]` without copying.
    ///
    /// # Failure
    ///
    /// Fails if the CU16String is null.
    #[inline]
    pub fn as_u16_vec<'a>(&'a self) -> &'a [u16] {
        if self.buf.is_null() { fail!("CU16String is null!"); }
        unsafe {
            cast::transmute((self.buf, self.len() + 1))
        }
    }
}

impl ToStr for CU16String {
    #[inline]
    fn to_str(&self) -> ~str {
        if self.buf.is_null() {
        }
        str::from_utf16(self.as_u16_vec())
    }
}

impl Drop for CU16String {
    fn drop(&mut self) {
        if self.owns_buffer_ {
            unsafe {
                libc::free(self.buf as *libc::c_void)
            }
        }
    }
}

impl Container for CU16String {
    #[inline]
    fn len(&self) -> uint {
        unsafe {
            ptr::position(self.buf, |c| *c == 0)
        }
    }
}

/// Parses a C utf-16 "multistring".
/// See `std::c_str::from_c_multistring` for detailed explanation.
pub unsafe fn from_c_u16_multistring(buf: *u16, count: Option<uint>, f: |&CU16String|) -> uint {

    let mut curr_ptr: uint = buf as uint;
    let mut ctr = 0;
    let (limited_count, limit) = match count {
        Some(limit) => (true, limit),
        None => (false, 0)
    };
    while ((limited_count && ctr < limit) || !limited_count)
          && *(curr_ptr as *u16) != 0 as u16 {
        let cstr = CU16String::new(curr_ptr as *u16, false);
        f(&cstr);
        curr_ptr += (cstr.len() * 1 + 1) * 2;
        ctr += 1;
    }
    return ctr;
}

/// A generic trait for converting a value to a `CU16String`, like `ToCStr`.
pub trait ToCU16Str {
    fn to_c_u16(&self) -> ~[u16];

    fn with_c_u16_str<T>(&self, f: |*u16| -> T) -> T {
        let t = self.to_c_u16();
        f(t.as_ptr())
    }

    fn with_c_u16_str_mut<T>(&mut self, f: |*mut u16| -> T) -> T {
        let mut t = self.to_c_u16();
        f(t.as_mut_ptr())
    }
}

impl<'a> ToCU16Str for &'a str {
    fn to_c_u16(&self) -> ~[u16] {
        let mut t = self.to_utf16();
        t.push(0u16);
        t
    }
}

impl<S: Str> ToCU16Str for Option<S> {
    fn to_c_u16(&self) -> ~[u16] {
        match self {
            &None => ~[],
            &Some(ref s) => s.as_slice().to_c_u16(),
        }
    }

    fn with_c_u16_str<T>(&self, f: |*u16| -> T) -> T {
        match self {
            &None => f(ptr::null()),
            &Some(ref s) => {
                s.as_slice().with_c_u16_str(f)
            },
        }
    }

    fn with_c_u16_str_mut<T>(&mut self, f: |*mut u16| -> T) -> T {
        match self {
            &None => f(ptr::mut_null()),
            &Some(ref s) => {
                s.as_slice().with_c_u16_str_mut(f)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::CU16String;
    use super::from_c_u16_multistring;

    #[test]
    fn test_as_u16_vec() {
        let u16s: &[u16] = [
            0xac00, 0x20, 0xac00, 0x00,
        ];

        let cu = unsafe { CU16String::new(u16s.as_ptr(), false) };
        let v = cu.as_u16_vec();
        assert_eq!(v, u16s);
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
            from_c_u16_multistring(buf, None, |cu| {
                let b = cu.to_str();
                assert_eq!(b.char_len(), i + 1);
                assert_eq!(b, compare[i].to_owned());
                i += 1;
            });
        }
        assert_eq!(i, 4);
    }
}
