#[test]
fn self_method() {
    let s = S(2);
    assert_eq!(200, s.f());
}

#[test]
fn mut_self_method() {
    let s = S(3);
    assert_eq!(300, s.f_mut());
}

#[test]
fn self_ref_method() {
    let s = S(4);
    assert_eq!(400, s.f_ref(&S(400)));
    assert_eq!(4, s.0);
}

#[test]
fn self_mut_ref_method() {
    let mut s = S(5);
    assert_eq!(500, s.f_ref_mut(&mut S(500)));
    assert_eq!(5, s.0);
}

struct S(u32);

use std::mem;

impl <'a> S {
    fn f(self) -> u32 {
        unsafe {
            *(&self as *const Self as *mut Self) = mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized()).s();
        }
        self.0
    }

    fn f_mut(mut self) -> u32 {
        self = self.s();
        self.0
    }

    fn f_ref(&'a self, other: &'a Self) -> u32 {
        unsafe {
            *(&self as *const &Self as *mut &Self) = mem::replace(&mut*(&self as *const &Self as *mut &Self), mem::uninitialized()).t(other);
        }
        self.0
    }

    fn f_ref_mut(&'a mut self, other: &'a mut Self) -> u32 {
        unsafe {
            *(&self as *const &mut Self as *mut &mut Self) = mem::replace(&mut*(&self as *const &mut Self as *mut &mut Self), mem::uninitialized()).u(other);
        }
        self.0
    }

    fn s(self) -> Self {
        S(self.0 * 100)
    }

    fn t(&'a self, other: &'a Self) -> &'a Self {
        other
    }

    fn u(&'a mut self, other: &'a mut Self) -> &'a mut Self {
        other
    }
}
