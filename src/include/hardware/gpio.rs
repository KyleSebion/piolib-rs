pub const PARAM_ASSERTIONS_ENABLED_GPIO: bool = cfg!(feature = "PARAM_ASSERTIONS_ENABLED_GPIO");

pub const NUM_BANK0_GPIOS: u32 = 32;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum gpio_function {
    GPIO_FUNC_XIP = 0,
    GPIO_FUNC_SPI = 1,
    GPIO_FUNC_UART = 2,
    GPIO_FUNC_I2C = 3,
    GPIO_FUNC_PWM = 4,
    GPIO_FUNC_SIO = 5,
    GPIO_FUNC_PIO0 = 6,
    GPIO_FUNC_PIO1 = 7,
    GPIO_FUNC_GPCK = 8,
    GPIO_FUNC_USB = 9,
    GPIO_FUNC_NULL = 0x1f,
}

pub const GPIO_OUT: u32 = 1;
pub const GPIO_IN: u32 = 0;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum gpio_irq_level {
    GPIO_IRQ_LEVEL_LOW = 0x1,
    GPIO_IRQ_LEVEL_HIGH = 0x2,
    GPIO_IRQ_EDGE_FALL = 0x4,
    GPIO_IRQ_EDGE_RISE = 0x8,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum gpio_override {
    GPIO_OVERRIDE_NORMAL = 0,      //< peripheral signal selected via \ref gpio_set_function
    GPIO_OVERRIDE_INVERT = 1,      //< invert peripheral signal selected via \ref gpio_set_function
    GPIO_OVERRIDE_LOW = 2,         //< drive low/disable output
    GPIO_OVERRIDE_HIGH = 3,        //< drive high/enable output
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum gpio_slew_rate {
    GPIO_SLEW_RATE_SLOW = 0,  //< Slew rate limiting enabled
    GPIO_SLEW_RATE_FAST = 1   //< Slew rate limiting disabled
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum gpio_drive_strength {
    GPIO_DRIVE_STRENGTH_2MA = 0, //< 2 mA nominal drive strength
    GPIO_DRIVE_STRENGTH_4MA = 1, //< 4 mA nominal drive strength
    GPIO_DRIVE_STRENGTH_8MA = 2, //< 8 mA nominal drive strength
    GPIO_DRIVE_STRENGTH_12MA = 3 //< 12 mA nominal drive strength
}

pub fn check_gpio_param(gpio: u32) {
    crate::invalid_params_if!(GPIO, gpio >= NUM_BANK0_GPIOS);
}
