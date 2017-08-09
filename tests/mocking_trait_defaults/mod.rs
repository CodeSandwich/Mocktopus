use super::*;

mod when_trait_generic_struct_generic_method_generic;
mod when_trait_generic_struct_generic_method_regular;
mod when_trait_generic_struct_regular_method_generic;
mod when_trait_generic_struct_regular_method_regular;
mod when_trait_regular_struct_generic_method_generic;
mod when_trait_regular_struct_generic_method_regular;
mod when_trait_regular_struct_regular_method_generic;
mod when_trait_regular_struct_regular_method_regular;

mod mocking_default_impl_of_trait_of_struct {
    use super::*;

    #[inject_mocks]
    trait Trait {
        fn method() -> &'static str {
            "not mocked"
        }
    }

    struct Struct1;

    impl Trait for Struct1 {

    }

    struct Struct2;

    impl Trait for Struct2 {

    }

    #[test]
    fn does_not_mock_default_impl_of_other_struct() {
        Struct1::method.mock_raw(|| MockResult::Return("mocked"));

        assert_eq!("mocked", Struct1::method());
        assert_eq!("not mocked", Struct2::method());
    }
}
