Mocking framework for Rust

```
#[mockable]
fn world() -> &'static str {
    "world"
}

fn hello_world() -> String {
    format!("Hello {}!", world())
}

#[test]
fn mock_test() {
    world.mock_safe(|| MockResult::Return("mocking"));

    assert_eq!("Hello mocking!", hello_world());
}
```
