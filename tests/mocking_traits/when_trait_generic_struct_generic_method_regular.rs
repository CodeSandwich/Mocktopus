use super::*;

trait Trait<V: Display> {
    fn static_method(arg: bool, trait_generic: V) -> String;
    fn ref_method(&self, arg: bool, trait_generic: V) -> String;
    fn ref_mut_method(&mut self, arg: bool, trait_generic: V) -> String;
    fn val_method(self, arg: bool, trait_generic: V) -> String;
}

struct Struct<T>(T);

#[mockable]
impl<T: Display + Default, V: Display> Trait<V> for Struct<T> {
    fn static_method(arg: bool, trait_generic: V) -> String {
        format!("{} {}", arg, trait_generic)
    }

    fn ref_method(&self, arg: bool, trait_generic: V) -> String {
        format!("{} {} {}", self.0, arg, trait_generic)
    }

    fn ref_mut_method(&mut self, arg: bool, trait_generic: V) -> String {
        self.0 = T::default();
        format!("{} {} {}", self.0, arg, trait_generic)
    }

    fn val_method(self, arg: bool, trait_generic: V) -> String {
        format!("{} {} {}", self.0, arg, trait_generic)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true a", Struct::<u8>::static_method(true, 'a'));
        assert_eq!("true a", Struct::<&str>::static_method(true, 'a'));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        unsafe {
            <Struct<u8> as Trait<char>>::static_method.mock_raw(|a, b|
                MockResult::Continue((!a, b.to_ascii_uppercase())));
        }

        assert_eq!("false A", Struct::<u8>::static_method(true, 'a'));
        assert_eq!("true a", Struct::<&str>::static_method(true, 'a'));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        unsafe {
            <Struct<u8> as Trait<char>>::static_method.mock_raw(|a, b|
                MockResult::Return(format!("mocked {} {}", a, b), ));
        }

        assert_eq!("mocked true a", Struct::<u8>::static_method(true, 'a'));
        assert_eq!("true a", Struct::<&str>::static_method(true, 'a'));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true a", Struct(2u8).ref_method(true, 'a'));
        assert_eq!("abc true a", Struct("abc").ref_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2u8).ref_method(true, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_method.mock_raw(|_, b, c|
                MockResult::Continue((&struct_3, !b, c.to_ascii_uppercase())));
        }

        assert_eq!("3 false A", struct_2.ref_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
        assert_eq!("abc true a", Struct("abc").ref_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2u8).ref_method(true, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_method.mock_raw(|a, b, c|
                MockResult::Return(format!("mocked {} {} {}", a.0, b, c), ));
        }

        assert_eq!("mocked 2 true a", struct_2.ref_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!("abc true a", Struct("abc").ref_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2u8).ref_method(true, "abc"));
    }
}

mod and_method_is_ref_mut_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        let mut struct_2 = Struct(2u8);
        let mut struct_str = Struct("str");
        let mut struct_4 = Struct(4u8);

        assert_eq!("0 true a", struct_2.ref_mut_method(true, 'a'));
        assert_eq!(0, struct_2.0);
        assert_eq!(" true a", struct_str.ref_mut_method(true, 'a'));
        assert_eq!("", struct_str.0);
        assert_eq!("0 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(0, struct_4.0);
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let mut struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        let mut struct_str = Struct("str");
        let mut struct_4 = Struct(4u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_mut_method.mock_raw(|_, b, c|
                MockResult::Continue((as_mut(&struct_3), !b, c.to_ascii_uppercase())));
        }

        assert_eq!("0 false A", struct_2.ref_mut_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(0, struct_3.0);
        assert_eq!(" true a", struct_str.ref_mut_method(true, 'a'));
        assert_eq!("", struct_str.0);
        assert_eq!("0 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(0, struct_4.0);
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let mut struct_2 = Struct(2u8);
        let mut struct_str = Struct("str");
        let mut struct_4 = Struct(4u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_mut_method.mock_raw(|a, b, c|
                MockResult::Return(format!("mocked {} {} {}", a.0, b, c), ));
        }

        assert_eq!("mocked 2 true a", struct_2.ref_mut_method(true, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(" true a", struct_str.ref_mut_method(true, 'a'));
        assert_eq!("", struct_str.0);
        assert_eq!("0 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(0, struct_4.0);
    }
}

mod and_method_is_val_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true a", Struct(2u8).val_method(true, 'a'));
        assert_eq!("abc true a", Struct("abc").val_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2u8).val_method(true, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            <Struct<u8> as Trait<char>>::val_method.mock_raw(move |_, b, c|
                MockResult::Continue((Struct(3u8), !b, c.to_ascii_uppercase())));
        }

        assert_eq!("3 false A", Struct(2u8).val_method(true, 'a'));
        assert_eq!("abc true a", Struct("abc").val_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2u8).val_method(true, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            <Struct<u8> as Trait<char>>::val_method.mock_raw(|a, b, c|
                MockResult::Return(format!("mocked {} {} {}", a.0, b, c), ));
        }

        assert_eq!("mocked 2 true a", Struct(2u8).val_method(true, 'a'));
        assert_eq!("abc true a", Struct("abc").val_method(true, 'a'));
        assert_eq!("2 true abc", Struct(2u8).val_method(true, "abc"));
    }
}
