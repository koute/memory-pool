use std::mem;

/// A trait for objects that internally use a dynamically allocated buffer which can be safely
/// reused in new instances of those (or other `Poolable`) objects.
pub trait Poolable {
    /// Returns the internal buffer along with its capacity.
    fn get_buffer( &mut self ) -> (*mut u8, usize);

    /// Constructs a fresh, empty `Self`.
    fn empty() -> Self;

    /// Converts given `ptr` into an instance of `Self`. Will only be called with a non-null
    /// `ptr` and non-zero `capacity`.
    unsafe fn from_buffer( ptr: *mut u8, capacity: usize ) -> Self;
}

impl<T> Poolable for Vec<T> {
    fn get_buffer( &mut self ) -> (*mut u8, usize) {
        unsafe {
            (mem::transmute( self.as_mut_ptr() ), self.capacity() * mem::size_of::< T >())
        }
    }

    fn empty() -> Self {
        Vec::new()
    }

    unsafe fn from_buffer( ptr: *mut u8, capacity: usize ) -> Self {
        Vec::from_raw_parts( mem::transmute( ptr ), 0, capacity / mem::size_of::< T >() )
    }
}

impl Poolable for String {
    fn get_buffer( &mut self ) -> (*mut u8, usize) {
        unsafe {
            (mem::transmute( self.as_mut_vec().as_mut_ptr() ), self.capacity())
        }
    }

    fn empty() -> Self {
        String::new()
    }

    unsafe fn from_buffer( ptr: *mut u8, capacity: usize ) -> Self {
        String::from_raw_parts( mem::transmute( ptr ), 0, capacity )
    }
}
