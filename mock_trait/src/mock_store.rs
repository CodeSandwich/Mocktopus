use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local!{
    pub static MOCK_STORE: RefCell<HashMap<TypeId, String>> = RefCell::new(HashMap::new())
}
