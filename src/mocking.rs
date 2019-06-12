use crate::mock_store::MockStore;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
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
    /// fn get_string(context: &Context) -> &String {
    ///     context.get_string()
    /// }
    ///
    /// #[test]
    /// fn get_string_test() {
    ///     let mocked = "mocked".to_string();
    ///     unsafe {
    ///         get_string.mock_raw(|_| MockResult::Return(&mocked));
    ///     }
    ///
    ///     assert_eq!("mocked", get_string(&Context::default()));
    /// }
    /// ```
    unsafe fn mock_raw<M: FnMut<T, Output=MockResult<T, O>>>(&self, mock: M);

    /// A safe variant of [mock_raw](#tymethod.mock_raw) for static closures
    ///
    /// The safety is guaranteed by forcing passed closure to be static.
    /// This eliminates the problem of using non-static values, which may not live long enough.
    ///
    /// ```
    /// #[mockable]
    /// fn get_string() -> String {
    ///     "not mocked".to_string()
    /// }
    ///
    /// #[test]
    /// fn get_string_test() {
    ///     get_string.mock_safe(|| MockResult::Return("mocked".to_string()));
    ///
    ///     assert_eq!("mocked", get_string());
    /// }
    /// ```
    fn mock_safe<M: FnMut<T, Output=MockResult<T, O>> + 'static>(&self, mock: M);

    /// Stop mocking this function.
    ///
    /// All future invocations will be forwarded to the real implementation.
    fn clear_mock(&self);

    #[doc(hidden)]
    /// Called before every execution of a mockable function. Checks if mock is set and if it is, calls it.
    fn call_mock(&self, input: T) -> MockResult<T, O>;

    #[doc(hidden)]
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
    static MOCK_STORE: MockStore = MockStore::default()
}

/// Clear all mocks in the ThreadLocal; only necessary if tests share threads
pub fn clear_mocks() {
    MOCK_STORE.with(|mock_store| mock_store.clear())
}

struct ScopedMock<'a> {
    phantom: PhantomData<&'a ()>,
    id: TypeId,
}

impl<'a> ScopedMock<'a> {
    unsafe fn new<T, O, M: Mockable<T, O> + 'a, F: FnMut<T, Output=MockResult<T, O>>>(
        mockable: &M,
        mock: F,
    ) -> Self {
        mockable.mock_raw(mock);
        ScopedMock {
            phantom: PhantomData,
            id: mockable.get_mock_id(),
        }
    }
}

impl<'a> Drop for ScopedMock<'a> {
    fn drop(&mut self) {
        clear_id(self.id);
    }
}

fn clear_id(id: TypeId) {
    MOCK_STORE.with(|mock_store| mock_store.clear_id(id))
}

impl<T, O, F: FnOnce<T, Output=O>> Mockable<T, O> for F {
    unsafe fn mock_raw<M: FnMut<T, Output=MockResult<T, O>>>(&self, mock: M) {
        let id = self.get_mock_id();
        let boxed = Box::new(mock) as Box::<FnMut<_, Output = _>>;
        let static_boxed: Box<FnMut<T, Output = MockResult<T, O>> + 'static> = transmute(boxed);
        MOCK_STORE.with(|mock_store| mock_store.add(id, static_boxed))
    }

    fn mock_safe<M: FnMut<T, Output=MockResult<T, O>> + 'static>(&self, mock: M) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    fn clear_mock(&self) {
        let id = unsafe { self.get_mock_id() };
        clear_id(id);
    }

    fn call_mock(&self, input: T) -> MockResult<T, O> {
        unsafe {
            let id = self.get_mock_id();
            let mock_opt = MOCK_STORE.with(|mock_store| mock_store.get(id));
            match mock_opt {
                Some(mock) => mock.call(input),
                None => MockResult::Continue(input),
            }
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).type_id()
    }
}

/// `MockContext` allows for safe capture of local variables.
///
/// It does this by forcing only mocking the actual function while in the body
/// of [`run`].
///
/// # Examples
///
/// Simple function replacement:
///
/// ```
/// use mocktopus::macros::mockable;
/// use mocktopus::mocking::{MockContext, MockResult};
///
/// #[mockable]
/// fn f() -> i32 {
///     0
/// }
///
/// MockContext::new()
///     .mock_safe(f, || MockResult::Return(1))
///     .run(|| {
///         assert_eq!(f(), 1);
///     });
/// ```
///
/// Using local variables:
///
/// ```
/// use mocktopus::macros::mockable;
/// use mocktopus::mocking::{MockContext, MockResult};
///
/// #[mockable]
/// fn as_str(s: &String) -> &str {
///     &s
/// }
///
/// let mut count = 0;
/// MockContext::new()
///     .mock_safe(as_str, |s| { count += 1; MockResult::Return(&s) })
///     .run(|| {
///         assert_eq!(as_str(&"abc".to_string()), "abc");
///     });
/// assert_eq!(count, 1);
/// ```
#[derive(Default)]
pub struct MockContext<'a> {
    planned_mocks: HashMap<TypeId, Box<FnOnce() -> ScopedMock<'a> + 'a>>,
}

impl<'a> MockContext<'a> {
    /// Create a new MockContext object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set up a function to be mocked.
    ///
    /// This function doesn't actually mock the function.  It registers it as a
    /// function that will be mocked when [`run`](#method.run) is called.
    pub fn mock_safe<
        Args,
        Output,
        M: Mockable<Args, Output> + 'a,
        F: FnMut<Args, Output = MockResult<Args, Output>> + 'a,
    >(
        mut self,
        mock: M,
        body: F,
    ) -> Self {
        self.planned_mocks.insert(
            unsafe { mock.get_mock_id() },
            Box::new(move || unsafe { ScopedMock::new(&mock, body) }),
        );
        self
    }

    /// Run the function while mocking all the functions.
    ///
    /// This function will mock all functions registered for mocking, run the
    /// function passed in, then deregister those functions.  It does this in a
    /// panic-safe way.  Note that functions are only mocked in the current
    /// thread and other threads may invoke the real implementations.
    ///
    /// Register a function for mocking with [`mock_safe`](#method.mock_safe).
    pub fn run<T, F: FnOnce() -> T>(self, f: F) -> T {
        let _scoped_mocks = self
            .planned_mocks
            .into_iter()
            .map(|entry| entry.1())
            .collect::<Vec<_>>();
        f()
    }
}
