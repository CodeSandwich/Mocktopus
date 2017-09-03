use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::transmute;

/// Trait for setting up mocks
///
/// The trait is implemented for all functions, so its methods can be called on any function.
///
/// Note: methods have any effect only if called on functions [annotated as mockable](https://docs.rs/mocktopus_macros).
pub trait Mockable<T, O> {
    /// Core function for setting up mocks
    ///
    /// The passed closure is called whenever the mocked function is called. Depending on variant of returned
    /// [MockResult](enum.MockResult.html) the mocked function continues to run or returns immediately.
    /// In case of continuation the function arguments can be modified or replaced.
    ///
    /// The mock closure is saved in a
    /// [thread local static storage](https://doc.rust-lang.org/std/macro.thread_local.html),
    /// so it has effect only in thread, where it was set.
    /// Each Rust test is executed in separate thread, so mocks do not leak between them.
    /// # Safety
    /// It is up to the user to make sure, that the closure is valid long enough to serve all calls to mocked function.
    /// If the mock closure uses any non-static values or references, it will silently become invalid at some point of
    /// host thread lifetime.
    ///
    /// ```
    /// #[mockable]
    /// impl Context {
    /// fn get_string(&self) -> &String {
    ///     // retrieve string
    /// }
    ///
    /// fn append_string(context: &Context, string: &mut String) {
    ///     string.push_str(context.get_string())
    /// }
    ///
    /// #[test]
    /// fn append_string_test() {
    ///     let fetched = "mocked".to_string();
    ///     unsafe {
    ///         Context::get_string.mock_raw(|_| MockResult::Return(&fetched));
    ///     }
    ///     let mut modified = String::new();
    ///
    ///     append_string(&Context::default(), &mut modified);
    ///
    ///     assert_eq!("mocked", modified);
    /// }
    /// ```
    unsafe fn mock_raw<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M);
    /// A safe variant of [mock_raw](#tymethod.mock_raw) for static closures
    ///
    /// The safety is guaranteed by forcing passed closure to be static.
    /// This eliminates the problem of using non-static values, which may not live long enough.
    ///
    /// ```
    /// #[mockable]
    /// fn get_string(is_a: bool) -> &'static str {
    ///     match is_a {
    ///         true => "A",
    ///         false => "B",
    ///     }
    /// }
    ///
    /// fn append_string(string: &mut String) {
    ///     string.push_str(get_string(false))
    /// }
    ///
    /// #[test]
    /// fn append_string_test() {
    ///     get_string.mock_safe(|_| MockResult::Continue((true,)));
    ///     let mut modified = String::new();
    ///
    ///     append_string(&mut modified);
    ///
    ///     assert_eq!("A", modified);
    /// }
    /// ```
    fn mock_safe<M: Fn<T, Output=MockResult<T, O>> + 'static>(&self, mock: M);
    /// **For internal use only**
    ///
    /// Called before every execution of a mockable function. Checks if mock is set and if it is, calls it.
    fn call_mock(&self, input: T) -> MockResult<T, O>;
    /// **For internal use only**
    ///
    /// Returns a unique ID of the function, which is used to set and get its mock.
    unsafe fn get_mock_id(&self) -> TypeId;
}

/// Controls mocked function behavior when returned from [mock closure](trait.Mockable.html)
pub enum MockResult<T, O> {
    /// Function runs normally as if it was called with given arguments.
    /// The arguments are passed inside enum variant as a tuple.
    Continue(T),
    /// Function returns immediately with a given value. The returned value is passed inside enum variant.
    Return(O),
}

thread_local!{
    static MOCK_STORE: RefCell<HashMap<TypeId, Box<Fn<(), Output=()>>>> = RefCell::new(HashMap::new())
}

impl<T, O, F: FnOnce<T, Output=O>> Mockable<T, O> for F {
    unsafe fn mock_raw<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M) {
        let id = self.get_mock_id();
        MOCK_STORE.with(|mock_ref_cell| {
            let fn_box: Box<Fn<T, Output=MockResult<T, O>>> = Box::new(mock);
            let stored: Box<Fn<(), Output=()>> = transmute(fn_box);
            let mock_map = &mut*mock_ref_cell.borrow_mut();
            mock_map.insert(id, stored);
        })
    }

    fn mock_safe<M: Fn<T, Output=MockResult<T, O>> + 'static>(&self, mock: M) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn call_mock(&self, input: T) -> MockResult<T, O> {
        unsafe {
            let id = self.get_mock_id();
            MOCK_STORE.with(|mock_ref_cell| {
                let mock_map = &*mock_ref_cell.borrow();
                match mock_map.get(&id) {
                    Some(stored_box) => {
                        let stored = &**stored_box;
                        let mock: &Fn<T, Output=MockResult<T, O>> = transmute(stored);
                        mock.call(input)
                    },
                    None => MockResult::Continue(input),
                }
            })
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).get_type_id()
    }
}
