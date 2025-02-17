#!/bin/bash
set -e

# mount pico (you may need to change the device depending on your setup)
udisksctl mount -b /dev/sdb1

elf2uf2-rs --deploy --serial --term $1

