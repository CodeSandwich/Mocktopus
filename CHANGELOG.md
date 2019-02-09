## 0.6.0 - 2019-09-02
- Fix for rustc 1.31.0-nightly (ad6e5c003 2018-09-28) and newer (thank you @asomers and @rye!)
## 0.5.0 - 2018-07-17
- Fix for rustc 1.29.0-nightly (1ecf6929d 2018-07-16) and newer
## 0.4.1 - 2018-07-15
- Fix tests for rustc 1.29.0-nightly (254f8796b 2018-07-13) and newer
## 0.4.0 - 2018-05-23
- Port to rustc 1.28.0-nightly (71e87be38 2018-05-22) and newer
## 0.3.8 - 2018-05-13
- Add mocking with mutable closure
## 0.3.7 - 2018-05-13
- Fix uninitialized memory access when fn arg has drop and mock returns continue
- Fix specific case of injecting trait impl
## 0.3.6 - 2018-05-11
- Fix injecting of fns with unused generics
- Prevent injecting of Drop::drop
- Upgrade minimum Syn version to 0.13.7
## 0.3.5 - 2018-05-08
- Enable mocking of code in a no_std crate when std lib is available
## 0.3.4 - 2018-05-06
- Make annotating items unparsable for Syn raise warning and continue without making it mockable instead of failing whole compilation
## 0.3.3 - 2018-05-04
- Prevent macros from injecting unsafe functions, which would make them uncompilable
## 0.3.2 - 2018-05-04
- Fix segfault when fn arg has drop and mock is returned
- Fix returning mocks of generic types, which was broken in version 0.3.1
## 0.3.1 - 2018-05-03
- Fix injecting trait impls when return type has longer lifetime than required by trait
## 0.3.0 - 2018-04-30
- Upgrade syn to 0.13.4 and make it use nightly features of proc-macro2. This lets compiler correctly point at errors in injected functions, which closes issue #5.
## 0.2.1 - 2018-04-29
- Add support for proc_macros feature being split up into proc_macros and proc_macro_mod in Rust nightly from 2018-04-27
## 0.2.0 - 2018-01-23
- Migrate Mocktopus macros from Syn 0.11 to 0.12
## 0.1.2 - 2018-01-23
- Panicking inside mock code does not cause dropping of uninitialized memory
- Items with two `#[mockable]` attributes on them are not injected twice
extern crate mocktopus can be now aliased
## 0.1.1 - 2017-09-05
- Remove Cargo.lock, add logo in macro docs and bump version to 0.1.1
## 0.1.0 - 2017-09-05
- Remove Cargo.lock, add logo in macro docs and bump version to 0.1.1
