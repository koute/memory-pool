//! A Rust library providing a global, thread-safe memory pool,
//! ideal for when you need a temporary scratch buffer but you
//! don't want to be constantly allocating memory.
//!
//! ```rust
//! memory_pool::borrow( |aux: &mut String| {
//!     aux.push_str( "Do you like cupcakes?" );
//! });
//! ```
//!
//! ```rust
//! memory_pool::borrow( |vec: &mut Vec< u32 >| {
//!     vec.push( 1 );
//!     vec.push( 2 );
//!     vec.push( 3 );
//! });
//! ```
//! You can also manually acquire and release memory:
//!
//! ```rust
//! let mut buffer: String = memory_pool::acquire();
//! buffer.push_str( "I like cupcakes!" );
//! memory_pool::release( buffer );
//! ```

mod memory_pool;
mod poolable;

pub use memory_pool::{acquire, release, borrow};
