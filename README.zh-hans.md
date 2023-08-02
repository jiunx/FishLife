# 模拟生物进化程序

本项目是一个模拟生物进化的程序，利用遗传算法、神经网络技术对鱼的眼睛和大脑进行模拟。该项目是使用 Rust 语言编写的，并编译为 WebAssembly (Wasm) 格式，使其可以在浏览器中运行。

## 特点

- **遗传算法**：利用遗传算法来模拟生物的自然选择和进化。
- **神经网络**：对鱼的大脑进行模拟，使其可以做出决策。
- **模拟眼睛**：模拟鱼的视觉系统，使其可以感知环境。
- **WebAssembly**：借助 Rust 的高性能优势，编译为 Wasm 格式并在浏览器中运行。

## 开始使用

### 安装依赖

确保您已经安装了 Rust 和 wasm-pack。如果还没有安装，请按照 [Rust 官方文档](https://www.rust-lang.org/) 和 [wasm-pack 官方文档](https://rustwasm.github.io/wasm-pack/) 进行安装。

### 构建

使用以下命令构建项目：

```bash
 wasm-pack build libs/simulation-wasm 
```

### 运行

```bash
npm run start -prefix web  
```
## 贡献

欢迎提交 pull request 来帮助改进或扩展此项目。

## 引用

- [Learning to Fly Pt.1](https://pwy.io/posts/learning-to-fly-pt1/)
- [Shorelark by Patryk27 on GitHub](https://github.com/Patryk27/shorelark)
