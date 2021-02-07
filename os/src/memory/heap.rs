/// Heap space used for dynamic memory allocation
///
/// The size is [`KERNEL_HEAP_SIZE`]
/// After compilation, this space will be placed in the bss section of 
/// the OS execution program
/// 
use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;


static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

/// Heap, dynamic memory allocator
///
/// ### `#[global_allocator]`
/// [`LockedHeap`] implements the [`alloc::alloc::GlobalAlloc`] trait,
/// You can allocate space where the heap is needed globally. For example `Box` `Arc` etc.
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

// init the space of heap when OS start running
pub fn init() {
    // Tell the allocator to use this reserved space as a heap
    unsafe {
        HEAP.lock().init(
            HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
        )
    }
}

// Callback when space is allocated incorrectly, panic!
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("alloc error")
}