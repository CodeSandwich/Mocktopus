[![Crates.io](https://img.shields.io/crates/d/rustc-serialize.svg)](https://crates.io/crates/mocktopus)
[![Docs.rs](https://docs.rs/mocktopus/badge.svg)](https://docs.rs/mocktopus)
[![Build Status](https://travis-ci.org/CodeSandwich/Mocktopus.svg?branch=master)](https://travis-ci.org/CodeSandwich/Mocktopus)

Mocking framework for Rust. See [documentation](https://docs.rs/mocktopus) for more information.

```rust
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
