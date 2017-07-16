#![feature(const_fn, proc_macro)]

extern crate mocktopus_injector;
extern crate mocktopus;

use mocktopus::*;
use mocktopus_injector::*;
use std::fmt::Display;

mod mocking_fns;
mod mocking_methods;

static STATIC_U8: u8 = 1;
static STATIC_BOOL: bool = true;
static STATIC_F32: f32 = 1.5;
static STATIC_CHAR: char = 'a';
static STATIC_STR: &str = "abc";

// generic variants: fn, struct, trait
// self variants: none, val, ref, mut ref
// fns                   ~ 2 generic combinations * 1 self variants = 2  DONE  1*2=02/02
// methods               ~ 4 generic combinations * 4 self variants = 16 TODO  1*1=01/16
// trait methods         ~ 8 generic combinations * 4 self variants = 32 TODO  0*0=00/32
// default trait methods ~ 8 generic combinations * 4 self variants = 32 TODO  0*0=00/32
// directory + mod file      file                     test          = 82           02/82

// naming: function, val_method, ref_method, ref_mut_method, Struct, Trait
// tested fn has <self if method> + 1 + <1 if fn generic> + <1 if trait generic> Display args
// all types must be configured so that final string consists of u8, bool, f32 and char
// fns returns formatted String with all args
// methods return String starting with self field
// &mut self methods double own field before printing
// structs are 1 field named tuples

// each test needs unmocked, Return and Continue versions
// Return mocked fns return same String, but prefixed with "mocked"
// Continue mocked fns increment or negate the args
// generics need test with each generic not leaking and all lifetimes leaked
// this test must be for static and local ref, both Return mocked
// &self and &mut self methods check that self is not modified when mocked
