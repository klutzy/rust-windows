use std::ptr;

/// A generic trait for converting a value to a `CU16String`, like `ToCStr`.
pub trait ToCU16Str {
    fn with_c_u16_str<T>(&self, f: &fn(*u16) -> T) -> T;
}

impl<'self> ToCU16Str for &'self str {
    fn with_c_u16_str<T>(&self, f: &fn(*u16) -> T) -> T {
        let mut t = self.to_utf16();
        // Null terminate before passing on.
        t.push(0u16);
        t.as_imm_buf(|buf, _len| f(buf))
    }
}

impl<S: Str> ToCU16Str for Option<S> {
    fn with_c_u16_str<T>(&self, f: &fn(*u16) -> T) -> T {
        match self {
            &None => f(ptr::null()),
            &Some(ref s) => {
                s.as_slice().with_c_u16_str(f)
            },
        }
    }
}
