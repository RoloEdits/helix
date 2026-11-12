use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

pub struct Arena(stumpalo::Arena);

impl Arena {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(stumpalo::Arena::new())
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(size: usize) -> Self {
        Self(stumpalo::Arena::with_capacity(size))
    }

    #[inline]
    pub fn clear(&mut self) {
        let arena = &mut self.0;
        arena.clear();
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Allocator for Arena {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let arena = &self.0;
        arena.try_alloc_layout(layout).map_err(|_| AllocError)
    }

    #[inline]
    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // Individual allocations are never freed.
    }
}
