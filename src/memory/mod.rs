mod physical;
use physical::EarlyPhysicalFrameAllocator;

use crate::multiboot::info::memory::MemoryMap;
use spin::Mutex;

pub use crate::arch::memory::{VirtualAddress, PhysicalAddress};

static ALLOCATOR: Mutex<Option<EarlyPhysicalFrameAllocator>> = Mutex::new(None);

pub fn initialize(map: MemoryMap<'static>) {
    ALLOCATOR.lock().replace(EarlyPhysicalFrameAllocator::new_from(map));
}
