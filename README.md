[![Build Status](https://travis-ci.org/sdleffler/empty-option-rs.svg?branch=master)](https://travis-ci.org/sdleffler/empty-option-rs)
[![Docs Status](https://docs.rs/empty-option/badge.svg)](https://docs.rs/empty-option)
[![On crates.io](https://img.shields.io/crates/v/empty-option.svg)](https://crates.io/crates/empty-option)

# empty-option: guards for safely taking and dealing with values from mutable references to `Option<T>`

This crate provides convenient wrappers for dealing with `&mut Option<T>`. There are two main types, `OptionGuard` and `OptionGuardMut`:

## `OptionGuard`

Using `EmptyOptionExt::steal` on an `&mut Option<T>` produces the `T` from the option as well as an `OptionGuard`. If `OptionGuard::restore` is not called before the `OptionGuard` is dropped, then a panic will occur.

### Examples

Calling `guard.restore()` puts the stolen value back into the original option:

```rust
use empty_option::EmptyOptionExt;

// A mutable option, from which we shall steal a value!
let mut thing = Some(5);

// Scope so that when we do `guard.restore()`, the mutable borrow on `thing` will end.
{
    // Steal the value - we now have the guard and also a concrete `T` from our `Option<T>`.
    let (guard, five) = thing.steal();

    assert_eq!(five, 5);

    // Move the value back into `thing` - we're done.
    guard.restore(6);
}

// The value is returned by `guard.restore()`.
assert_eq!(thing, Some(6));
```

But, if the guard is dropped instead, a runtime panic results.

```rust,should_panic
use empty_option::EmptyOptionExt;

let mut thing = Some(5);

let (_, _) = thing.steal();

// Never return the value!
```

## `OptionGuardMut`

Using `EmptyOptionExt::steal_mut` on an `&mut Option<T>` produces an `OptionGuardMut`, which dereferences to a `T`. To get the inner value out, `OptionGuardMut::into_inner` can be called. On `Drop`, if the `OptionGuardMut` is not consumed with `OptionGuardMut::into_inner`, the value in the `OptionGuardMut` will be returned to the `Option` that it was borrowed from.

### Examples

Take a value from an option, which is automatically returned:

```rust
use empty_option::EmptyOptionExt;

let mut thing = Some(5);

{
    let mut stolen = thing.steal_mut();

    assert_eq!(*stolen, 5);

    *stolen = 6;
}

assert_eq!(thing, Some(6));
```

If the guard is consumed, the value is never returned.

```rust
use empty_option::EmptyOptionExt;

let mut thing = Some(5);

{
    // Keep the thing!
    let stolen = thing.steal_mut().into_inner();

    assert_eq!(stolen, 5);
}

assert_eq!(thing, None);
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
