use std::ptr;

// XXX copy of std::os::win32::as_utf16_p
pub fn with_utf16_p<T>(s: &str, f: &fn(*u16) -> T) -> T {
    let mut t = s.to_utf16();
    // Null terminate before passing on.
    t.push(0u16);
    t.as_imm_buf(|buf, _len| f(buf))
}

pub fn with_utf16_p_or_null<T>(s: &Option<~str>, f: &fn(*u16) -> T) -> T {
    match s {
        &None => f(ptr::null()),
        &Some(ref s) => with_utf16_p(*s, f),
    }
}
