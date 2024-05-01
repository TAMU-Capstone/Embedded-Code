/*
These can be used in place of calls like `uinfo`, `ainfo`, `uerr`, `aerr`, `spiinfo`,  etc.
they all compile down to a `syslog` call.

TODO: Implement a Global Allocator for Rust, which would allow use of `format!`
allowing for Rust style format strings, rather than sticking with C
*/


#[macro_export]
macro_rules! err {
    () => {};
    ($fmt:expr) => {{
        #[cfg(all(CONFIG_DEBUG_ERROR, CONFIG_CPP_HAVE_VARARGS))]
        unsafe {
            crate::bindings::syslog(crate::bindings::LOG_ERR, $fmt.as_ptr());
        }
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        #[cfg(all(CONFIG_DEBUG_ERROR, CONFIG_CPP_HAVE_VARARGS))]
        unsafe {
            crate::bindings::syslog(crate::bindings::LOG_ERR, $fmt.as_ptr(), $($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! info {
    () => {};
    ($fmt:expr) => {{
        #[cfg(all(CONFIG_DEBUG_INFO, CONFIG_CPP_HAVE_VARARGS))]
        unsafe {
            crate::bindings::syslog(crate::bindings::LOG_INFO, $fmt.as_ptr());
        }
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        #[cfg(all(CONFIG_DEBUG_INFO, CONFIG_CPP_HAVE_VARARGS))]
        unsafe {
            crate::bindings::syslog(crate::bindings::LOG_INFO, $fmt.as_ptr(), $($arg)*);
        }
    }};
}