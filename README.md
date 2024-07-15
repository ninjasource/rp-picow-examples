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

