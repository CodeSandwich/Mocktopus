#![feature(fn_traits, get_type_id, unboxed_closures)]

mod mock_trait;
mod mock_store;

pub use mock_trait::{MockResult, MockTrait};
use mock_store::MOCK_STORE;

