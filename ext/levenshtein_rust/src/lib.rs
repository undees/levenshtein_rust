use rb_sys::{
    RB_TYPE,
    RSTRING_LEN,
    RSTRING_PTR,
    VALUE,
    rb_define_module,
    rb_define_module_function,
    rb_eArgError,
    rb_eTypeError,
    rb_enc_get,
    rb_funcall,
    rb_intern,
    rb_raise,
    rb_uint2inum,
    rb_utf8_encoding,
};
use rb_sys::ruby_value_type::RUBY_T_STRING;
use std::os::raw::c_char;

/// Compute the Levenshtein edit distance between two UTF-8 strings.
#[inline]
fn distance(s1: &str, s2: &str) -> usize {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    if s1_len == 0 || s2_len == 0 {
        return s1_len.max(s2_len);
    }

    if s1 == s2 {
        return 0;
    }

    let mut distances: Vec<usize> = (0..=s2_len).collect();
    let mut workspace: Vec<usize> = vec![0; s2_len + 1];

    for (i, c1) in s1.chars().enumerate() {
        workspace[0] = i + 1;

        for (j, c2) in s2.chars().enumerate() {
            let deletion_cost = distances[j + 1] + 1;
            let insertion_cost = workspace[j] + 1;
            let substitution_cost = distances[j] +
                (if c1 == c2 { 0 } else { 1 });

            workspace[j + 1] = deletion_cost
                .min(insertion_cost)
                .min(substitution_cost);
        }

        std::mem::swap(&mut distances, &mut workspace);
    }

    // SAFETY: `distances` is guaranteed to be non-empty because `s2_len > 0`.
    *distances.last().unwrap()
}

unsafe fn is_valid_encoding(s: VALUE) -> bool {
    let method_sym = rb_intern("valid_encoding?\0".as_ptr() as *const c_char);
    rb_funcall(s, method_sym, 0) != 0
}

#[inline(always)]
/// SAFETY: `v` must be a VALUE of Ruby type `RUBY_T_STRING` containing valid UTF-8.
unsafe fn value_to_str(v: VALUE) -> Result<&'static str, ()> {
    let ptr = RSTRING_PTR(v) as *const u8;
    let len = RSTRING_LEN(v) as usize;
    let bytes = std::slice::from_raw_parts(ptr, len);

    if bytes.contains(&b'\0') {
        return Err(());
    }

    std::str::from_utf8(bytes).map_err(|_| ())
}

unsafe fn ensure_valid_utf8<'a>(s: VALUE) -> &'a str {
    if RB_TYPE(s) != RUBY_T_STRING {
        rb_raise(rb_eTypeError, b"Expected a String\0".as_ptr() as *const c_char);
    }

    if rb_enc_get(s) != rb_utf8_encoding() {
        rb_raise(rb_eArgError, b"Expected UTF-8 encoding\0".as_ptr() as *const c_char);
    }

    if !is_valid_encoding(s) {
        rb_raise(rb_eArgError, b"Invalid UTF-8 in byte sequence\0".as_ptr() as *const c_char);
    }

    match value_to_str(s) {
        Ok(s) => s,
        Err(_) => rb_raise(rb_eArgError, b"String contains embedded NUL bytes\0".as_ptr() as *const c_char),
    }
}

#[no_mangle]
pub unsafe extern "C" fn distance_ruby(_self: VALUE, s1: VALUE, s2: VALUE) -> VALUE {
    let s1 = ensure_valid_utf8(s1);
    let s2 = ensure_valid_utf8(s2);

    let result: usize = distance(s1, s2);
    let ruby_result = rb_uint2inum(result as _);

    ruby_result
}

#[no_mangle]
pub extern "C" fn Init_levenshtein_rust() {
    let module = unsafe {
        rb_define_module(b"LevenshteinRust\0".as_ptr() as *const c_char)
    };

    let function = Some(unsafe {
        // SAFETY: Ruby expects a function pointer matching VALUE, VALUE, VALUE -> VALUE,
        // but the rb_define_module_function API requires a cast to fn() -> VALUE.
        // This is a common FFI hack to accommodate Ruby's varargs function pointer type.
        std::mem::transmute::<
            unsafe extern "C" fn(VALUE, VALUE, VALUE) -> VALUE,
            unsafe extern "C" fn() -> VALUE,
        >(distance_ruby)
    });

    unsafe {
        rb_define_module_function(
            module,
            b"distance\0".as_ptr() as *const c_char,
            function,
            2,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::distance;

    #[test]
    fn test_assert_emptiness() {
        assert_eq!(distance("", ""), 0);
        assert_eq!(distance("a", ""), 1);
        assert_eq!(distance("", "a"), 1);
    }

    #[test]
    fn test_assert_edits() {
        assert_eq!(distance("abc", "azc"), 1);
        assert_eq!(distance("abc", "acb"), 2);
        assert_eq!(distance("abc", "ac" ), 1);
        assert_eq!(distance("ac",  "abc"), 1);
    }

    #[test]
    fn test_utf8() {
        assert_eq!(distance("la", "là"), 1);
        assert_eq!(distance("🦀", "🦁"), 1);
    }
}
