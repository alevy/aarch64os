use core::arch::asm;
use core::fmt::Write;
use core::ptr;
use core::str;

const AUX_ENABLE:     *mut u32 = 0x3F215004 as _;
const AUX_MU_IO:      *mut u32 = 0x3F215040 as _;
const AUX_MU_IER:     *mut u32 = 0x3F215044 as _;
const AUX_MU_IIR:     *mut u32 = 0x3F215048 as _;
const AUX_MU_LCR:     *mut u32 = 0x3F21504C as _;
const AUX_MU_MCR:     *mut u32 = 0x3F215050 as _;
const AUX_MU_LSR:     *mut u32 = 0x3F215054 as _;
const AUX_MU_MSR:     *mut u32 = 0x3F215058 as _;
const AUX_MU_SCRATCH: *mut u32 = 0x3F21505C as _;
const AUX_MU_CNTL:    *mut u32 = 0x3F215060 as _;
const AUX_MU_STAT:    *mut u32 = 0x3F215064 as _;
const AUX_MU_BAUD:    *mut u32 = 0x3F215068 as _;

pub struct UART();

impl UART {
    pub const unsafe fn new() -> UART {
        UART()
    }

    pub fn init(&mut self) {
        unsafe {
            ptr::write_volatile(AUX_ENABLE, ptr::read_volatile(AUX_ENABLE) | 1);
            ptr::write_volatile(AUX_MU_CNTL, 0);
            ptr::write_volatile(AUX_MU_LCR, 3);       // 8 bits
            ptr::write_volatile(AUX_MU_MCR, 0);
            ptr::write_volatile(AUX_MU_IER, 0);
            ptr::write_volatile(AUX_MU_IIR, 0xc6);    // disable interrupts
            ptr::write_volatile(AUX_MU_BAUD, 270);    // 115200 baud

            ptr::write_volatile(AUX_MU_CNTL, 3); // enable Tx, Rx
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            while ptr::read_volatile(AUX_MU_LSR) & 0x20 == 0 {}
            ptr::write_volatile(AUX_MU_IO, byte as u32);
        }
    }

    pub fn write_bytes(&mut self, s: &[u8]) {
        for byte in s.iter() {
            self.write_byte(*byte);
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        unsafe {
            while ptr::read_volatile(AUX_MU_LSR) & 0x01 == 0 {}
            ptr::read_volatile(AUX_MU_IO) as u8
        }
    }
}

impl Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}
