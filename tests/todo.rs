#[test]
fn method_experiments() {
    let s4 = S(4);
    let mut s5 = S(5);
    assert_eq!(200, S(2).f());
    assert_eq!(300, S(3).f_mut());
    assert_eq!(400, s4.f_ref(&S(400)));
    assert_eq!(4,   s4.0);
    assert_eq!(500, s5.f_ref_mut(&mut S(500)));
    assert_eq!(5,   s5.0);
}

struct S(u32);

use std::mem;

impl <'a> S {
    fn f(self) -> u32 {
        unsafe {
            let moved_self = mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized());
            let new_self = moved_self.s();
            *(&self as *const Self as *mut Self) = new_self;
//            *(&self as *const Self as *mut Self) =
//                mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized())
//                    .s();
        }
        self.0
    }

    fn f_mut(mut self) -> u32 {
        unsafe {
            let moved_self = mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized());
            let new_self = moved_self.s();
            *(&self as *const Self as *mut Self) = new_self;
//            *(&self as *const Self as *mut Self) =
//                mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized())
//                    .s();
        }
        self.0
    }

    fn f_ref(&'a self, other: &'a Self) -> u32 {
        unsafe {
            let moved_self = mem::replace(&mut*(&self as *const &Self as *mut &Self), mem::uninitialized());
            let new_self = moved_self.t(other);
            *(&self as *const &Self as *mut &Self) = new_self;
//            *(&self as *const &Self as *mut &Self) =
//                mem::replace(&mut*(&self as *const &Self as *mut &Self), mem::uninitialized())
//                    .t(other);
        }
        self.0
    }

    fn f_ref_mut(&'a mut self, other: &'a mut Self) -> u32 {
        unsafe { // 'unsafe' IF SELF
            //A
            let new_self = Self::u(mem::replace(&mut*(&self as *const &mut Self as *mut &mut Self), mem::uninitialized()), other);
            *(&self as *const &mut Self as *mut &mut Self) = new_self; // IF SELF
            //B
//            let moved_self = mem::replace(&mut*(&self as *const &mut Self as *mut &mut Self), mem::uninitialized());
//            let new_self = moved_self.u(other);
//            *(&self as *const &mut Self as *mut &mut Self) = new_self;
            //C
//            *(&self as *const &mut Self as *mut &mut Self) =
//                mem::replace(&mut*(&self as *const &mut Self as *mut &mut Self), mem::uninitialized())
//                    .u(other);
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
