macro_rules! printexpr {
    ($e: expr) => {
        println!("{} {:?}", stringify!($e), $e)
    };
}
macro_rules! tryprintexpr {
    ($e: expr) => {
        $e;
        println!("{}", stringify!($e))
    };
}

fn main() {
    println!("{}", piolib_rs::include::hardware::clocks::clock_index::clk_peri as u32);
    printexpr!(piolib_rs::PARAM_ASSERTIONS_ENABLED_::ENABLE_ALL);
    printexpr!(piolib_rs::PARAM_ASSERTIONS_ENABLED_::DISABLE_ALL);
    printexpr!(piolib_rs::PARAM_ASSERTIONS_ENABLED_::GPIO);
    tryprintexpr!(piolib_rs::valid_params_if!(GPIO, true));
    tryprintexpr!(piolib_rs::invalid_params_if!(GPIO, false));
    tryprintexpr!(piolib_rs::valid_params_if!(GPIO, false));
    tryprintexpr!(piolib_rs::invalid_params_if!(GPIO, true));
    println!("done");
}
