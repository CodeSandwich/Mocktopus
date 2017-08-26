/// Converts non-mutable reference to a mutable one
///
/// Allows creating multiple mutable references to a single item breaking Rust's safety policy.
/// # Safety
/// Use with extreme caution, may cause all sorts of mutability related undefined behaviors!
///
/// One safe use case is when mocking function, which gets called only once during whole test execution, for example:
///
/// ```
/// #[mockable]
/// impl Context {
/// fn get_string(&mut self) -> &mut String {
///     // retrieve string
/// }
///
/// fn modify_string(context: &mut Context) {
///     context.get_string().push_str("modified")
/// }
///
/// #[test]
/// fn append_string_test() {
///     let string = String::new();
///     unsafe {
///         // MockResult::Return(&mut string) would fail
///         Context::get_string.mock_raw(|| MockResult::Return(as_mut(&string)));
///     }
///
///     modify_string(&mut Context::default());
///
///     assert_eq!("modified", string);
/// }
/// ```
pub unsafe fn as_mut<T>(t_ref: &T) -> &mut T {
    &mut *(t_ref as *const T as *mut T)
}
