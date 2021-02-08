// Segment Tree Allocator Implement

use super::Allocator
use alloc::{vec, vec::Vec}

pub struct SegmentTreeAllocator {
    tree: Vec<u8>,
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {

    }

    fn alloc(&mut self) -> Option<usize> {
        
    }
}