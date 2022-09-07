# rust的包管理工具Cargo
cargo 提供了一系列的工具，从项目的建立、构建到测试、运行直至部署，为 Rust 项目的管理提供尽可能完整的手段。同时，与 Rust 语言及其编译器 rustc 紧密结合.

# 创建项目
```shell
cargo new hello_world
```
**项目目录**
```shell
$ tree
.
├── .git
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```
**运行项目**
```shell
cd helllo_world
// 默认为debug模式,debug模式下rustc编译器不会对编译结果进行优化
cargo run --release
```
**cargo check**
cargo check 是我们在代码开发过程中最常用的命令，它的作用很简单：快速的检查一下代码能否编译通过。因此该命令速度会非常快，能节省大量的编译时间。

**Cargo.toml 和 Cargo.lock**
Cargo.toml 和 Cargo.lock 是 cargo 的核心文件，它的所有活动均基于此二者。

- Cargo.toml 是 cargo 特有的项目数据描述文件。它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml。

- Cargo.lock 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们一般不用修改它，只需要对着 Cargo.toml 文件撸就行了。

> 什么情况下该把 Cargo.lock 上传到 git 仓库里？很简单，当你的项目是一个可运行的程序时，就上传 Cargo.lock，如果是一个依赖库项目，那么请把它添加到 .gitignore 中

**package 配置段落**
package中记录了项目的描述信息,典型的如下:
```toml
[package]
name = "world_hello"
version = "0.1.0"
edition = "2021"
```
**项目依赖**
使用 cargo 工具的最大优势就在于，能够对该项目的各种依赖项进行方便、统一和灵活的管理。

在 Cargo.toml 中，主要通过各种依赖段落来描述该项目的各种依赖项：

- 基于 Rust 官方仓库 crates.io，通过版本说明来描述
- 基于项目源代码的 git 仓库地址，通过 URL 来描述
- 基于本地项目的绝对路径或者相对路径，通过类 Unix 模式的路径来描述

示例
```toml
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```
