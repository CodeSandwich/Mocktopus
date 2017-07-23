use super::*;

struct Struct(u8);

static STRUCT_3: Struct = Struct(3);

#[inject_mocks]
impl Struct {
    fn static_method(arg: bool) -> String {
        format!("{}", arg)
    }

    fn ref_method(&self, arg: bool) -> String {
        format!("{} {}", self.0, arg)
    }

    fn ref_mut_method(&mut self, arg: bool) -> String {
        self.0 *= 2;
        format!("{} {}", self.0, arg)
    }

    fn val_method(self, arg: bool) -> String {
        format!("{} {}", self.0, arg)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true", Struct::static_method(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        Struct::static_method.mock_raw(|a| MockResult::Continue((!a,)));

        assert_eq!("false", Struct::static_method(true));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        Struct::static_method.mock_raw(|a| MockResult::Return(format!("mocked {}", a),));

        assert_eq!("mocked true", Struct::static_method(true));
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true", Struct(2).ref_method(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        Struct::ref_method.mock_raw(|_, b| MockResult::Continue((&STRUCT_3, !b)));
        let struct_2 = Struct(2);

        assert_eq!("3 false", struct_2.ref_method(true));
        assert_eq!(2, struct_2.0);
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        Struct::ref_method.mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a.0, b),));
        let struct_2 = Struct(2);

        assert_eq!("mocked 2 true", struct_2.ref_method(true));
        assert_eq!(2, struct_2.0);
    }
}
