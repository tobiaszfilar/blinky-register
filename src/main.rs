#![no_main]
#![no_std]

use core::ptr::{read_volatile, write_volatile};

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
    unsafe {
        let ahb_val = read_volatile(RCC_AHB1ENR) & !(0b00);
        write_volatile(RCC_AHB1ENR, ahb_val | (1 << 0b00));
    }

    const GPIOA_MODER: *mut u32 = 0x4002_0000 as *mut u32;
    const GPIOA_OTYPER: *mut u32 = 	0x4002_0004 as *mut u32;
    const GPIOA_OSPEEDR: *mut u32 = 0x4002_0008 as *mut u32;
    const GPIOA_BSRR: *mut u32 = 0x4002_0018 as *mut u32;
    unsafe {
        let moder_val = read_volatile(GPIOA_MODER) & !(0b11 << 10);
        write_volatile(GPIOA_MODER, moder_val | (0b01 << 10));

        let otyper_val = read_volatile(GPIOA_OTYPER) & !(0b0 << 5);
        write_volatile(GPIOA_OTYPER, otyper_val | (0b0 << 5));

        let ospeedr_value = read_volatile(GPIOA_OSPEEDR) & !(0b01 << 10);
        write_volatile(GPIOA_OSPEEDR, ospeedr_value | (0b01 << 10));
    }

    loop {
        unsafe {
            write_volatile(GPIOA_BSRR, 0b1 << 21);
        }
        for _ in 0..40_000 {
            nop();
        }
        unsafe {
            write_volatile(GPIOA_BSRR, 0b1 << 5);
        }
        for _ in 0..40_000 {
            nop();
        }
    }
}
