extern crate mocktopus;

use mocktopus::*;
use std::fmt::Debug;

trait Trait {
    fn f(x: u32) -> u32;
    fn g(x: u32) -> u32 {
        let x = match mocktopus::Mockable::call_mock(&<Self as Trait>::g, (x,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        x * 2
    }
}

//TODO TEST TRAIT DEFAULT LEAKAGE
//TODO TEST REGULAR AND TRAIT METHODS LEAKAGE


struct StructG<T>(T);

struct StructG2<T>(T);

impl <T> Into<StructG2<T>> for StructG<T> {
    fn into(mut self) -> StructG2<T> {
        self = match mocktopus::Mockable::call_mock(&Self::into, (self,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        StructG2(self.0)
    }
}

struct Struct();

impl Struct {
    fn into(self) -> u32 {
        match mocktopus::Mockable::call_mock(&Self::into, (self,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        42
    }
}

impl Into<u32> for Struct {
    fn into(self) -> u32 {
        match mocktopus::Mockable::call_mock(&<Self as Into<u32>>::into, (self,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        42
    }
}

impl Into<f64> for Struct {
    fn into(self) -> f64 {
        match mocktopus::Mockable::call_mock(&<Self as Into<f64>>::into, (self,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        42.
    }
}

impl Trait for Struct {
    fn f(x: u32) -> u32 {
        let x = match mocktopus::Mockable::call_mock(&Self::f, (x,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        x * 2
    }
}
struct Struct2();

impl Trait for Struct2 {
    fn f(x: u32) -> u32 {
        let x = match mocktopus::Mockable::call_mock(&<Self as Trait>::f, (x,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        x * 2
    }
}

#[test]
fn trait_experiments() {
    <Struct as Trait>::f.mock_raw(|_| MockResult::Return(123));
    assert_eq!(123, Struct::f(3));
    <Struct as Trait>::f.mock_raw(|x| MockResult::Continue((x+1,)));
    assert_eq!(8, Struct::f(3));


    <Struct as Trait>::g.mock_raw(|_| MockResult::Return(123));
    assert_eq!(123, Struct::g(3));
    <Struct as Trait>::g.mock_raw(|x| MockResult::Continue((x+1,)));
    assert_eq!(8, Struct::g(3));
    assert_eq!(6, Struct2::g(3));
}
