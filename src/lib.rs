pub mod include {
    pub mod hardware {
        pub mod regs {
            pub mod proc_pio;
        }
        pub mod clocks;
        pub mod gpio;
        pub mod pio;
        pub mod pio_instructions;
        pub mod timer;
    }
    pub mod pico {
        pub mod stdlib;
    }
    pub mod pio_platform;
    pub mod piolib;
}
#[allow(non_snake_case)]
pub mod PARAM_ASSERTIONS_ENABLED_;
