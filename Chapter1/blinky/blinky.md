# 使用 rust 点亮 ART-PI
- git clone https://github.com/thread-liu/rust-for-rt-thread.git
- cd Chapter1/bliny
- cargo build --release
- cargo objcopy target/thumbv7em-none-eabihf/release/blinky -- -O binary target/thumbv7em-none-eabihf/release/binky.bin

使用工具烧写 bliny.bin 文件， ART-Pi LED 等会交替闪烁
