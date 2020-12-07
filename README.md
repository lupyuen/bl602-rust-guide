# BL602 Rust Guide for PineCone

Refer to the article...

["Quick Peek of PineCone BL602 RISC-V Evaluation Board"](https://lupyuen.github.io/articles/pinecone)

## Install and run OpenOCD for macOS

Install driver for Sipeed JTAG Debugger...

https://docs.platformio.org/en/latest/plus/debug-tools/sipeed-rv-debugger.html#id2

Connect Sipeed JTAG Debugger to PineCone. Enter...

```bash
git clone --recursive https://github.com/sipeed/bl602-pac
git clone --recursive https://github.com/sipeed/bl602-hal
git clone --recursive https://github.com/lupyuen/bl602-rust-guide
cd bl602-rust-guide
rustup target add riscv32imac-unknown-none-elf
cargo build

# Alternatively...
# cargo build --example bl602-gpio-blinky

wget https://github.com/xpack-dev-tools/riscv-none-embed-gcc-xpack/releases/download/v8.3.0-2.3/xpack-riscv-none-embed-gcc-8.3.0-2.3-darwin-x64.tar.gz
tar xvf xpack-riscv-none-embed-gcc-8.3.0-2.3-darwin-x64.tar.gz
ln -s "$PWD/xpack-riscv-none-embed-gcc-8.3.0-2.3/bin/riscv-none-embed-gdb" "$PWD/xpack-riscv-none-embed-gcc-8.3.0-2.3/bin/riscv64-unknown-elf-gdb"

wget https://github.com/xpack-dev-tools/openocd-xpack/releases/download/v0.10.0-14/xpack-openocd-0.10.0-14-darwin-x64.tar.gz
tar -xvf xpack-openocd-0.10.0-14-darwin-x64.tar.gz
xpack-openocd-0.10.0-14/bin/openocd
```

We should see...

```
xPack OpenOCD, x86_64 Open On-Chip Debugger 0.10.0+dev-00378-ge5be992df (2020-06-26-12:31)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Ready for Remote Connections
Info : clock speed 2000 kHz
Warn : Haven't made progress in mpsse_flush() for 2039ms.
Warn : Haven't made progress in mpsse_flush() for 4078ms.
Warn : Haven't made progress in mpsse_flush() for 8157ms.
```

Open another Command Prompt and enter...

```bash
cd bl602-rust-guide
export PATH="$PWD/xpack-riscv-none-embed-gcc-8.3.0-2.3/bin:$PATH"
cargo run

# Alternatively...
# cargo run --example bl602-gpio-blinky
```

This shows a GDB error. Perhaps because the JTAG Pins are connected to the LED?

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `riscv64-unknown-elf-gdb -q -x openocd.gdb target/riscv32imac-unknown-none-elf/debug/bl602-rust-guide`
Reading symbols from target/riscv32imac-unknown-none-elf/debug/bl602-rust-guide...
openocd.gdb:1: Error in sourced command file:
:3333: Operation timed out.
```

If the Sipeed JTAG Debugger is not detected, we will see in OpenOCD...

```
Error: no device found
Error: unable to open ftdi device with vid 0403, pid 6010, description '*', serial '*' at bus location '*'
```

## Install and run OpenOCD for Windows

Install driver for Sipeed JTAG Debugger...

https://docs.platformio.org/en/latest/plus/debug-tools/sipeed-rv-debugger.html#id2

Download OpenOCD from...

https://github.com/xpack-dev-tools/openocd-xpack/releases/download/v0.10.0-15/xpack-openocd-0.10.0-15-win32-x64.zip

TODO

## Try it out!

Open one terminal and execute:

```
openocd
```

Open another and execute:

```
cargo run --example bl602-gpio-blinky
```
## License

This project is licensed under either of Mulan PSL v2 or MIT.

```
Copyright (c) 2020 Sipeed Co.,Ltd.
bl602-hal is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:

    http://license.coscl.org.cn/MulanPSL2

THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
```
