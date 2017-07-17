#[test]
fn methods() {
    assert_eq!(200, S(2).f());
    assert_eq!(300, S(3).f_mut());
    assert_eq!(400, S(4).f_ref(&S(400)));
    assert_eq!(500, S(5).f_ref_mut(&mut S(500)));
}

struct S(u32);

use std::mem;

impl <'a> S {
    fn f(self) -> u32 {
        unsafe {
            *(&self as *const Self as *mut Self) =
                mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized())
                    .s();
        }
        self.0
    }

    fn f_mut(mut self) -> u32 {
//        self = self.s();
        unsafe {
            *(&self as *const Self as *mut Self) =
                mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized())
                    .s();
        }
        self.0
    }

    fn f_ref(&'a self, other: &'a Self) -> u32 {
        unsafe {
            *(&self as *const &Self as *mut &Self) =
                mem::replace(&mut*(&self as *const &Self as *mut &Self), mem::uninitialized())
                    .t(other);
        }
        self.0
    }

//    let header_str = format!(
//        r#"{{
//            let ({}) = {{
//                use mocktopus::*;
//                match Mockable::call_mock(&{}, (({}))) {{
//                    MockResult::Continue(input) => input,
//                    MockResult::Return(result) => return result,
//                }}
//            }};
//        }}"#, original_arg_names, full_fn_name, unignored_arg_names);

//    let header_str = format!(
//        r#"{{
//            #[allow(unused_unsafe)]
//            let ({}) = unsafe {{  <= shadow list must be trimmed of self
//                use mocktopus::*;
//                match Mockable::call_mock(&{}, (({}))) {{ <= unignored_arg_names have self special treated
//                    MockResult::Continue(input) => { <= generated from arg list
//                        restore self
//                        (input 1..), <= trimmed of self
//                      }
//                    MockResult::Return(result) => return result,
//                }}
//            }};
//        }}"#, original_arg_names, full_fn_name, unignored_arg_names);

    fn f_ref_mut(&'a mut self, other: &'a mut Self) -> u32 {
        unsafe {
//            let self_mut_ref = &self as *const &mut Self as *mut &mut Self;
//            let self_val = mem::replace(&mut*(&self as *const &mut Self as *mut &mut Self), mem::uninitialized());
            let results = Self::u(mem::replace(&mut*(&self as *const &mut Self as *mut &mut Self), mem::uninitialized()), other);
            *(&self as *const &mut Self as *mut &mut Self) = results;
            // reteurn the rest of result

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
