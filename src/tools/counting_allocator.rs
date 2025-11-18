use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAllocator;

static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout)
        }
    }
}

#[global_allocator]
static A: CountingAllocator = CountingAllocator;

pub fn counting_allocator() -> usize {
    ALLOCATION_COUNT.load(Ordering::Relaxed)
}