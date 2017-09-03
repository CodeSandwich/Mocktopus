Mocking framework for Rust (currently only nightly). See documentation for more.

```
#[mockable]
mod hello_world {
    pub fn world() -> &'static str {
        "world"
    }

    pub fn hello_world() -> String {
        format!("Hello {}!", world())
    }
}

#[test]
fn mock_test() {
    hello_world::world.mock_safe(|| MockResult::Return("mocking"));

    assert_eq!("Hello mocking!", hello_world::hello_world());
}
```
