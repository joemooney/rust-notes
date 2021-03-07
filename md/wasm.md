# WASM

## Types
* i32/i64
* f32/f64
* funcref

Other types we pass reference and read it from memory

## Limitations
* No GPU access
* No garbase collection (planned)
* No exception handling

## Planned
* Garbase collection
* Host Bindings will allow sharing for JS/DOM objects
* Threads will be supplied via Web Workers
* Exception handling (long term)
* Reference Types - anyref type, serialization

Most applications open a file by some reference, which is usually a file path on the filesystem. Any application that has that reference (file path) can interact with that file, provided the Ambient Authority allows.

Normally, if user runs a binary, the binary has the same file access permissions as the user. This is called "Ambient Authority".
WASI instead uses a "Capability Model". A binary must be explicitly granted permissions.



