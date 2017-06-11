#![feature(unboxed_closures, get_type_id)]

mod mock_trait;
mod mock_store;

pub use mock_trait::{MockResult, MockTrait};
use mock_store::MOCK_STORE;

