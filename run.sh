#!/bin/sh
set -e
cargo build
mkdir -p iso/EFI/BOOT iso/boot/limine
cp /usr/share/limine/BOOTX64.EFI iso/EFI/BOOT/
cp limine.conf iso/boot/limine/
cp target/x86_64-unknown-none/debug/hobby-os iso/boot/kernel
qemu-system-x86_64 -machine q35 -m 256M \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/x64/OVMF_CODE.4m.fd \
  -drive format=raw,file=fat:rw:iso \
  -serial stdio
