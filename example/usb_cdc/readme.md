# 使用 rust 实现 USB 串口收发功能
- git clone https://github.com/thread-liu/rust-for-rt-thread.git
- rustup target add thumbv7em-none-eabihf
- cd example/usb_cdc
- cargo build --release
- arm-none-eabi-objcopy target/thumbv7em-none-eabihf/release/usb_cdc -O binary usb_cdc.bin

使用工具烧写 usb_cdc.bin 文件；打开串口调试助手，连接虚拟串口；串口调试助手会显示键盘输入的内容。
