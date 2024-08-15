core-math
=========
[![Build status](https://github.com/jdh8/core-math/actions/workflows/rust.yml/badge.svg)](https://github.com/jdh8/core-math)
[![Crates.io](https://img.shields.io/crates/v/core-math.svg)](https://crates.io/crates/core-math)
[![Documentation](https://docs.rs/core-math/badge.svg)](https://docs.rs/core-math)

Rusty API for [CORE-MATH](https://core-math.gitlabpages.inria.fr/)

CORE-MATH is a correctly rounded mathematical library in C.  Correct rounding is
the theoretical accuracy and beats most old C libraries such as glibc.
Meanwhile, its speed is competitive with the most popular C libraries, even
faster most of the time.

This crate provides a Rusty API to the CORE-MATH library via
[`core-math-sys`](https://crates.io/crates/core-math-sys), the system crate.