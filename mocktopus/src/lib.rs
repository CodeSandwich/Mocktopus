#![feature(fn_traits, get_type_id, unboxed_closures)]

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::transmute;

pub trait Mockable<T, O> {
    fn set_mock<M: Fn<T, Output=MockResult<T, O>> + 'static>(&self, mock: M);
    unsafe fn set_mock_unsafe<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M);
    fn call_mock(&self, input: T) -> MockResult<T, O>;
    unsafe fn get_mock_id(&self) -> TypeId;
}

pub enum MockResult<T, O> {
    Continue(T),
    Return(O),
}

thread_local!{
    static MOCK_STORE: RefCell<HashMap<TypeId, Box<Fn<(), Output=()>>>> = RefCell::new(HashMap::new())
}

impl<T, O, F: FnOnce<T, Output=O>> Mockable<T, O> for F {
    fn set_mock<M: Fn<T, Output=MockResult<T, O>> + 'static>(&self, mock: M) {
        unsafe {
            self.set_mock_unsafe(mock);
        }
    }

    unsafe fn set_mock_unsafe<M: Fn<T, Output=MockResult<T, O>>>(&self, mock: M) {
        let id = self.get_mock_id();
        MOCK_STORE.with(|mock_ref_cell| {
            let fn_box: Box<Fn<T, Output=MockResult<T, O>>> = Box::new(mock);
            let stored: Box<Fn<(), Output=()>> = transmute(fn_box);
            let mock_map = &mut*mock_ref_cell.borrow_mut();
            mock_map.insert(id, stored);
        })
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
