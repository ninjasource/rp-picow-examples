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

## Running

Unplug the pico usb, hold down the BOOTSEL button when plugging it back in (you can now release the BOOTSEL button). 
Windows will automatically mount the drive but you will have to do it manually on Linux (just click on RPI-RP2 in your file explorer and you will see two files)

Now run the following command:

`cargo run --release --bin 01_logs` 


If the pico is NOT in boot mode you will get the following error:
```
david@mint-pc:~/source/rp-picow-examples$ cargo run --release --bin 01_logs
    Finished `release` profile [optimized + debuginfo] target(s) in 0.14s
     Running `elf2uf2-rs --deploy --serial --verbose target/thumbv6m-none-eabi/release/01_logs`
Error: "Unable to find mounted pico"
```

However, If everything worked correctly you will get the following in your console:
```
david@mint-pc:~/source/rp-picow-examples$ cargo run --release --bin 01_logs
    Finished `release` profile [optimized + debuginfo] target(s) in 0.14s
     Running `elf2uf2-rs --deploy --serial --verbose target/thumbv6m-none-eabi/release/01_logs`
Found pico uf2 disk /media/david/RPI-RP2
Detected FLASH binary
Mapped segment 0x10000100->0x100001c0 (0x10000100->0x100001c0)
Mapped segment 0x100001c0->0x1000797c (0x100001c0->0x1000797c)
Mapped segment 0x1000797c->0x100090a0 (0x1000797c->0x100090a0)
Mapped segment 0x100090a0->0x10009174 (0x20000000->0x200000d4)
Mapped segment 0x10000000->0x10000100 (0x10000000->0x10000100)
Transfering program to pico
Page 0 / 146 0x10000000
Page 1 / 146 0x10000100
Page 2 / 146 0x10000200
Page 3 / 146 0x10000300
...
Page 142 / 146 0x10008e00
Page 143 / 146 0x10008f00
Page 144 / 146 0x10009000
Page 145 / 146 0x10009100

Found pico serial on /dev/ttyACM0
started
count: 0
count: 1
count: 2
count: 3
count: 4
count: 5
count: 6
```

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