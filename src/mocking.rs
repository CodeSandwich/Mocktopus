use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::transmute;
use std::rc::Rc;

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

    /// A variant of [mock_raw](#tymethod.mock_raw) that can use local variables.
    ///
    /// For a safe variant, see [`MockContext`].
    ///
    /// # Safety
    ///
    /// `mock_scoped` returns a [`ScopedMock`](struct.ScopedMock.html) object.
    /// If this object's `drop` impl is not called (for example because the
    /// `ScopedMock` object is `std::mem::forgot`) and
    /// [`clear_mock`](#tymethod.clear_mock) is not called, the next call to the
    /// mock function will invoke undefined behavior.
    ///
    /// If you do not do anything crazy, this is safe.
    ///
    /// # Examples
    ///
    /// ```
    /// #[mockable]
    /// fn print_first(x: &[i32]) {
    ///     print!("{:?}", &x[0]);
    /// }
    ///
    /// let mut called = false;
    /// let _scope = unsafe { get_string.mock_scoped(|| {
    ///     called = true;
    ///     MockResult::Return(&x[0])
    /// }) };
    /// assert!(called);
    /// ```
    unsafe fn mock_scoped<'a, M: FnMut<T, Output=MockResult<T, O>> + 'a>(&self, mock: M) -> ScopedMock<'a>;

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
    static MOCK_STORE: RefCell<HashMap<TypeId, Rc<RefCell<Box<FnMut<(), Output=()>>>>>> = RefCell::new(HashMap::new())
}

/// Clear all mocks in the ThreadLocal; only necessary if tests share threads
pub fn clear_mocks() {
    MOCK_STORE.with(|mock_ref_cell| {
        mock_ref_cell.borrow_mut().clear();
    });
}

pub struct ScopedMock<'a> {
    phantom: PhantomData<&'a ()>,
    id: TypeId,
}

impl<'a> Drop for ScopedMock<'a> {
    fn drop(&mut self) {
        clear_id(self.id);
    }
}

fn clear_id(id: TypeId) {
    MOCK_STORE.with(|mock_ref_cell| {
        mock_ref_cell.borrow_mut().remove(&id);
    });
}

impl<T, O, F: FnOnce<T, Output=O>> Mockable<T, O> for F {
    unsafe fn mock_raw<M: FnMut<T, Output=MockResult<T, O>>>(&self, mock: M) {
        let id = self.get_mock_id();
        MOCK_STORE.with(|mock_ref_cell| {
            let real = Rc::new(RefCell::new(Box::new(mock) as Box<FnMut<_, Output=_>>));
            let stored = transmute(real);
            mock_ref_cell.borrow_mut()
                .insert(id, stored);
        })
    }

    fn mock_safe<M: FnMut<T, Output=MockResult<T, O>> + 'static>(&self, mock: M) {
        unsafe {
            self.mock_raw(mock)
        }
    }

    unsafe fn mock_scoped<'a, M: FnMut<T, Output=MockResult<T, O>> + 'a>(&self, mock: M) -> ScopedMock<'a> {
        self.mock_raw(mock);
        ScopedMock {
            phantom: PhantomData,
            id: self.get_mock_id(),
        }
    }

    fn clear_mock(&self) {
        let id = unsafe { self.get_mock_id() };
        clear_id(id);
    }

    fn call_mock(&self, input: T) -> MockResult<T, O> {
        unsafe {
            let id = self.get_mock_id();
            let rc_opt = MOCK_STORE.with(|mock_ref_cell|
                mock_ref_cell.borrow()
                    .get(&id)
                    .cloned()
            );
            let stored_opt = rc_opt.as_ref()
                .and_then(|rc| rc.try_borrow_mut().ok());
            match stored_opt {
                Some(mut stored) => {
                    let real: &mut Box<FnMut<_, Output=_>> = transmute(&mut*stored);
                    real.call_mut(input)
                }
                None => MockResult::Continue(input),
            }
        }
    }

    unsafe fn get_mock_id(&self) -> TypeId {
        (||()).type_id()
    }
}

pub struct MockContext<'a> {
    planned_mocks: Vec<Box<FnOnce() -> ScopedMock<'a> + 'a>>,
}

impl<'a> MockContext<'a> {
    pub fn new() -> Self {
        MockContext {
            planned_mocks: Vec::new(),
        }
    }

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
        self.planned_mocks
            .push(Box::new(move || unsafe { mock.mock_scoped(body) }));
        self
    }

    pub fn run<T, F: FnOnce() -> T>(self, f: F) -> T {
        let _scoped_mocks = self
            .planned_mocks
            .into_iter()
            .map(|f| f())
            .collect::<Vec<_>>();
        f()
    }
}
