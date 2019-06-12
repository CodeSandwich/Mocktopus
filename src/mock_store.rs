use crate::mocking::MockResult;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::transmute;
use std::rc::Rc;

#[derive(Default)]
pub struct MockStore {
    mocks: RefCell<HashMap<TypeId, ErasedStoredMock>>,
}

impl MockStore {
    pub fn clear(&self) {
        self.mocks.borrow_mut().clear()
    }

    pub fn clear_id(&self, id: TypeId) {
        self.mocks.borrow_mut().remove(&id);
    }

    pub unsafe fn add<I, O>(&self, id: TypeId, mock: Box<FnMut<I, Output=MockResult<I, O>> + 'static>) {
        let stored = StoredMock::new(mock).erase();
        self.mocks.borrow_mut().insert(id, stored);
    }

    pub unsafe fn get<I, O>(&self, id: TypeId) -> Option<StoredMock<I, O>> {
        self.mocks.borrow().get(&id).cloned().map(|mock| mock.unerase())
    }
}

/// Guarantees that while mock is running it's not overwritten, destroyed, or called again
#[derive(Clone)]
pub struct StoredMock<I, O> {
    mock: Rc<RefCell<Box<FnMut<I, Output=MockResult<I, O>>>>>
}

impl<I, O> StoredMock<I, O> {
    fn new(mock: Box<FnMut<I, Output=MockResult<I, O>> + 'static>) -> Self {
        StoredMock {
            mock: Rc::new(RefCell::new(mock))
        }
    }

    pub fn call(&self, input: I) -> MockResult<I, O> {
        match self.mock.try_borrow_mut() {
            Ok(mut mock) => mock.call_mut(input),
            Err(_) => MockResult::Continue(input),
        }
    }

    fn erase(self) -> ErasedStoredMock {
        unsafe {
            ErasedStoredMock {
                mock: transmute(self),
            }
        }
    }
}

#[derive(Clone)]
struct ErasedStoredMock {
    mock: StoredMock<(), ()>,
}

impl ErasedStoredMock {
    unsafe fn unerase<I, O>(self) -> StoredMock<I, O> {
        transmute(self.mock)
    }
}
