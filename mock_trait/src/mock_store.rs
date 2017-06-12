use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local!{
    pub static MOCK_STORE: RefCell<HashMap<TypeId, Box<Fn<(), Output=()>>>> = RefCell::new(HashMap::new())
}
