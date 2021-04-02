use super::*;

struct Struct(u8);

fn plus_one(value: u8) -> u8 {
    value + 1
}

#[mockable]
impl Struct {
    async fn ref_method_with_binding(&self, arg: bool) -> String {
        let method_call = || async { self.0 };

        format!("{} {}", method_call().await, arg)
    }

    async fn ref_method_with_ref(&self, arg: &bool) -> String {
        format!("{} {}", self.0, arg)
    }

    async fn ref_method_with_call(&self, arg: bool) -> String {
        format!("{} {}", plus_one(self.0), arg)
    }

    async fn assoc_method_with_ref(arg: &str) -> String {
        format!("Hello {}", arg)
    }

    async fn assoc_method_with_ref_and_lifetime<'a>(arg1: &'a str, arg2: &'a str) -> String {
        format!("{} {}", arg1, arg2)
    }
}

mod and_method_is_ref_method_with_binding {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true", Struct(2).ref_method_with_binding(true).await);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2);
        let struct_3 = Struct(3);
        unsafe {
            Struct::ref_method_with_binding.mock_raw(|_, b| MockResult::Continue((&struct_3, !b)));
        }

        assert_eq!("3 false", struct_2.ref_method_with_binding(true).await);
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2);
        unsafe {
            Struct::ref_method_with_binding.mock_raw(|a, b| {
                MockResult::Return(Box::pin(async move { format!("mocked {} {}", a.0, b) }))
            });
        }

        assert_eq!(
            "mocked 2 true",
            struct_2.ref_method_with_binding(true).await
        );
        assert_eq!(2, struct_2.0);
    }
}

mod and_method_is_ref_method_with_ref {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("2 true", Struct(2).ref_method_with_ref(&true).await);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2);
        unsafe {
            Struct::ref_method_with_ref.mock_raw(|a, b| {
                MockResult::Return(Box::pin(async move { format!("mocked {} {}", a.0, b) }))
            });
        }

        assert_eq!("mocked 2 true", struct_2.ref_method_with_ref(&true).await);
        assert_eq!(2, struct_2.0);
    }
}

mod and_method_is_ref_method_with_call {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("3 true", Struct(2).ref_method_with_call(true).await);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        let struct_2 = Struct(2);
        let struct_3 = Struct(3);
        unsafe {
            Struct::ref_method_with_call.mock_raw(|_, b| MockResult::Continue((&struct_3, !b)));
        }

        assert_eq!("4 false", struct_2.ref_method_with_call(true).await);
        assert_eq!(2, struct_2.0);
        assert_eq!(3, struct_3.0);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        let struct_2 = Struct(2);
        unsafe {
            Struct::ref_method_with_call.mock_raw(|a, b| {
                MockResult::Return(Box::pin(async move { format!("mocked {} {}", a.0, b) }))
            });
        }

        assert_eq!("mocked 2 true", struct_2.ref_method_with_call(true).await);
        assert_eq!(2, struct_2.0);
    }
}

mod and_method_is_assoc_method_with_ref {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("Hello World", Struct::assoc_method_with_ref("World").await);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            Struct::assoc_method_with_ref.mock_raw(|_| MockResult::Continue(("Universe",)));
        }

        assert_eq!("Hello Universe", Struct::assoc_method_with_ref("World").await);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            Struct::assoc_method_with_ref.mock_raw(|s| {
                MockResult::Return(Box::pin(async move { format!("Welcome {}", s) }))
            });
        }

        assert_eq!("Welcome World", Struct::assoc_method_with_ref("World").await);
    }
}

mod and_method_is_assoc_method_with_ref_and_lifetime {
    use super::*;

    #[tokio::test]
    async fn and_not_mocked_then_runs_normally() {
        assert_eq!("Hello World", Struct::assoc_method_with_ref_and_lifetime("Hello", "World").await);
    }

    #[tokio::test]
    async fn and_continue_mocked_then_runs_with_modified_args() {
        unsafe {
            Struct::assoc_method_with_ref_and_lifetime.mock_raw(|s1, _| MockResult::Continue((s1, "Universe",)));
        }

        assert_eq!("Hello Universe", Struct::assoc_method_with_ref_and_lifetime("Hello", "World").await);
    }

    #[tokio::test]
    async fn and_return_mocked_then_returns_mocking_result() {
        unsafe {
            Struct::assoc_method_with_ref_and_lifetime.mock_raw(|_, s2| {
                MockResult::Return(Box::pin(async move { format!("Welcome {}", s2) }))
            });
        }

        assert_eq!("Welcome World", Struct::assoc_method_with_ref_and_lifetime("Hello", "World").await);
    }
}
