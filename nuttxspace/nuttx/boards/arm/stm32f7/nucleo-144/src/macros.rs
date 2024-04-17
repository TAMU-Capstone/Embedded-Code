
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