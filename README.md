# Raspberry Pi Pico W examples

This repo was created to demonstrate a simple network interaction between two rp picow dev boards with minimal external circuitry required. 

## Setup

Install bootloader tool:
```
cargo install elf2uf2-rs
```

Wifi connection settings:

In order to connect to the wifi network please create the following two files in the `src` folder:
`WIFI_SSID.txt` and `WIFI_PASSWORD.txt`
The files above should contain the `exact` ssid and password to connect to the wifi network. No newline characters or quotes.

## Troubleshooting

Compile errors:
```
couldn't read `src/WIFI_SSID.txt`: No such file or directory (os error 2)
couldn't read `src/WIFI_PASSWORD.txt`: No such file or directory (os error 2)
```
Solution: You need to create the file above with the wifi network name in the file. No quotes required and no newlines either!

## What is the `memory.x` file? 

One of the last steps in compilation is linking which is the process of assigning physical memory addreses to variables and code.
On a computer with an operating system the OS uses virtual memory but embedded systems like the rp-pico don't have an OS 
and we need to create an executable with physical memory addresses in the correct locations that are expected by the pico. 
The `memory.x` file is the the developer facing linker script that tells the linker when RAM and FLASH physically start. 
If you look at `.cargo/config.toml` you will see a whole bunch of linker scripts referenced there. The `link.x` script references `memory.x`

## How can this be compiled on a PC and run on a pico?

Rust supports cross compilation and this is setup in the `.cargo/config.toml` file with the following config:

```
[build]
target = "thumbv6m-none-eabi" # Cortex-M0 and Cortex-M0+
```