# 使用 rust 实现串口收发功能
- git clone https://github.com/thread-liu/rust-for-rt-thread.git
- rustup target add thumbv7em-none-eabihf
- cd Chapter1/uart
- cargo build --release
- arm-none-eabi-objcopy target/thumbv7em-none-eabihf/release/uart -O binary uart.bin

使用工具烧写 bliny.bin 文件；打开串口调试助手，连接 ST-Link 虚拟串口；串口调试助手会显示键盘输入的内容。
