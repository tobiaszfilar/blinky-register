#![no_main]
#![no_std]

use core::ptr::{read_volatile, write_volatile};

use cortex_m::{peripheral::syst::SystClkSource, Peripherals};
use cortex_m_rt::{entry, exception};
use panic_halt as _;

static mut DELAY_TICK: u32 = 0;

macro_rules! LedOn {
    () => {
        core::ptr::write_volatile(0x4002_0018 as *mut u32, 0b1 << 5);
    };
}

macro_rules! LedOff {
    () => {
        core::ptr::write_volatile(0x4002_0018 as *mut u32, 0b1 << 21);
    };
}

macro_rules! ButtonPressed {
    () => {
        core::ptr::read_volatile(0x4002_0410 as *mut u32) & (1 << 12) == 0
    };
}

macro_rules! Toggle {
    () => {
        core::ptr::write_volatile(
            0x4002_0014 as *mut u32,
            core::ptr::read_volatile(0x4002_0014 as *const u32) ^ (1 << 5),
        );
    };
}

#[entry]
fn main() -> ! {
    configure_systick();
    configure_l13();
    configure_user_button();

    loop {
        unsafe {
            if ButtonPressed!() {
                LedOn!();
                delay(1000);
                LedOff!();
                delay(1000);
            } else {
                Toggle!();
                delay(300);
            }
        }
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

fn configure_systick() {
    let dp = Peripherals::take().unwrap();
    let mut sys_tick = dp.SYST;

    sys_tick.set_clock_source(SystClkSource::Core);
    sys_tick.set_reload(16_000_000 / 1_000 - 1); // 1 second
    sys_tick.clear_current();
    sys_tick.enable_counter();
    sys_tick.enable_interrupt();
}

fn configure_l13() {
    const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
    unsafe {
        let ahb_val = read_volatile(RCC_AHB1ENR) & !(0b1 << 0);
        write_volatile(RCC_AHB1ENR, ahb_val | (0b1 << 0));
    }

    const GPIOA_MODER: *mut u32 = 0x4002_0000 as *mut u32;
    const GPIOA_OTYPER: *mut u32 = 0x4002_0004 as *mut u32;
    const GPIOA_OSPEEDR: *mut u32 = 0x4002_0008 as *mut u32;
    unsafe {
        let moder_val = read_volatile(GPIOA_MODER) & !(0b11 << 10);
        write_volatile(GPIOA_MODER, moder_val | (0b01 << 10));

        let otyper_val = read_volatile(GPIOA_OTYPER) & !(0b0 << 5);
        write_volatile(GPIOA_OTYPER, otyper_val | (0b0 << 5));

        let ospeedr_value = read_volatile(GPIOA_OSPEEDR) & !(0b01 << 10);
        write_volatile(GPIOA_OSPEEDR, ospeedr_value | (0b01 << 10));
    }
}

fn configure_user_button() {
    const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
    unsafe {
        let ahb_val = read_volatile(RCC_AHB1ENR) & !(0b1 << 1);
        write_volatile(RCC_AHB1ENR, ahb_val | (0b1 << 1));
    }

    const GPIOB_MODER: *mut u32 = 0x4002_0400 as *mut u32;
    const GPIOB_PUPDR: *mut u32 = 0x4002_000c as *mut u32;

    unsafe {
        let moder_val = read_volatile(GPIOB_MODER) & !(0b00 << 24);
        write_volatile(GPIOB_MODER, moder_val | (0b00 << 24));

        let pupdr_val = read_volatile(GPIOB_PUPDR) & !(0b01 << 24);
        write_volatile(GPIOB_PUPDR, pupdr_val | (0b01 << 24));
    }
}

#[allow(non_snake_case)]
#[exception]
fn SysTick() {
    unsafe {
        DELAY_TICK += 1;
    }
}
