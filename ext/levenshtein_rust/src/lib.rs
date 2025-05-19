use rb_sys::{
    RB_TYPE,
    VALUE,
    rb_define_module,
    rb_define_module_function,
    rb_eArgError,
    rb_raise,
    rb_string_value_ptr,
    rb_uint2inum,
    ruby_value_type,
};
use std::ffi::CStr;
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

#[inline(always)]
/// SAFETY: `v` must be a VALUE of Ruby type `T_STRING` containing valid UTF-8.
unsafe fn value_to_cstr(v: VALUE) -> Result<&'static str, ()> {
    let ptr = rb_string_value_ptr(&v as *const VALUE as *mut VALUE) as *const c_char;
    CStr::from_ptr(ptr).to_str().map_err(|_| ())
}

#[no_mangle]
pub extern "C" fn distance_ruby(_self: VALUE, s1: VALUE, s2: VALUE) -> VALUE {
    unsafe {
        const T_STRING: ruby_value_type = unsafe { std::mem::transmute(5i32) };

        if RB_TYPE(s1) != T_STRING {
            rb_raise(rb_eArgError, b"Expected first argument to be a String\0".as_ptr() as *const c_char);
        }

        if RB_TYPE(s2) != T_STRING {
            rb_raise(rb_eArgError, b"Expected second argument to be a String\0".as_ptr() as *const c_char);
        }

        let s1 = match value_to_cstr(s1) {
            Ok(s) => s,
            Err(_) => rb_raise(rb_eArgError, b"Invalid UTF-8 in s1\0".as_ptr() as *const c_char),
        };

        let s2 = match value_to_cstr(s2) {
            Ok(s) => s,
            Err(_) => rb_raise(rb_eArgError, b"Invalid UTF-8 in s2\0".as_ptr() as *const c_char),
        };

        let result: usize = distance(s1, s2);
        let ruby_result = rb_uint2inum(result as _);

        ruby_result
    }
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
