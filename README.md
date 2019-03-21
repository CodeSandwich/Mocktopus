[![Crates.io](https://img.shields.io/crates/d/mocktopus.svg)](https://crates.io/crates/mocktopus)
[![Docs.rs](https://docs.rs/mocktopus/badge.svg)](https://docs.rs/crate/mocktopus)
[![Build Status](https://travis-ci.org/CodeSandwich/Mocktopus.svg?branch=master)](https://travis-ci.org/CodeSandwich/Mocktopus)

<p align="center">
  <img src="https://raw.githubusercontent.com/CodeSandwich/mocktopus/master/logo.png" alt="logo"/>
</p>

Mocking framework for Rust (currently only nightly). See [documentation](https://docs.rs/mocktopus/) for more.

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
