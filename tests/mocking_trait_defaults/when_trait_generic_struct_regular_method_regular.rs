use super::*;

#[mockable]
trait Trait<V: Display>: Sized {
    fn static_method(arg: bool, trait_generic: V) -> String {
        format!("{} {}", arg, trait_generic)
    }

    fn ref_method(&self, arg: bool, trait_generic: V) -> String {
        format!("{} {} {}", self.to_string(), arg, trait_generic)
    }

    fn ref_mut_method(&mut self, arg: bool, trait_generic: V) -> String {
        self.modify();
        format!("{} {} {}", self.to_string(), arg, trait_generic)
    }

    fn val_method(self, arg: bool, trait_generic: V) -> String {
        format!("{} {} {}", self.to_string(), arg, trait_generic)
    }

    fn to_string(&self) -> String;
    fn modify(&mut self);
}

struct Struct(u8);

impl<V: Display> Trait<V> for Struct {
    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn modify(&mut self) {
        self.0 *= 2;
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true a", Struct::static_method(true, 'a'));
        assert_eq!("true abc", Struct::static_method(true, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            <Struct as Trait<char>>::static_method
                .mock_raw(|a, b| MockResult::Continue((!a, b.to_ascii_uppercase())));
        }

        assert_eq!("false A", Struct::static_method(true, 'a'));
        assert_eq!("true abc", Struct::static_method(true, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            <Struct as Trait<char>>::static_method
                .mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a, b)));
        }

        assert_eq!("mocked true a", Struct::static_method(true, 'a'));
        assert_eq!("true abc", Struct::static_method(true, "abc"));
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true a", Struct(2).ref_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2).ref_method(true, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2);
        let struct_3 = Struct(3);
        unsafe {
            <Struct as Trait<char>>::ref_method
                .mock_raw(|_, b, c| MockResult::Continue((&struct_3, !b, c.to_ascii_uppercase())));
        }

        assert_eq!("3 false A", struct_2.ref_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
        assert_eq!("2 true abc", Struct(2).ref_method(true, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2);
        unsafe {
            <Struct as Trait<char>>::ref_method
                .mock_raw(|a, b, c| MockResult::Return(format!("mocked {} {} {}", a.0, b, c)));
        }

        assert_eq!("mocked 2 true a", struct_2.ref_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!("2 true abc", Struct(2).ref_method(true, "abc"));
    }
}

mod and_method_is_ref_mut_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        let mut struct_2 = Struct(2);
        let mut struct_4 = Struct(4);

        assert_eq!("4 true a", struct_2.ref_mut_method(true, 'a'));
        assert_eq!(4, struct_2.0);
        assert_eq!("8 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(8, struct_4.0);
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let mut struct_2 = Struct(2);
        let struct_3 = Struct(3);
        let mut struct_4 = Struct(4);
        unsafe {
            <Struct as Trait<char>>::ref_mut_method.mock_raw(|_, b, c| {
                MockResult::Continue((as_mut(&struct_3), !b, c.to_ascii_uppercase()))
            });
        }

        assert_eq!("6 false A", struct_2.ref_mut_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(6, struct_3.0);
        assert_eq!("8 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(8, struct_4.0);
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let mut struct_2 = Struct(2);
        let mut struct_4 = Struct(4);
        unsafe {
            <Struct as Trait<char>>::ref_mut_method
                .mock_raw(|a, b, c| MockResult::Return(format!("mocked {} {} {}", a.0, b, c)));
        }

        assert_eq!("mocked 2 true a", struct_2.ref_mut_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!("8 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(8, struct_4.0);
    }
}

mod and_method_is_val_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true a", Struct(2).val_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2).val_method(true, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            <Struct as Trait<char>>::val_method.mock_raw(move |_, b, c| {
                MockResult::Continue((Struct(3), !b, c.to_ascii_uppercase()))
            });
        }

        assert_eq!("3 false A", Struct(2).val_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2).val_method(true, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            <Struct as Trait<char>>::val_method
                .mock_raw(|a, b, c| MockResult::Return(format!("mocked {} {} {}", a.0, b, c)));
        }

        assert_eq!("mocked 2 true a", Struct(2).val_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2).val_method(true, "abc"));
    }
}
