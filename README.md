core-math
=========
[![Crates.io](https://img.shields.io/crates/v/core-math.svg)](https://crates.io/crates/core-math)
[![Documentation](https://docs.rs/core-math/badge.svg)](https://docs.rs/core-math)
[![Build status](https://github.com/jdh8/core-math/actions/workflows/rust.yml/badge.svg)](https://github.com/jdh8/core-math)

Rusty API for [CORE-MATH](https://core-math.gitlabpages.inria.fr/)

CORE-MATH is a correctly rounded mathematical library in C.  Correct rounding is
the theoretical accuracy and beats most old C libraries such as glibc.
Meanwhile, its speed is competitive with the most popular C libraries, even
faster most of the time.

This crate provides a Rusty API to the CORE-MATH library via
[`core-math-sys`](https://crates.io/crates/core-math-sys), the system crate.

Cargo features
--------------

The default build exposes the `f32` and `f64` functions and works on stable
Rust (1.85+).  Functions on more exotic types are opt-in because their Rust
primitives are unstable and their C types need a recent compiler:

- `f16` — binary16 functions (`*f16`, 43 functions).  Requires nightly Rust
  for the unstable [`f16`](https://doc.rust-lang.org/std/primitive.f16.html)
  primitive, and GCC&nbsp;12+ or Clang&nbsp;17+ (the C sources need `_Float16`
  and `__builtin_roundeven`).
- `f128` — binary128 functions (`*q`, 13 functions).  Requires nightly Rust
  for the unstable [`f128`](https://doc.rust-lang.org/std/primitive.f128.html)
  primitive, and Clang&nbsp;15+ or GCC&nbsp;14+ (the C sources use
  `__builtin_addcl`).
