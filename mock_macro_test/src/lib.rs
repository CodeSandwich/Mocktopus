#![feature(proc_macro)]
extern crate mocktopus_macro;
extern crate mocktopus;

use mocktopus_macro::mock_it;
use mocktopus::{MockResult, MockTrait};

//mod reflection;

//#[mock_it]
pub fn a() {
    println!("a")
}

pub fn main() {

}

#[mock_it]
mod mod_a;
pub use mod_a::mod_b::mod_a_mod_b_fn;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        mod_a::mod_b::mod_a_mod_b_fn();
    }
}
//}
