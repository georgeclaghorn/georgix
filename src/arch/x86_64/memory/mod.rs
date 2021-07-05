mod addressing;
pub(super) mod segmentation;

pub use addressing::{PhysicalAddress, VirtualAddress};

pub(super) fn initialize() {
    segmentation::initialize();
}
