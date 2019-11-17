use crate::mocking::MockResult;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::transmute;
use std::rc::Rc;

pub struct MockStore {
    layers: RefCell<Vec<MockLayer>>,
}

impl MockStore {
    pub fn clear(&self) {
        for layer in self.layers.borrow_mut().iter_mut() {
            layer.clear()
        }
    }

    pub fn clear_id(&self, id: TypeId) {
        for layer in self.layers.borrow_mut().iter_mut() {
            layer.clear_id(id)
        }
    }

    /// Layer will be in use as long as MockLayerGuard is alive
    /// MockLayerGuards must always be dropped and always in reverse order of their creation
    pub unsafe fn add_layer(&self, layer: MockLayer) {
        self.layers.borrow_mut().push(layer)
    }

    pub unsafe fn remove_layer(&self) {
        self.layers.borrow_mut().pop();
    }

    pub unsafe fn add_to_thread_layer<I, O>(
        &self,
        id: TypeId,
        mock: Box<dyn FnMut<I, Output = MockResult<I, O>> + 'static>,
    ) {
        self.layers
            .borrow_mut()
            .first_mut()
            .expect("Thread mock level missing")
            .add(id, mock);
    }

    pub unsafe fn call<I, O>(&self, id: TypeId, mut input: I) -> MockResult<I, O> {
        // Do not hold RefCell borrow while calling mock, it can try to modify mocks
        let layer_count = self.layers.borrow().len();
        for layer_idx in (0..layer_count).rev() {
            let mock_opt = self
                .layers
                .borrow()
                .get(layer_idx)
                .expect("Mock layer removed while iterating")
                .get(id);
            if let Some(mock) = mock_opt {
                match mock.call(input) {
                    MockLayerResult::Handled(result) => return result,
                    MockLayerResult::Unhandled(new_input) => input = new_input,
                }
            }
        }
        MockResult::Continue(input)
    }
}

//TODO tests
// clear
// clear id
// add and remove layer
//   inside mock closure

impl Default for MockStore {
    fn default() -> Self {
        MockStore {
            layers: RefCell::new(vec![MockLayer::default()]),
        }
    }
}

#[derive(Default)]
pub struct MockLayer {
    mocks: HashMap<TypeId, ErasedStoredMock>,
}

impl MockLayer {
    fn clear(&mut self) {
        self.mocks.clear()
    }

    fn clear_id(&mut self, id: TypeId) {
        self.mocks.remove(&id);
    }

    pub unsafe fn add<I, O>(
        &mut self,
        id: TypeId,
        mock: Box<dyn FnMut<I, Output = MockResult<I, O>> + 'static>,
    ) {
        let stored = StoredMock::new(mock).erase();
        self.mocks.insert(id, stored);
    }

    unsafe fn get(&self, id: TypeId) -> Option<ErasedStoredMock> {
        self.mocks.get(&id).cloned()
    }
}

pub enum MockLayerResult<I, O> {
    Handled(MockResult<I, O>),
    Unhandled(I),
}

#[derive(Clone)]
struct ErasedStoredMock {
    mock: StoredMock<(), ()>,
}

impl ErasedStoredMock {
    unsafe fn call<I, O>(self, input: I) -> MockLayerResult<I, O> {
        let unerased: StoredMock<I, O> = transmute(self.mock);
        unerased.call(input)
    }
}

/// Guarantees that while mock is running it's not overwritten, destroyed, or called again
#[derive(Clone)]
struct StoredMock<I, O> {
    mock: Rc<RefCell<Box<dyn FnMut<I, Output = MockResult<I, O>>>>>,
}

impl<I, O> StoredMock<I, O> {
    fn new(mock: Box<dyn FnMut<I, Output = MockResult<I, O>> + 'static>) -> Self {
        StoredMock {
            mock: Rc::new(RefCell::new(mock)),
        }
    }

    fn call(&self, input: I) -> MockLayerResult<I, O> {
        match self.mock.try_borrow_mut() {
            Ok(mut mock) => MockLayerResult::Handled(mock.call_mut(input)),
            Err(_) => MockLayerResult::Unhandled(input),
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
