use super::tags::MemoryMapTag;

#[derive(Debug)]
pub struct MemoryMap<'a> {
    tag: &'a MemoryMapTag
}

impl<'a> MemoryMap<'a> {
    pub fn regions(&self) -> Regions {
        Regions::new(self.tag)
    }
}

impl<'a> From<&'a MemoryMapTag> for MemoryMap<'a> {
    fn from(tag: &MemoryMapTag) -> MemoryMap {
        MemoryMap { tag }
    }
}

impl<'a> core::fmt::Display for MemoryMap<'a> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.regions().fold(Ok(()), |result, region| result.and_then(|_| writeln!(formatter, "{}", region)))
    }
}

use core::marker::PhantomData;

pub struct Regions<'a> {
    current: *const Region,
    last: *const Region,
    entry_size: u32,
    phantom: PhantomData<&'a Region>
}

impl<'a> Regions<'a> {
    fn new(tag: &'a MemoryMapTag) -> Regions<'a> {
        Regions {
            current: tag.first_region(),
            last: tag.last_region(),
            entry_size: tag.entry_size,
            phantom: PhantomData
        }
    }

    fn current(&self) -> Option<&'a Region> {
        if self.current > self.last {
            None
        } else {
            Some(unsafe { &*self.current })
        }
    }

    fn advance(&self) -> *const Region {
        (self.current as usize + self.entry_size as usize) as *const Region
    }
}

use tap::tap::Tap;

impl<'a> Iterator for Regions<'a> {
    type Item = &'a Region;

    fn next(&mut self) -> Option<&'a Region> {
        self.current().tap(|region| {
            if region.is_some() {
                self.current = self.advance()
            }
        })
    }
}

#[repr(C)]
pub struct Region {
    base:   u64,
    length: u64,
    kind:   u32
}

impl Region {
    pub fn starts_at(&self) -> usize {
        self.base as usize
    }

    pub fn ends_at(&self) -> usize {
        self.starts_at() + self.length()
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn kind(&self) -> Kind {
        match self.kind {
            1 => Kind::Available,
            3 => Kind::Reclaimable,
            4 => Kind::Nonvolatile,
            5 => Kind::Defective,
            _ => Kind::Reserved
        }
    }
}

impl core::fmt::Display for Region {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "[{:#016x}-{:#016x}] {}", self.starts_at(), self.ends_at(), self.kind())
    }
}

impl core::fmt::Debug for Region {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.debug_struct("Region")
            .field("starts_at", &format_args!("{:#x}", self.starts_at()))
            .field("ends_at", &format_args!("{:#x}", self.ends_at()))
            .field("length", &self.length())
            .field("kind", &self.kind())
            .finish()
    }
}

#[derive(Debug)]
pub enum Kind {
    Reserved,
    Available,
    Reclaimable,
    Nonvolatile,
    Defective
}

impl Kind {
    fn to_str(&self) -> &str {
        match self {
            Kind::Reserved    => "Reserved",
            Kind::Available   => "Available",
            Kind::Reclaimable => "Reclaimable (ACPI)",
            Kind::Nonvolatile => "Non-volatile (ACPI)",
            Kind::Defective   => "Defective"
        }
    }
}

impl core::fmt::Display for Kind {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "{}", self.to_str())
    }
}
