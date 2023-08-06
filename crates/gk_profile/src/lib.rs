#[macro_export]
macro_rules! function {
    ($($arg: tt)*) => {
        #[cfg(feature = "puffin")]
        puffin::profile_function!($($arg)*);
    };
}

#[macro_export]
macro_rules! scope {
    ($($arg: tt)*) => {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!($($arg)*);
    };
}

#[macro_export]
macro_rules! init {
    () => {
        #[cfg(feature = "puffin")]
        {
            puffin::set_scopes_on(true);
        }
    };
}

#[macro_export]
macro_rules! tick {
    () => {
        #[cfg(feature = "puffin")]
        puffin::GlobalProfiler::lock().new_frame();
    };
}
