# A global, thread-safe memory pool

[![Build Status](https://travis-ci.org/koute/memory-pool.svg)](https://travis-ci.org/koute/memory-pool)

[API documentation](https://koute.github.io/memory-pool/memory_pool/index.html)

A Rust library providing a global, thread-safe memory pool,
ideal for when you need a temporary scratch buffer but you
don't want to be constantly allocating memory.

```rust
memory_pool::borrow( |aux: &mut String| {
    aux.push_str( "Do you like cupcakes?" );
});
```

```rust
memory_pool::borrow( |vec: &mut Vec< u32 >| {
    vec.push( 1 );
    vec.push( 2 );
    vec.push( 3 );
});
```
You can also manually acquire and release memory:

```rust
let mut buffer: String = memory_pool::acquire();
buffer.push_str( "I like cupcakes!" );
memory_pool::release( buffer );
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
memory-pool = "0.1.0"
```

Then add this to your crate root:

```rust
extern crate memory_pool;
```
