# bvh-arena

A bounding-volume hierarchy for in-game broad-phase collision detection

## Features

* Fast insertion and removal
* Fast iteration over overlaping pairs
* Efficient reuse of memory when volumes are removed (backed by a generational-arena storage)


## Non-goals

This library does *not* attempt to be optimized for ray-tracing (though, ray-cast may eventually be supported)


## `no_std` support

This crate is `no_std` compatible but requires a [global allocator](https://doc.rust-lang.org/stable/core/alloc/trait.GlobalAlloc.html).


## Motivation

In the context of building my own collision detection logic, I wanted to use a bounding-volume-hierarchy (BVH)
for the broad-phase.

But it seamed that most of the existing BVH implementations are biased toward the ray-tracing use-case. 
And that means, they provide different API, algorithms and trade-offs than what I am looking for.


## License

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
