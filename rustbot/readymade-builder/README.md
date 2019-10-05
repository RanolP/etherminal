# readymade-builder

This crate does type-juggling to provide **reassign-safe**, and well-typed builders.
Public fields are settable by default, private fields are unsettable by default.

```rust
#[derive(ReadymadeBuilder)]
struct User {
  first_name: String,
  last_name: String,
  age: Option<i32>,
}
```

will allow us to use those code:

```rust
let user = User::builder()
  // by default, Into<T> accepted.
  .first_name("Paul")
  .last_name("Randolph")
  .age(30)
  // you can build because you filled first_name and last_name
  .build();
```
