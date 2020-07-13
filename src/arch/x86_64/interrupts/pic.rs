use crate::arch::x86_64::io::Port;

pub struct ChainedPIC {
    parent: PIC,
    child: PIC
}

impl ChainedPIC {
    pub fn new(parent: PIC, child: PIC) -> ChainedPIC {
        ChainedPIC { parent, child }
    }

    pub fn disable(&mut self) {
        self.parent.disable();
        self.child.disable();
    }
}

#[allow(dead_code)]
pub struct PIC {
    command_port: Port,
    data_port: Port
}

impl PIC {
    pub fn new(command_port: u16, data_port: u16) -> PIC {
        PIC {
            command_port: Port::new(command_port),
            data_port: Port::new(data_port)
        }
    }

    pub fn disable(&mut self) {
        unsafe { self.data_port.write(0xFFu8); }
    }
}
