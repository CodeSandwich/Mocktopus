pub fn no_args_no_ret_panics() {
    panic!("no_args_no_ret_panics was called");
}

pub fn one_arg_multiplies_by_2(x: u32) -> u32 {
    x * 2
}
