#![no_main]
#![no_std]

use core::arch::{asm, global_asm};
use core::fmt::Write;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("boot.S"));

use core::panic::PanicInfo;

mod uart;

extern "C" {
    static HEAP_START: usize;
}

#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

#[no_mangle]
pub extern "C" fn kernel_main() {
    use core::ptr;
    unsafe {
        const GPFSEL1: *mut u32 = 0x3F200004 as _;
        const GPPUD: *mut u32 = 0x3F200094 as _;
        const GPPUDCLK0: *mut u32 = 0x3F200098 as _;
        let r = (ptr::read_volatile(GPFSEL1) & !((7<<12)|(7<<15))) | (2<<12)|(2<<15);
        ptr::write_volatile(GPFSEL1, r);
        ptr::write_volatile(GPPUD, 0);
        for i in 0..150 { asm!("nop"); }
        ptr::write_volatile(GPPUDCLK0, (1<<14)|(1<<15));
        for i in 0..150 { asm!("nop"); }
        ptr::write_volatile(GPPUDCLK0, 0);        // flush GPIO setup

        let mut uart = uart::UART::new();
        uart.init();
        let _ = write!(&mut uart, "Hello world\n");
        loop {
            match uart.read_byte() {
                b'\r' => uart.write_byte(b'\n'),
                0x7F  => {
                    uart.write_str("\x1B[1D");
                    uart.write_str("\x1B[K");
                },
                r => {
                    uart.write_byte(r);
                },
            }
        }
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo<'_>) -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
