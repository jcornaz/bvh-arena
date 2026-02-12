# bvh-arena

[![License](https://img.shields.io/crates/l/bvh-arena)](#Unlicense)
[![Crates.io](https://img.shields.io/crates/v/bvh-arena)](https://crates.io/crates/bvh-arena)
![rustc](https://img.shields.io/badge/rustc-1.56+-blue?logo=rust)
[![Docs](https://docs.rs/bvh-arena/badge.svg)](https://docs.rs/bvh-arena)
![Maintenance](https://img.shields.io/maintenance/passively/2026)

A bounding-volume hierarchy for in-game broad-phase *collision detection* in rust


## Features

* Fast insertion and removal
* Fast iteration over overlaping pairs
* Efficient reuse of memory when volumes are removed (backed by a generational-arena storage)


## Non-goals

This library does *not* attempt to be optimal for:

* ray-tracing (though, basic ray-cast may eventually be supported)
* static scenes

## How usage looks like

```rust
use bvh_arena::{Bvh, volumes::Aabb};
use glam::Vec2; // <-- You may use any math library, or even none.

// Create a bounding volume hierarchy
let mut bvh: Bvh<u8, Aabb<2>> = Bvh::default();

// Insert a bounding volume
let id = bvh.insert(1, Aabb::from_min_max(Vec2::ZERO, Vec2::new(1.0, 1.0)));

// Remove a bounding volume
bvh.remove(id);

// Iteration over overlaping pairs
bvh.for_each_overlaping_pair(|a, b| println!("{a} overlaps {b}"));
```

## `no_std` support

This crate is `no_std` compatible but requires a [global allocator](https://doc.rust-lang.org/stable/core/alloc/trait.GlobalAlloc.html).


## Installation

Add to `Cargo.toml`:

```toml
bvh-arena = "1"
```

## MSRV

The minimum supported rust version is currently: `1.56`

**It *may* be increased to a newer stable version in a minor release.** (but only if needed)

It *will* be increased to the latest stable version in a major release. (even if not needed)


## Motivation

In the context of building my own collision detection logic, I wanted to use a bounding-volume-hierarchy (BVH)
for the broad-phase.

But it seamed that most of the existing BVH implementations are biased toward the ray-tracing use-case. 
And that means, they provide different API, algorithms and trade-offs than what I am looking for.


## Unlicense

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
