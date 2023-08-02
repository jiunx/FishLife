# Evolution Simulation Program

This project simulates biological evolution utilizing genetic algorithms and neural networks to emulate the eyes and brains of fish. It's written in Rust and compiled to WebAssembly (Wasm) to run in web browsers.

![Aug-02-2023 15-04-03.gif](Aug-02-2023%2015-04-03.gif)

## Language Versions

- [简体中文 (Simplified Chinese)](README.zh-hans.md)
- [繁體中文 (Traditional Chinese)](README.zh-hant.md)

## Features

- **Genetic Algorithms**: Emulate natural selection and evolution.
- **Neural Networks**: Simulate fish brains allowing decision-making.
- **Eye Simulation**: Emulate fish visual systems for environmental perception.
- **WebAssembly**: High-performance execution with Rust, compiled to Wasm for browser deployment.

## Getting Started

### Prerequisites

Ensure you have both Rust and wasm-pack installed. If not, please refer to the [official Rust documentation](https://www.rust-lang.org/) and [wasm-pack documentation](https://rustwasm.github.io/wasm-pack/) for installation instructions.

### Building

Build the project using:

```bash
wasm-pack build libs/simulation-wasm
```

### Running

```bash
npm run start -prefix web  
```
## Contribution

Pull requests to help improve or extend this project are welcome.

## References

- [Learning to Fly Pt.1](https://pwy.io/posts/learning-to-fly-pt1/)
- [Shorelark by Patryk27 on GitHub](https://github.com/Patryk27/shorelark)
