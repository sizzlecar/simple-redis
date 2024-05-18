# Simple Redis by Rust

这是一个用 Rust 实现的简单 Redis 服务器。它旨在提供一个轻量级、高性能的 Redis 服务器，可以用于学习和研究 Redis 协议和数据结构。

## 功能

- 支持基本的 Redis 命令，如 GET、SET、DEL 等。
- 使用异步 I/O 和多线程来处理并发连接。
- 提供简单的持久化功能。

## 如何使用

首先，你需要安装 Rust。然后，你可以使用以下命令来编译和运行服务器：

```bash
cargo build --release
./target/release/simple_redis
