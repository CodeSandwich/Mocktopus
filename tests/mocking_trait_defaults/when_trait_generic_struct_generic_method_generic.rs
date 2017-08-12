use super::*;

#[mockable]
trait Trait<V: Display>: Sized {
    fn static_method<U: Display>(arg: bool, method_generic: U, trait_generic: V) -> String {
        format!("{} {} {}", arg, method_generic, trait_generic)
    }

    fn ref_method<U: Display>(&self, arg: bool, method_generic: U, trait_generic: V) -> String {
        format!("{} {} {} {}", self.to_string(), arg, method_generic, trait_generic)
    }

    fn ref_mut_method<U: Display>(&mut self, arg: bool, method_generic: U, trait_generic: V) -> String {
        self.modify();
        format!("{} {} {} {}", self.to_string(), arg, method_generic, trait_generic)
    }

    fn val_method<U: Display>(self, arg: bool, method_generic: U, trait_generic: V) -> String {
        format!("{} {} {} {}", self.to_string(), arg, method_generic, trait_generic)
    }

    fn to_string(&self) -> String;
    fn modify(&mut self);
}

struct Struct<T>(T);

impl<T: Display + Default, V: Display> Trait<V> for Struct<T> {
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
        assert_eq!("true 2.5 a", Struct::<u8>::static_method(true, 2.5f32, 'a'));
        assert_eq!("true abc a", Struct::<u8>::static_method(true, "abc", 'a'));
        assert_eq!("true 2.5 a", Struct::<&str>::static_method(true, 2.5f32, 'a'));
        assert_eq!("true 2.5 abc", Struct::<u8>::static_method(true, 2.5f32, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        unsafe {
            <Struct<u8> as Trait<char>>::static_method::<f32>.mock_raw(|a, b, c|
                MockResult::Continue((!a, b + 1., c.to_ascii_uppercase())));
        }

        assert_eq!("false 3.5 A", Struct::<u8>::static_method(true, 2.5f32, 'a'));
        assert_eq!("true abc a", Struct::<u8>::static_method(true, "abc", 'a'));
        assert_eq!("true 2.5 a", Struct::<&str>::static_method(true, 2.5f32, 'a'));
        assert_eq!("true 2.5 abc", Struct::<u8>::static_method(true, 2.5f32, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        unsafe {
            <Struct<u8> as Trait<char>>::static_method::<f32>.mock_raw(|a, b, c|
                MockResult::Return(format!("mocked {} {} {}", a, b, c), ));
        }

        assert_eq!("mocked true 2.5 a", Struct::<u8>::static_method(true, 2.5f32, 'a'));
        assert_eq!("true abc a", Struct::<u8>::static_method(true, "abc", 'a'));
        assert_eq!("true 2.5 a", Struct::<&str>::static_method(true, 2.5f32, 'a'));
        assert_eq!("true 2.5 abc", Struct::<u8>::static_method(true, 2.5f32, "abc"));
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true 1.5 a", Struct(2u8).ref_method(true, 1.5f32, 'a'));
        assert_eq!("2 true abc a", Struct(2u8).ref_method(true, "abc", 'a'));
        assert_eq!("abc true 1.5 a", Struct("abc").ref_method(true, 1.5f32, 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2u8).ref_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_method::<f32>.mock_raw(|_, b, c, d|
                MockResult::Continue((&struct_3, !b, c + 1., d.to_ascii_uppercase())));
        }

        assert_eq!("3 false 2.5 A", struct_2.ref_method(true, 1.5f32, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
        assert_eq!("2 true abc a", Struct(2u8).ref_method(true, "abc", 'a'));
        assert_eq!("abc true 1.5 a", Struct("abc").ref_method(true, 1.5f32, 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2u8).ref_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_method::<f32>.mock_raw(|a, b, c, d|
                MockResult::Return(format!("mocked {} {} {} {}", a.0, b, c, d), ));
        }

        assert_eq!("mocked 2 true 1.5 a", struct_2.ref_method(true, 1.5f32, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!("2 true abc a", Struct(2u8).ref_method(true, "abc", 'a'));
        assert_eq!("abc true 1.5 a", Struct("abc").ref_method(true, 1.5f32, 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2u8).ref_method(true, 1.5f32, "abc"));
    }
}

mod and_method_is_ref_mut_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        let mut struct_2 = Struct(2u8);
        let mut struct_4 = Struct(4u8);
        let mut struct_str = Struct("abc");
        let mut struct_5 = Struct(5u8);

        assert_eq!("0 true 1.5 a", struct_2.ref_mut_method(true, 1.5f32, 'a'));
        assert_eq!(0, struct_2.0);
        assert_eq!("0 true abc a", struct_4.ref_mut_method(true, "abc", 'a'));
        assert_eq!(0, struct_4.0);
        assert_eq!(" true 1.5 a", struct_str.ref_mut_method(true, 1.5, 'a'));
        assert_eq!("", struct_str.0);
        assert_eq!("0 true 1.5 abc", struct_5.ref_mut_method(true, 1.5f32, "abc"));
        assert_eq!(0, struct_5.0);
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let mut struct_2 = Struct(2u8);
        let struct_3 = Struct(3u8);
        let mut struct_4 = Struct(4u8);
        let mut struct_str = Struct("abc");
        let mut struct_5 = Struct(5u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_mut_method::<f32>.mock_raw(|_, b, c, d|
                MockResult::Continue((as_mut(&struct_3), !b, c + 1., d.to_ascii_uppercase())));
        }

        assert_eq!("0 false 2.5 A", struct_2.ref_mut_method(true, 1.5f32, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(0, struct_3.0);
        assert_eq!("0 true abc a", struct_4.ref_mut_method(true, "abc", 'a'));
        assert_eq!(0, struct_4.0);
        assert_eq!(" true 1.5 a", struct_str.ref_mut_method(true, 1.5, 'a'));
        assert_eq!("", struct_str.0);
        assert_eq!("0 true 1.5 abc", struct_5.ref_mut_method(true, 1.5f32, "abc"));
        assert_eq!(0, struct_5.0);
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let mut struct_2 = Struct(2u8);
        let mut struct_4 = Struct(4u8);
        let mut struct_str = Struct("abc");
        let mut struct_5 = Struct(5u8);
        unsafe {
            <Struct<u8> as Trait<char>>::ref_mut_method::<f32>.mock_raw(|a, b, c, d|
                MockResult::Return(format!("mocked {} {} {} {}", a.0, b, c, d), ));
        }

        assert_eq!("mocked 2 true 1.5 a", struct_2.ref_mut_method(true, 1.5f32, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!("0 true abc a", struct_4.ref_mut_method(true, "abc", 'a'));
        assert_eq!(0, struct_4.0);
        assert_eq!(" true 1.5 a", struct_str.ref_mut_method(true, 1.5, 'a'));
        assert_eq!("", struct_str.0);
        assert_eq!("0 true 1.5 abc", struct_5.ref_mut_method(true, 1.5f32, "abc"));
        assert_eq!(0, struct_5.0);
    }
}

mod and_method_is_val_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true 1.5 a", Struct(2u8).val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true abc a", Struct(2u8).val_method(true, "abc", 'a'));
        assert_eq!("abc true 1.5 a", Struct("abc").val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2u8).val_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            <Struct<u8> as Trait<char>>::val_method::<f32>.mock_raw(move |_, b, c, d|
                MockResult::Continue((Struct(3u8), !b, c + 1., d.to_ascii_uppercase())));
        }

        assert_eq!("3 false 2.5 A", Struct(2u8).val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true abc a", Struct(2u8).val_method(true, "abc", 'a'));
        assert_eq!("abc true 1.5 a", Struct("abc").val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2u8).val_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            <Struct<u8> as Trait<char>>::val_method::<f32>.mock_raw(|a, b, c, d|
                MockResult::Return(format!("mocked {} {} {} {}", a.0, b, c, d), ));
        }

        assert_eq!("mocked 2 true 1.5 a", Struct(2u8).val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true abc a", Struct(2u8).val_method(true, "abc", 'a'));
        assert_eq!("abc true 1.5 a", Struct("abc").val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2u8).val_method(true, 1.5f32, "abc"));
    }
}
