#![no_main]
#![no_std]

use core::ptr::{read_volatile, write_volatile};

use cortex_m::{asm::nop, peripheral::syst::SystClkSource, Peripherals};
use cortex_m_rt::{entry, exception};
use panic_halt as _;

static mut DELAY_TICK: u32 = 0;

#[entry]
fn main() -> ! {
    const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
    unsafe {
        let ahb_val = read_volatile(RCC_AHB1ENR) & !(0b00);
        write_volatile(RCC_AHB1ENR, ahb_val | (1 << 0b00));
    }

    let dp = Peripherals::take().unwrap();
    let mut sys_tick = dp.SYST;

    sys_tick.set_clock_source(SystClkSource::Core);
    sys_tick.set_reload(16_000_000 / 1_000 - 1); // 1 second
    sys_tick.clear_current();
    sys_tick.enable_counter();
    sys_tick.enable_interrupt();

    const GPIOA_MODER: *mut u32 = 0x4002_0000 as *mut u32;
    const GPIOA_OTYPER: *mut u32 = 0x4002_0004 as *mut u32;
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
        delay(500);
        unsafe {
            write_volatile(GPIOA_BSRR, 0b1 << 5);
        }
        delay(500);
    }
}

fn delay(ms: u32) {
    unsafe {
        let start_time = DELAY_TICK;
        while DELAY_TICK < (start_time + ms) {
            cortex_m::asm::wfi();
        }
    }
}

#[exception]
fn SysTick() {
    unsafe {
        DELAY_TICK += 1;
    }
}
