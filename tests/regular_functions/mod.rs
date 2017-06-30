pub fn no_args_no_ret_panics() {
    panic!("no_args_no_ret_panics was called");
}

pub fn one_arg_multiplies_by_2(x: u32) -> u32 {
    x * 2
}

pub fn no_args_returns_str() -> &'static str {
    "not mocked"
}

pub const fn const_fn() -> u32 {
    1
}

pub fn two_args_returns_first_ignores_second(x: u32, _: u32) -> u32 {
    x
}
