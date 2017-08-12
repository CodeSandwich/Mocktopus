use super::*;

trait Trait {
    fn static_method<U: Display>(arg: bool, method_generic: U) -> String;
    fn ref_method<U: Display>(&self, arg: bool, method_generic: U) -> String;
    fn ref_mut_method<U: Display>(&mut self, arg: bool, method_generic: U) -> String;
    fn val_method<U: Display>(self, arg: bool, method_generic: U) -> String;
}

struct Struct<T>(T);

#[mockable]
impl<T: Display + Default> Trait for Struct<T> {
    fn static_method<U: Display>(arg: bool, method_generic: U) -> String {
        format!("{} {}", arg, method_generic)
    }

    fn ref_method<U: Display>(&self, arg: bool, method_generic: U) -> String {
        format!("{} {} {}", self.0, arg, method_generic)
    }

    fn ref_mut_method<U: Display>(&mut self, arg: bool, method_generic: U) -> String {
        self.0 = T::default();
        format!("{} {} {}", self.0, arg, method_generic)
    }

    fn val_method<U: Display>(self, arg: bool, method_generic: U) -> String {
        format!("{} {} {}", self.0, arg, method_generic)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true 2.5", Struct::<u8>::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
        assert_eq!("true 2.5", Struct::<&str>::static_method(true, 2.5f32));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        unsafe {
            Struct::<u8>::static_method::<f32>.mock_raw(|a, b| MockResult::Continue((!a, b + 1.)));
        }

        assert_eq!("false 3.5", Struct::<u8>::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
        assert_eq!("true 2.5", Struct::<&str>::static_method(true, 2.5f32));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        unsafe {
            Struct::<u8>::static_method::<f32>.mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a, b), ));
        }

        assert_eq!("mocked true 2.5", Struct::<u8>::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
        assert_eq!("true 2.5", Struct::<&str>::static_method(true, 2.5f32));
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true 1.5", Struct(2u8).ref_method(true, 1.5f32));
        assert_eq!("2 true abc", Struct(2u8).ref_method(true, "abc"));
        assert_eq!("abc true 1.5", Struct("abc").ref_method(true, 1.5f32));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        unsafe {
            Struct::<u8>::ref_method::<f32>.mock_raw(|_, b, c| MockResult::Continue((&struct_3, !b, c + 1.)));
        }

        assert_eq!("3 false 2.5", struct_2.ref_method(true, 1.5f32));
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
        assert_eq!("2 true abc", Struct(2u8).ref_method(true, "abc"));
        assert_eq!("abc true 1.5", Struct("abc").ref_method(true, 1.5f32));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2u8);
        unsafe {
            Struct::<u8>::ref_method::<f32>.mock_raw(|a, b, c| MockResult::Return(format!("mocked {} {} {}", a.0, b, c), ));
        }

        assert_eq!("mocked 2 true 1.5", struct_2.ref_method(true, 1.5f32));
        assert_eq!(2, struct_2.0);
        assert_eq!("2 true abc", Struct(2).ref_method(true, "abc"));
        assert_eq!("abc true 1.5", Struct("abc").ref_method(true, 1.5f32));
    }
}

mod and_method_is_ref_mut_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        let mut struct_2 = Struct(2u8);
        let mut struct_4 = Struct(4u8);
        let mut struct_str = Struct("abc");

        assert_eq!("0 true 1.5", struct_2.ref_mut_method(true, 1.5f32));
        assert_eq!(0, struct_2.0);
        assert_eq!("0 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(0, struct_4.0);
        assert_eq!(" true 1.5", struct_str.ref_mut_method(true, 1.5));
        assert_eq!("", struct_str.0);
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let mut struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        let mut struct_4 = Struct(4u8);
        let mut struct_str = Struct("abc");
        unsafe {
            Struct::<u8>::ref_mut_method::<f32>.mock_raw(|_, b, c|
                MockResult::Continue((as_mut(&struct_3), !b, c + 1.)));
        }

        assert_eq!("0 false 2.5", struct_2.ref_mut_method(true, 1.5f32));
        assert_eq!(2, struct_2.0);
        assert_eq!(0, struct_3.0);
        assert_eq!("0 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(0, struct_4.0);
        assert_eq!(" true 1.5", struct_str.ref_mut_method(true, 1.5));
        assert_eq!("", struct_str.0);
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let mut struct_2 = Struct(2u8);
        let mut struct_4 = Struct(4u8);
        let mut struct_str = Struct("abc");
        unsafe {
            Struct::<u8>::ref_mut_method::<f32>.mock_raw(|a, b, c|
                MockResult::Return(format!("mocked {} {} {}", a.0, b, c), ));
        }

        assert_eq!("mocked 2 true 1.5", struct_2.ref_mut_method(true, 1.5f32));
        assert_eq!(2, struct_2.0);
        assert_eq!("0 true abc", struct_4.ref_mut_method(true, "abc"));
        assert_eq!(0, struct_4.0);
        assert_eq!(" true 1.5", struct_str.ref_mut_method(true, 1.5));
        assert_eq!("", struct_str.0);
    }
}

mod and_method_is_val_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true 1.5", Struct(2u8).val_method(true, 1.5f32));
        assert_eq!("2 true abc", Struct(2u8).val_method(true, "abc"));
        assert_eq!("abc true 1.5", Struct("abc").val_method(true, 1.5f32));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            Struct::<u8>::val_method::<f32>.mock_raw(move |_, b, c| MockResult::Continue((Struct(3u8), !b, c + 1.)));
        }

        assert_eq!("3 false 2.5", Struct(2u8).val_method(true, 1.5f32));
        assert_eq!("2 true abc", Struct(2u8).val_method(true, "abc"));
        assert_eq!("abc true 1.5", Struct("abc").val_method(true, 1.5f32));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            Struct::<u8>::val_method::<f32>.mock_raw(|a, b, c| MockResult::Return(format!("mocked {} {} {}", a.0, b, c), ));
        }

        assert_eq!("mocked 2 true 1.5", Struct(2u8).val_method(true, 1.5f32));
        assert_eq!("2 true abc", Struct(2u8).val_method(true, "abc"));
        assert_eq!("abc true 1.5", Struct("abc").val_method(true, 1.5f32));
    }
}
