macro_rules! print_expr {
    ($e: expr) => {
        println!("{} {:?}", stringify!($e), $e)
    };
}
macro_rules! print_expr_or_panic {
    ($e: expr) => {
        $e;
        println!("{}", stringify!($e))
    };
}

fn main() {
    print_expr!(piolib_rs::include::hardware::clocks::clock_index::clk_peri as u32);
    print_expr!(piolib_rs::include::hardware::gpio::gpio_function::GPIO_FUNC_NULL as u32);
    print_expr!(piolib_rs::include::hardware::pio_instructions::pio_src_dest::pio_exec_mov);
    print_expr!(piolib_rs::include::hardware::pio_instructions::pio_src_dest::pio_x as u32);
    print_expr!(piolib_rs::PARAM_ASSERTIONS_ENABLED_::ENABLE_ALL);
    print_expr!(piolib_rs::PARAM_ASSERTIONS_ENABLED_::DISABLE_ALL);
    print_expr!(piolib_rs::PARAM_ASSERTIONS_ENABLED_::GPIO);
    print_expr_or_panic!(piolib_rs::valid_params_if!(GPIO, true));
    print_expr_or_panic!(piolib_rs::invalid_params_if!(GPIO, false));
    print_expr_or_panic!(piolib_rs::valid_params_if!(GPIO, false));
    print_expr_or_panic!(piolib_rs::invalid_params_if!(GPIO, true));
    println!("done");
}
