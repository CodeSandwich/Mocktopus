use std::string::ToString;

pub fn single_generic_fn<T: ToString>(t: T) -> String {
    t.to_string()
}
