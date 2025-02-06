# Infix macro

[![Build Icon]][Build Status]&emsp;[![Docs Icon]][Docs]&emsp;[![Version Icon]][Crate]&emsp;[![License Icon]][LICENSE]

[Build Icon]: https://img.shields.io/github/actions/workflow/status/1Git2Clone/infix-macro/build.yml?branch=main
[Build Status]: https://github.com/1git2clone/infix-macro/actions?query=branch%3Amain
[Docs Icon]: https://docs.rs/infix-fn/badge.svg
[Docs]: https://docs.rs/infix-fn
[Version Icon]: https://img.shields.io/crates/v/infix-fn.svg
[Crate]: https://crates.io/crates/infix-fn
[License Icon]: https://img.shields.io/badge/license-MIT-blue.svg
[LICENSE]: LICENSE

A very simple procedural macro used to make infix function calls similarly to
Haskell's syntax (`#` instead of `\``).

## Table of contents

- [Infix macro](#infix-macro)
  - [Example](#example)

## Example

```rust
use infix_macro::infix;

// Function definition:
fn add(lhs: i32, rhs: i32) -> i32 {
  lhs + rhs
}

// Macro usage:
let (lhs, rhs) = (6, 9);
assert_eq!(lhs + rhs, infix!(lhs # add # rhs));
```
