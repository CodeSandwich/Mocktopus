#![feature(proc_macro)]

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

//#[inject_mocks]
impl Into<u32> for Struct {
    fn into(self) -> u32 {
        match mocktopus::Mockable::call_mock(&<Self as Into<u32>>::into, (self,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        42
    }
}

//#[inject_mocks]
impl ::std::convert::Into<f64> for Struct {
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

struct S3(u32);

#[inject_mocks]
impl<'a> ::std::convert::From<&'a u32> for S3 {
    fn from(f: &'a u32) -> Self {
        let f = match mocktopus::Mockable::call_mock(&<Self as ::std::convert::From<&'a u32>>::from, (f,)) {
            mocktopus::MockResult::Continue((xx,)) => xx,
            mocktopus::MockResult::Return(xx) => return xx,
        };
        S3(*f)
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

    let f = 4;
    assert_eq!(4, S3::from(&f).0);
    <S3 as From<&u32>>::from.mock_raw(|_| MockResult::Return(S3(5)));
    assert_eq!(5, S3::from(&f).0);
}
