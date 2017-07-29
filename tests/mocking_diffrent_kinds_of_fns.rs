#![feature(const_fn, proc_macro)]

extern crate mocktopus;

use mocktopus::*;
use std::fmt::Display;

mod mocking_fns;
mod mocking_methods;

// generic variants: fn, struct, trait
// self variants: none, val, ref, mut ref
// fns                   ~ 2 generic combinations * 1 self variants = 2  DONE        2=02/02
// methods               ~ 4 generic combinations * 4 self variants = 16 TODO  4+4+1+1=10/16
// trait methods         ~ 8 generic combinations * 4 self variants = 32 TODO        0=00/32
// default trait methods ~ 8 generic combinations * 4 self variants = 32 TODO        0=00/32
// directory + mod file      file                     test          = 82               12/82

// naming: function, val_method, ref_method, ref_mut_method, Struct, Trait
// tested fn has <self if method> + bool + <1 if fn generic> + <1 if trait generic> Display args
// fns return formatted String with all args
// methods return String starting with self field
// &mut self methods double own field before printing
// structs are 1 field named tuples u8 or generic mocked for u8
// fns are generic over 1 type, mocked for f32
// traits are generic over 1 type, mocked for char

// each test needs unmocked, Return and Continue versions
// Return mocked fns return same String, but prefixed with "mocked"
// Continue mocked fns increment or negate the args
// generics need test with each generic not leaking
// &self and &mut self methods check that self is not modified when mocked
