# learn_rust
学习Rust
### cargo
- cargo build
    - 创建一个可执行文件位于target/debug/hello_cargo.ex
    - 在顶层目录生成一个Cargo.lock文件
    - 该文件负责追踪项目依赖的精确版本，不要手动修改
- cargo run  编译代码+执行结果
    - 如果之前编译成功且源码未修改，则直接运行二进制文件

- cargo check 检查代码，确保可以通过编译，且不会生成可执行文件
    - cargo check 比cargo build 快得多

- cargo build --release  为发布构建
    - 编译时会优化
    - 代码会运行更快，但编译时间更长
    - 会在target/release 生成可执行文件

### let,mut,use
- use 导入库 use::std::io;  表示从变量准库导入输入输出
- let 使用let声明一个变量
    - rust中所有变量默认不可变，如果想修改就使用mut
- mut 配合let声明一个可变变量
- {}  格式化字符串



