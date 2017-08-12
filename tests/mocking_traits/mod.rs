use super::*;

mod when_trait_generic_struct_generic_method_generic;
mod when_trait_generic_struct_generic_method_regular;
mod when_trait_generic_struct_regular_method_generic;
mod when_trait_generic_struct_regular_method_regular;
mod when_trait_regular_struct_generic_method_generic;
mod when_trait_regular_struct_generic_method_regular;
mod when_trait_regular_struct_regular_method_generic;
mod when_trait_regular_struct_regular_method_regular;

mod mocking_impls_of_traits_with_path {
    use super::*;
    use self::trait_mod::Trait;

    struct Struct();

    mod trait_mod {
        pub trait Trait {
            fn method() -> &'static str;
        }
    }

    #[mockable]
    impl self::trait_mod::Trait for Struct {
        fn method() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn mocks_successfully() {
        unsafe {
            Struct::method.mock_raw(|| MockResult::Return("mocked"));
        }

        assert_eq!("mocked", Struct::method());
    }
}

mod mocking_impls_of_traits_generic_over_generic_refs {
    use super::*;

    struct Struct();

    trait Trait<T> {
        fn method() -> &'static str;
    }

    #[mockable]
    impl<'a, T> Trait<&'a T> for Struct {
        fn method() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn mocks_successfully() {
        unsafe {
            <Struct as Trait<&u32>>::method.mock_raw(|| MockResult::Return("mocked"));
        }

        assert_eq!("mocked", <Struct as Trait<&u32>>::method());
    }
}
