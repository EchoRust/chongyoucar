# 重庆邮电大学小车控制 Rust SDK

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][doc-badge]][doc-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/badge/crates-0.1.0-yellow
[crates-url]: https://crates.io/crates/chongyoucar
[doc-badge]: https://img.shields.io/badge/doc-latest-blue
[doc-url]: https://docs.rs/chongyoucar
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT

重庆邮电大学小车控制的 `Rust SDK`。

`MickRobot` 是一个开源小车制作项目，教程内容包含底盘的搭建过程，基于激光雷达的导航内容。项目中小车的3D图、控制板原理图和PCB文件、控制代码均为开源。。[MickRobot链接](https://mickrobot.github.io/)

## 使用方法

```toml
[dependencies]
chongyoucar = "0.1.0"
```

## 前进和左转

```rust
fn main() {
    let listen = chongyoucar::connect("/dev/ttyUSB0").expect("can not connect car");

    for _ in 0..3 {
        listen.send((0.2, 0.0)).ok();
        listen.send((0.0, 0.2)).ok();
    }
}
```

## 作者介绍

* 作者: 李扬(Leon)
* 个人网站: [https://echoli.cn](https://echoli.cn)

## License

The MIT License (MIT)

Copyright (c) 2024-present, 李扬(Leon)
