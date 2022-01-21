# bvh-arena

A bounding-volume hierarchy for in-game broad-phase collision detection

## Goals

* Provide a viable bounding-volume hierarchy for broad-phase collision-detection
* Being usable in-game where the scene can be very dynamic
  * Insertion algorithm
  * Fast insertion and removal
  * Efficient reuse of memory when volumes are removed (backed by a generational-arena storage)


## Non-goals

* Being optimized for ray-tracing (though, ray-cast will eventually be supported)


## Motivation

In the context of building my own collision detection library for rust, I wanted to use a bounding-volume-hierarchy (BVH)
for the broad-phase.

But it seamed that most of the existing BVH implementations are biased oriented toward the ray-tracing use-case. 
And they provide different API, algorithms and trades-off than what I am looking for.


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
