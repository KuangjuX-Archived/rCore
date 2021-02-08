// Segment Tree Allocator Implement

use super::Allocator;
use alloc::{vec, vec::Vec};

pub struct SegmentTreeAllocator {
    tree: Vec<u8>,
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        Self{
            tree: vec![(0, capacity)]
        }
    }

    fn alloc(&mut self) -> Option<usize> {

    }

    fn dealloc(&mut selfm index: usize) {

    }
}