use super::*;

trait Trait<V: Display> {
    fn static_method<T: Display>(arg: bool, method_generic: T, trait_generic: V) -> String;
    fn ref_method<T: Display>(&self, arg: bool, method_generic: T, trait_generic: V) -> String;
    fn ref_mut_method<T: Display>(
        &mut self,
        arg: bool,
        method_generic: T,
        trait_generic: V,
    ) -> String;
    fn val_method<T: Display>(self, arg: bool, method_generic: T, trait_generic: V) -> String;
}

struct Struct(u8);

#[mockable]
impl<V: Display> Trait<V> for Struct {
    fn static_method<T: Display>(arg: bool, method_generic: T, trait_generic: V) -> String {
        format!("{} {} {}", arg, method_generic, trait_generic)
    }

    fn ref_method<T: Display>(&self, arg: bool, method_generic: T, trait_generic: V) -> String {
        format!("{} {} {} {}", self.0, arg, method_generic, trait_generic)
    }

    fn ref_mut_method<T: Display>(
        &mut self,
        arg: bool,
        method_generic: T,
        trait_generic: V,
    ) -> String {
        self.0 *= 2;
        format!("{} {} {} {}", self.0, arg, method_generic, trait_generic)
    }

    fn val_method<T: Display>(self, arg: bool, method_generic: T, trait_generic: V) -> String {
        format!("{} {} {} {}", self.0, arg, method_generic, trait_generic)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true 1.5 a", Struct::static_method(true, 1.5f32, 'a'));
        assert_eq!("true abc a", Struct::static_method(true, "abc", 'a'));
        assert_eq!("true 1.5 abc", Struct::static_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        unsafe {
            <Struct as Trait<char>>::static_method::<f32>
                .mock_raw(|a, b, c| MockResult::Continue((!a, b + 1., c.to_ascii_uppercase())));
        }

        assert_eq!("false 3.5 A", Struct::static_method(true, 2.5f32, 'a'));
        assert_eq!("true abc a", Struct::static_method(true, "abc", 'a'));
        assert_eq!("true 1.5 abc", Struct::static_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        unsafe {
            <Struct as Trait<char>>::static_method::<f32>
                .mock_raw(|a, b, c| MockResult::Return(format!("mocked {} {} {}", a, b, c)));
        }

        assert_eq!(
            "mocked true 2.5 a",
            Struct::static_method(true, 2.5f32, 'a')
        );
        assert_eq!("true abc a", Struct::static_method(true, "abc", 'a'));
        assert_eq!("true 1.5 abc", Struct::static_method(true, 1.5f32, "abc"));
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true 1.5 a", Struct(2).ref_method(true, 1.5f32, 'a'));
        assert_eq!("2 true abc a", Struct(2).ref_method(true, "abc", 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2).ref_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2);
        let struct_3 = Struct(3);
        unsafe {
            <Struct as Trait<char>>::ref_method::<f32>.mock_raw(|_, b, c, d| {
                MockResult::Continue((&struct_3, !b, c + 1., d.to_ascii_uppercase()))
            });
        }

        assert_eq!("3 false 2.5 A", struct_2.ref_method(true, 1.5f32, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
        assert_eq!("2 true abc a", Struct(2).ref_method(true, "abc", 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2).ref_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2);
        unsafe {
            <Struct as Trait<char>>::ref_method::<f32>.mock_raw(|a, b, c, d| {
                MockResult::Return(format!("mocked {} {} {} {}", a.0, b, c, d))
            });
        }

        assert_eq!(
            "mocked 2 true 1.5 a",
            struct_2.ref_method(true, 1.5f32, 'a')
        );
        assert_eq!(2, struct_2.0);
        assert_eq!("2 true abc a", Struct(2).ref_method(true, "abc", 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2).ref_method(true, 1.5f32, "abc"));
    }
}

mod and_method_is_ref_mut_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        let mut struct_2 = Struct(2);
        let mut struct_4 = Struct(4);
        let mut struct_5 = Struct(5);

        assert_eq!("4 true 1.5 a", struct_2.ref_mut_method(true, 1.5f32, 'a'));
        assert_eq!(4, struct_2.0);
        assert_eq!("8 true abc a", struct_4.ref_mut_method(true, "abc", 'a'));
        assert_eq!(8, struct_4.0);
        assert_eq!(
            "10 true 1.5 abc",
            struct_5.ref_mut_method(true, 1.5f32, "abc")
        );
        assert_eq!(10, struct_5.0);
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        let mut struct_2 = Struct(2);
        let struct_3 = Struct(3);
        let mut struct_4 = Struct(4);
        let mut struct_5 = Struct(5);
        unsafe {
            <Struct as Trait<char>>::ref_mut_method::<f32>.mock_raw(|_, b, c, d| {
                MockResult::Continue((as_mut(&struct_3), !b, c + 1., d.to_ascii_uppercase()))
            });
        }

        assert_eq!("6 false 2.5 A", struct_2.ref_mut_method(true, 1.5f32, 'a'));
        assert_eq!(2, struct_2.0);
        assert_eq!(6, struct_3.0);
        assert_eq!("8 true abc a", struct_4.ref_mut_method(true, "abc", 'a'));
        assert_eq!(8, struct_4.0);
        assert_eq!(
            "10 true 1.5 abc",
            struct_5.ref_mut_method(true, 1.5f32, "abc")
        );
        assert_eq!(10, struct_5.0);
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        let mut struct_2 = Struct(2);
        let mut struct_4 = Struct(4);
        let mut struct_5 = Struct(5);
        unsafe {
            <Struct as Trait<char>>::ref_mut_method::<f32>.mock_raw(|a, b, c, d| {
                MockResult::Return(format!("mocked {} {} {} {}", a.0, b, c, d))
            });
        }

        assert_eq!(
            "mocked 2 true 1.5 a",
            struct_2.ref_mut_method(true, 1.5f32, 'a')
        );
        assert_eq!(2, struct_2.0);
        assert_eq!("8 true abc a", struct_4.ref_mut_method(true, "abc", 'a'));
        assert_eq!(8, struct_4.0);
        assert_eq!(
            "10 true 1.5 abc",
            struct_5.ref_mut_method(true, 1.5f32, "abc")
        );
        assert_eq!(10, struct_5.0);
    }
}

mod and_method_is_val_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true 1.5 a", Struct(2).val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true abc a", Struct(2).val_method(true, "abc", 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2).val_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            <Struct as Trait<char>>::val_method::<f32>.mock_raw(move |_, b, c, d| {
                MockResult::Continue((Struct(3), !b, c + 1., d.to_ascii_uppercase()))
            });
        }

        assert_eq!("3 false 2.5 A", Struct(2).val_method(true, 1.5f32, 'a'));
        assert_eq!("2 true abc a", Struct(2).val_method(true, "abc", 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2).val_method(true, 1.5f32, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            <Struct as Trait<char>>::val_method::<f32>.mock_raw(|a, b, c, d| {
                MockResult::Return(format!("mocked {} {} {} {}", a.0, b, c, d))
            });
        }

        assert_eq!(
            "mocked 2 true 1.5 a",
            Struct(2).val_method(true, 1.5f32, 'a')
        );
        assert_eq!("2 true abc a", Struct(2).val_method(true, "abc", 'a'));
        assert_eq!("2 true 1.5 abc", Struct(2).val_method(true, 1.5f32, "abc"));
    }
}
