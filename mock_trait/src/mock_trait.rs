use std::any::{Any, TypeId};
use ::MOCK_STORE;
use std::collections::HashMap;

pub trait MockTrait<T, O> {
    fn set_mock<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M);
    fn call_mock(&self, input: T) -> MockResult<T, O>;
    fn get_mock_id(&self) -> TypeId;
}

pub enum MockResult<T, O> {
    Continue(T),
    Return(O),
}

impl<T, O, F: FnOnce<T, Output=O>> MockTrait<T, O> for F {
    fn set_mock<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M) {
        let id = self.get_mock_id();
    }

    fn call_mock(&self, input: T) -> MockResult<T, O> {
        let id = self.get_mock_id();
        MOCK_STORE.with(|mock_ref_cell| {mock_ref_cell.borrow();});
        let mock = MOCK_STORE.with(|mock_ref_cell| {
            let mock_map: &HashMap<TypeId, String> = &*mock_ref_cell.try_borrow()
                .expect("Mockable function called while set_mock running in same thread");
            let mock_ref: &String = mock_map.get(&id).unwrap();
            mock_ref.clone()
        });

        MockResult::Continue(input)

    }

    fn get_mock_id(&self) -> TypeId {
        (||()).get_type_id()
    }
}
