# error-chain for dumb IDEs

This library contains a massive hack to make using certain IDEs cope less poorly
with `error-chain` heavy code.

## What?

[error-chain](https://crates.io/crates/error_chain) is *the* error handling
library for Rust. If you're not using it, you're doing it wrong.

However, it makes heavy use of Rust's macro system, which is not well understood
by some IDEs.

This library provides a set of stubs which makes the IDEs slightly less dumb,
and hence your experience slightly less awful.

Without it, completion fails for code like:
```rust
let a = foo().chain_err(|| "foo failed :(")?;
a. // <-- here.
```

Afterwards, it succeeds!

## How?

 1. Add this crate as a dependency. `dev-dependencies` is fine:
    ```toml
    [dev-dependencies.error-chain-for-dumb-ides]
    git = "https://github.com/FauxFaux/error-chain-for-dumb-ides"
    ```

    As crates.io packages' dependencies cannot be from git repositories, you
    can use the crates.io published version. I do not plan to maintain support
    for old versions, even in git HEAD. To do this, instead use this fragment:
    ```toml
    [dev-dependencies.error-chain-for-dumb-ides]
    version = "0"
    ```

 2. Import the crate. You can do this behind a nonsense `cfg`, as this is just
    totally ignored by the IDE, and the code is run anyway:
    ```rust
    #[cfg(intellij_type_hinting)]
    extern crate error_chain_for_dumb_ides;
    ```

 3. Inside your `mod errors {}`, import the crate's contents:
    ```rust
    #[cfg(intellij_type_hinting)]
    pub use error_chain_for_dumb_ides::stubs::*;
    ```

That's it! Now, suddenly, completion starts to work.

Here's [an example of doing this in a real project](https://github.com/FauxFaux/fapt/commit/be72bca76643d89ed6049f62bb7bdcf5f03f2e5a).

## License

[`CC0`](https://creativecommons.org/share-your-work/public-domain/cc0/).

Think "public domain", no rights reserved. Given this library is trivial,
and never part of your production codebase anyway, any license is
practically irrelevant anyway.
