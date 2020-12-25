mod rsdp;
use rsdp::RSDP;

use spin::Mutex;

static RSDP: Mutex<Option<RSDP>> = Mutex::new(None);

pub fn initialize() {
    if let Some(rsdp) = RSDP::find() {
        RSDP.lock().replace(rsdp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_the_rsdp_on_boot() {
        assert!(RSDP.lock().is_some())
    }
}
