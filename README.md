# Hello World - LED Blinking on NUCLEO-F411CE

This is a simple embedded "Hello World" project for the [NUCLEO-F411CE](https://kamami.pl/zestawy-uruchomieniowe-stm32/570384-ka-nucleo-f411cev2-plytka-rozwojowa-z-mikrokontrolerem-stm32f411ce-5906623433308.html) development board, demonstrating basic LED blinking functionality. The project is implemented at the register level, without using HAL (Hardware Abstraction Layer) or PAC (Peripheral Access Crate) libraries.

## Hardware

- **Board**: NUCLEO-F411CEv2 by Kamani
- **Microcontroller**: STM32F411CE
- **LED Pin**: PA5 (built-in user LED on the NUCLEO board)

## Features

- Blinks the built-in LED on the NUCLEO-F411CE development board.
- Direct register manipulation without abstraction libraries like HAL or PAC.
- Demonstrates basic bare-metal embedded development on STM32.

## Prerequisites

- **Rust toolchain** with `cargo` and `rustup`.
- `probe-rs` for flashing and debugging the microcontroller.
- STM32 NUCLEO-F411CE board connected via USB.

## Build and Flash

Configuration in `.cargo/config.toml` allows you to build and flash the program using `cargo build` and `cargo run` commands. Make sure your environment is properly set up to interface with the STM32 microcontroller.

### Step-by-step instructions:

1. Clone the repository:

   ```bash
   git clone https://github.com/tobiaszfilar/blinky-register
   cd blinky-register
   cargo build
   cargo run
This setup uses probe-rs to handle flashing and debugging directly from the command line.

## No HAL or PAC

This project directly manipulates the registers of the STM32F411CE microcontroller, avoiding the use of abstraction layers like HAL (Hardware Abstraction Layer) and PAC (Peripheral Access Crate). It provides a hands-on approach to understanding how the STM32 peripherals work at the register level.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
