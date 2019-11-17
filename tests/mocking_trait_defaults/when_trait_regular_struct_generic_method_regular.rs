use super::*;

#[mockable]
trait Trait: Sized {
    fn static_method(arg: bool) -> String {
        format!("{}", arg)
    }

    fn ref_method(&self, arg: bool) -> String {
        format!("{} {}", self.to_string(), arg)
    }

    fn ref_mut_method(&mut self, arg: bool) -> String {
        self.modify();
        format!("{} {}", self.to_string(), arg)
    }

    fn val_method(self, arg: bool) -> String {
        format!("{} {}", self.to_string(), arg)
    }

    fn to_string(&self) -> String;
    fn modify(&mut self);
}

struct Struct<T>(T);

impl<T: Display + Default> Trait for Struct<T> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn modify(&mut self) {
        self.0 = T::default();
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true", Struct::<u8>::static_method(true));
        assert_eq!("true", Struct::<&str>::static_method(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        unsafe {
            Struct::<u8>::static_method.mock_raw(|a| MockResult::Continue((!a,)));
        }

        assert_eq!("false", Struct::<u8>::static_method(true));
        assert_eq!("true", Struct::<&str>::static_method(true));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        unsafe {
            Struct::<u8>::static_method.mock_raw(|a| MockResult::Return(format!("mocked {}", a)));
        }

        assert_eq!("mocked true", Struct::<u8>::static_method(true));
        assert_eq!("true", Struct::<&str>::static_method(true));
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true", Struct(2u8).ref_method(true));
        assert_eq!("abc true", Struct("abc").ref_method(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        unsafe {
            Struct::<u8>::ref_method.mock_raw(|_, b| MockResult::Continue((&struct_3, !b)));
        }

        assert_eq!("3 false", struct_2.ref_method(true));
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
        assert_eq!("abc true", Struct("abc").ref_method(true));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2u8);
        unsafe {
            Struct::<u8>::ref_method
                .mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a.0, b)));
        }

        assert_eq!("mocked 2 true", struct_2.ref_method(true));
        assert_eq!(2, struct_2.0);
        assert_eq!("abc true", Struct("abc").ref_method(true));
    }
}

mod and_method_is_ref_mut_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        let mut struct_2 = Struct(2u8);
        let mut struct_str = Struct("str");

        assert_eq!("0 true", struct_2.ref_mut_method(true));
        assert_eq!(0, struct_2.0);
        assert_eq!(" true", struct_str.ref_mut_method(true));
        assert_eq!("", struct_str.0);
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let mut struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        let mut struct_str = Struct("str");
        unsafe {
            Struct::<u8>::ref_mut_method
                .mock_raw(|_, b| MockResult::Continue((as_mut(&struct_3), !b)));
        }

        assert_eq!("0 false", struct_2.ref_mut_method(true));
        assert_eq!(2, struct_2.0);
        assert_eq!(0, struct_3.0);
        assert_eq!(" true", struct_str.ref_mut_method(true));
        assert_eq!("", struct_str.0);
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let mut struct_2 = Struct(2u8);
        let mut struct_str = Struct("str");
        unsafe {
            Struct::<u8>::ref_mut_method
                .mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a.0, b)));
        }

        assert_eq!("mocked 2 true", struct_2.ref_mut_method(true));
        assert_eq!(2, struct_2.0);
        assert_eq!(" true", struct_str.ref_mut_method(true));
        assert_eq!("", struct_str.0);
    }
}

mod and_method_is_val_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true", Struct(2u8).val_method(true));
        assert_eq!("abc true", Struct("abc").val_method(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            Struct::<u8>::val_method.mock_raw(move |_, b| MockResult::Continue((Struct(3u8), !b)));
        }

        assert_eq!("3 false", Struct(2u8).val_method(true));
        assert_eq!("abc true", Struct("abc").val_method(true));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            Struct::<u8>::val_method
                .mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a.0, b)));
        }

        assert_eq!("mocked 2 true", Struct(2u8).val_method(true));
        assert_eq!("abc true", Struct("abc").val_method(true));
    }
}
