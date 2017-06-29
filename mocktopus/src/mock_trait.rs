use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::transmute;

pub trait MockTrait<T, O> {
    fn set_mock<M: Fn<T, Output=MockResult<T, O>> + 'static>(&self, mock: M);
    unsafe fn set_mock_unsafe<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M);
    fn call_mock(&self, input: T) -> MockResult<T, O>;
    unsafe fn get_mock_id(&self) -> TypeId;
}

pub enum MockResult<T, O> {
    Continue(T),
    Return(O),
}

thread_local!{
    static MOCK_STORE: RefCell<HashMap<TypeId, Box<Fn<(), Output=()>>>> = RefCell::new(HashMap::new())
}

impl<T, O, F: FnOnce<T, Output=O>> MockTrait<T, O> for F {
    fn set_mock<M: Fn<T, Output=MockResult<T, O>> + 'static>(&self, mock: M) {
        unsafe {
            self.set_mock_unsafe(mock);
        }
    }

    unsafe fn set_mock_unsafe<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M) {
        let id = self.get_mock_id();
        MOCK_STORE.with(|mock_ref_cell| {
            let fn_box: Box<Fn<T, Output=MockResult<T, O>>> = Box::new(mock);
            let stored: Box<Fn<(), Output=()>> = transmute(fn_box);
            let mock_map = &mut*mock_ref_cell.borrow_mut();
            mock_map.insert(id, stored);
        })
    }

    fn call_mock(&self, input: T) -> MockResult<T, O> {
        unsafe {
            let id = self.get_mock_id();
            MOCK_STORE.with(|mock_ref_cell| {
                let mock_map = &*mock_ref_cell.borrow();
                match mock_map.get(&id) {
                    Some(stored_box) => {
                        let stored = &**stored_box;
                        let mock: &Fn<T, Output=MockResult<T, O>> = transmute(stored);
                        mock.call(input)
                    },
                    None => MockResult::Continue(input),
                }
            })
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).get_type_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mocks_do_not_leak_between_tests {
        use super::*;

        fn mock_injected_function() -> &'static str {
            let () = match mock_injected_function.call_mock(()) {
                MockResult::Continue(input) => input,
                MockResult::Return(result) => return result,
            };
            "not mocked"
        }

        macro_rules! generate_tests {
            ($($fn_name:ident),+) => {
                $(
                    #[test]
                    fn $fn_name() {
                        assert_eq!("not mocked", mock_injected_function(), "function was mocked before mocking");

                        mock_injected_function.set_mock(|| MockResult::Return((stringify!($fn_name))));

                        assert_eq!(stringify!($fn_name), mock_injected_function(), "mocking failed");
                    }
                )+
            }
        }

        generate_tests!(t01, t02, t03, t04, t05, t06, t07, t08, t09, t10, t11, t12, t13, t14, t15, t16);
        generate_tests!(t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32);
    }

    mod mocks_functions {
        use super::*;

        fn mock_injected_function(input: i32) -> &'static str {
            let (input,) = match mock_injected_function.call_mock((input, )) {
                MockResult::Continue(input) => input,
                MockResult::Return(result) => return result,
            };
            if input >= 0 {
                "positive"
            } else {
                "negative"
            }
        }

        #[test]
        fn invokes_mock() {
            mock_injected_function.set_mock(|i|
                if i < -1 {
                    MockResult::Return("mocked negative")
                } else if i > 1 {
                    MockResult::Return("mocked positive")
                } else {
                    MockResult::Continue((i, ))
                });

            assert_eq!("mocked negative", mock_injected_function(-2));
            assert_eq!("negative", mock_injected_function(-1));
            assert_eq!("positive", mock_injected_function(1));
            assert_eq!("mocked positive", mock_injected_function(2));
        }
    }

    mod mocks_generic_functions {
        use super::*;

        fn mock_injected_function<T, U>(_ignored: T, returned: U) -> U {
            let (_ignored, returned) = match mock_injected_function.call_mock((_ignored, returned)) {
                MockResult::Continue(input) => input,
                MockResult::Return(result) => return result,
            };
            returned
        }

        #[test]
        fn invokes_mock_if_generics_match_ignoring_lifetimes() {
            mock_injected_function::<u32, &str>.set_mock(|_, _| MockResult::Return("mocked"));

            assert_eq!("mocked", mock_injected_function(1u32, "not mocked"));
            assert_eq!("mocked", mock_injected_function(1u32, &*"not mocked".to_string()));
            assert_eq!("not mocked", mock_injected_function(1i32, "not mocked"));
            assert_eq!(2, mock_injected_function(1u32, 2));
        }
    }

    mod mocks_generic_function_for_generic_trait_in_generic_struct {
        use super::*;
        use std::fmt::Display;

        trait GT<T: Display, U: Display> {
            fn gf<V: Display>(t: T, u: U, v: V) {
                println!("Default t: {}, u: {}, v: {}", t, u, v);
            }
        }

        struct DS<W: Display> (W);

        impl<T: Display, W: Display> GT<&'a str, U> for DS<W> {}

        struct CS<W: Display> (W);

        impl<'a, W: Display> GT<&'a str> for CS<W> {
            fn gf<U: Display, W>(t: &str, u: U, v: W) {
                println!("Custom t: {}, u: {}, v: {}", t, u, v);
            }
        }
    }
}
