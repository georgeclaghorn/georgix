use crate::multiboot::info::memory::*;
use crate::memory::PhysicalAddress;

use arrayvec::ArrayVec;
use tap::tap::Tap;

pub struct EarlyPhysicalFrameAllocator<const S: usize = 32> {
    available: ArrayVec<Region, S>,
    reserved:  ArrayVec<Region, S>
}

impl<const S: usize> EarlyPhysicalFrameAllocator<S> {
    pub fn new_from(map: MemoryMap) -> EarlyPhysicalFrameAllocator<S> {
        EarlyPhysicalFrameAllocator::new().tap_mut(|allocator| allocator.add_from(map))
    }

    pub fn new() -> EarlyPhysicalFrameAllocator<S> {
        EarlyPhysicalFrameAllocator {
            available: ArrayVec::new(),
            reserved:  ArrayVec::new()
        }
    }

    pub fn add(&mut self, base: PhysicalAddress, length: usize) {
        self.available.push(Region { base, length })
    }

    pub fn add_from(&mut self, map: MemoryMap) {
        for region in map.regions() {
            if let Available = region.kind() {
                self.add(region.starts_at(), region.length())
            }
        }
    }

    pub fn reserve(&mut self, base: PhysicalAddress, length: usize) {
        self.reserved.push(Region { base, length })
    }

    pub fn allocate(size: usize, alignment: usize) -> Result<PhysicalAddress, AllocationError> {
        Err(AllocationError)
    }
}

#[derive(Copy, Clone)]
struct Region {
    base:   PhysicalAddress,
    length: usize
}

pub struct AllocationError;
