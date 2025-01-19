pub const PICO_DEFAULT_LED_PIN: u32 = 4;
pub const PARAM_ASSERTIONS_ENABLE_ALL: bool = cfg!(feature = "PARAM_ASSERTIONS_ENABLE_ALL");
pub const PARAM_ASSERTIONS_DISABLE_ALL: bool = cfg!(feature = "PARAM_ASSERTIONS_DISABLE_ALL");
#[macro_export]
macro_rules! PARAM_ASSERTIONS_ENABLED {
    ($x:ident) => {{
        ($crate::PARAM_ASSERTIONS_ENABLED_::$x
            || $crate::PARAM_ASSERTIONS_ENABLED_::ENABLE_ALL)
            && !$crate::PARAM_ASSERTIONS_ENABLED_::DISABLE_ALL
    }};
}
#[macro_export]
macro_rules! invalid_params_if {
    ($x:ident, $($test:tt)*) => {{
        if ($crate::PARAM_ASSERTIONS_ENABLED!($x)) {
            assert!(!($($test)*));
        }
    }};
}
#[macro_export]
macro_rules! valid_params_if {
    ($x:ident, $($test:tt)*) => {{
        if ($crate::PARAM_ASSERTIONS_ENABLED!($x)) {
            assert!($($test)*);
        }
    }};
}
#[macro_export]
macro_rules! bool_to_bit {
    ($x: expr) => {{
        ($x) as u32
    }};
}
#[macro_export]
macro_rules! count_of {
    ($a: expr) => {{
        ($a).len()
    }};
}
