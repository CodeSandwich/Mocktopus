use super::*;

struct Struct(u8);

#[mockable]
impl Struct {
    async fn static_method(arg: bool) -> String {
        format!("{}", arg)
    }

    async fn ref_method(&self, arg: bool) -> String {
        format!("{} {}", self.0, arg)
    }

    async fn ref_mut_method(&mut self, arg: bool) -> String {
        self.0 *= 2;
        format!("{} {}", self.0, arg)
    }

    async fn val_method(self, arg: bool) -> String {
        format!("{} {}", self.0, arg)
    }
}

mod and_async_method_is_static {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("true", Struct::static_method(true).await);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            Struct::static_method.mock_raw(|a| MockResult::Continue((!a,)));
        }

        assert_eq!("false", Struct::static_method(true).await);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            Struct::static_method
                .mock_raw(|a| MockResult::Return(Box::pin(async move { format!("mocked {}", a) })));
        }

        assert_eq!("mocked true", Struct::static_method(true).await);
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true", Struct(2).ref_method(true).await);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2);
        let struct_3 = Struct(3);
        unsafe {
            Struct::ref_method.mock_raw(|_, b| MockResult::Continue((&struct_3, !b)));
        }

        assert_eq!("3 false", struct_2.ref_method(true).await);
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2);
        unsafe {
            Struct::ref_method.mock_raw(|a, b| {
                MockResult::Return(Box::pin(async move { format!("mocked {} {}", a.0, b) }))
            });
        }

        assert_eq!("mocked 2 true", struct_2.ref_method(true).await);
        assert_eq!(2, struct_2.0);
    }
}

mod and_async_method_is_ref_mut_method {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        let mut struct_2 = Struct(2);

        assert_eq!("4 true", struct_2.ref_mut_method(true).await);
        assert_eq!(4, struct_2.0);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        let mut struct_2 = Struct(2);
        let struct_3 = Struct(3);
        unsafe {
            Struct::ref_mut_method.mock_raw(|_, b| MockResult::Continue((as_mut(&struct_3), !b)));
        }

        assert_eq!("6 false", struct_2.ref_mut_method(true).await);
        assert_eq!(2, struct_2.0);
        assert_eq!(6, struct_3.0);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        let mut struct_2 = Struct(2);
        unsafe {
            Struct::ref_mut_method.mock_raw(|a, b| {
                MockResult::Return(Box::pin(async move { format!("mocked {} {}", a.0, b) }))
            });
        }

        assert_eq!("mocked 2 true", struct_2.ref_mut_method(true).await);
        assert_eq!(2, struct_2.0);
    }
}

mod and_async_method_is_val_method {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true", Struct(2).val_method(true).await);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            Struct::val_method.mock_raw(move |_, b| MockResult::Continue((Struct(3), !b)));
        }

        assert_eq!("3 false", Struct(2).val_method(true).await);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            Struct::val_method.mock_raw(|a, b| {
                MockResult::Return(Box::pin(async move { format!("mocked {} {}", a.0, b) }))
            });
        }

        assert_eq!("mocked 2 true", Struct(2).val_method(true).await);
    }
}
