use std::cell::UnsafeCell;
use std::mem;

use poolable::Poolable;

thread_local!( static POOL: UnsafeCell< MemoryPool > = UnsafeCell::new( MemoryPool::new() ) );

struct MemoryPool {
    buffers: Vec< (*mut u8, usize) >
}

impl MemoryPool {
    fn new() -> MemoryPool {
        MemoryPool {
            buffers: Vec::new()
        }
    }

    #[inline]
    fn acquire<T>( &mut self ) -> T where T: Poolable {
        match self.buffers.pop() {
            None => T::empty(),
            Some( (ptr, capacity) ) => unsafe { T::from_buffer( ptr, capacity ) }
        }
    }

    #[inline]
    fn release<T>( &mut self, mut value: T ) where T: Poolable {
        unsafe {
            let (ptr, capacity) = value.get_buffer();
            if capacity != 0 {
                mem::forget( value );
                self.buffers.push( (ptr, capacity) );
            }
        }
    }

    #[inline]
    fn borrow<T, F, R>( &mut self, callback: F ) -> R where F: FnOnce( &mut T ) -> R, T: Poolable {
        let mut value = self.acquire::<T>();
        let result = callback( &mut value );
        self.release::<T>( value );

        result
    }
}

impl Drop for MemoryPool {
    fn drop( &mut self ) {
        for &(ptr, capacity) in self.buffers.iter() {
            let vector = unsafe { Vec::from_raw_parts( ptr, 0, capacity ) };
            mem::drop( vector );
        }
    }
}

#[inline]
fn with_pool<F>( callback: F ) where F: FnOnce( &mut MemoryPool ) {
    POOL.with( |pool_cell| {
        callback( unsafe { mem::transmute( pool_cell.get() ) } );
    });
}

/// Constructs an object of type `T` with memory from the thread-local pool.
pub fn acquire<T>() -> T where T: Poolable {
    let mut result = unsafe { mem::uninitialized() };
    with_pool( |pool| {
        let mut tmp = pool.acquire::< T >();
        mem::swap( &mut result, &mut tmp );
        unsafe {
            mem::forget( tmp );
        }
    });

    result
}

/// Destroys the `value` and transfers its internal memory buffer back into the thread-local pool.
pub fn release<T>( value: T ) where T: Poolable {
    with_pool( |pool| {
        pool.release( value );
    });
}

/// Constructs a temporary instance of `T` using the memory from the thread-local pool.
pub fn borrow<F, T, R>( callback: F ) -> R where F: FnOnce( &mut T ) -> R, T: Poolable {
    let mut result = None;
    with_pool( |pool| {
        result = Some( pool.borrow( callback ) );
    });

    result.unwrap()
}

#[cfg(test)]
mod tests {
    mod memory_pool {
        pub use super::super::*;
    }

    #[test]
    fn borrow_string() {
        memory_pool::borrow( |aux: &mut String| {
            // We should get a clean buffer at first.
            assert_eq!( aux.len(), 0 );
            assert_eq!( aux.capacity(), 0 );
            aux.push_str( "Hello World!" );
            assert_eq!( aux.len(), 12 );
            assert!( aux.capacity() >= 12 );
            assert_eq!( aux, "Hello World!" );
        });

        memory_pool::borrow( |aux: &mut String| {
            // We should get the same buffer we got previously.
            assert_eq!( aux.len(), 0 );
            assert!( aux.capacity() >= 12 );
            unsafe { aux.as_mut_vec().set_len( 12 ) };
            assert_eq!( aux, "Hello World!" );
        });
    }

    #[test]
    fn acquire_and_release_string() {
        let mut string: String = memory_pool::acquire();
        assert_eq!( string.len(), 0 );
        assert_eq!( string.capacity(), 0 );
        string.push_str( "I like cupcakes!" );
        memory_pool::release( string );

        let string: String = memory_pool::acquire();
        assert_eq!( string.len(), 0 );
        assert!( string.capacity() >= 16 );
    }

    #[test]
    fn borrow_string_and_vector() {
        memory_pool::borrow( |aux: &mut String| {
            aux.push_str( "Do you like cupcakes?" );
            aux.shrink_to_fit();
            assert_eq!( aux.capacity(), 21 );
        });
        memory_pool::borrow( |vec: &mut Vec< u32 >| {
            assert_eq!( vec.capacity(), 21 / 4 );
        });
    }
}
