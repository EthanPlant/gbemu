pub struct CPU {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    pc: u16,
    sp: u16,
    memory_mut: Arc<Mutex<[u8; 65536]>>,
}

impl CPU {
    pub fn new(memory_mut: Arc<Mutex<[u8; 65536]>>) -> CPU {
        let af = 0;
        let bc = 0;
        let de = 0;
        let hl = 0;
        let pc = 0x0100;
        let sp = 0xFFFE;
        CPU {
            af,
            bc,
            de,
            hl,
            pc,
            sp,
            memory_mut,
        }
    }   
}