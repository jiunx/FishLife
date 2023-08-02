# 模擬生物進化程序

本項目是一個模擬生物進化的程序，利用遺傳算法、神經網絡技術對魚的眼睛和大腦進行模擬。該項目是使用 Rust 語言編寫的，並編譯為 WebAssembly (Wasm) 格式，使其可以在瀏覽器中運行。

## 特點

- **遺傳算法**：利用遺傳算法來模擬生物的自然選擇和進化。
- **神經網絡**：對魚的大腦進行模擬，使其可以做出決策。
- **模擬眼睛**：模擬魚的視覺系統，使其可以感知環境。
- **WebAssembly**：借助 Rust 的高性能優勢，編譯為 Wasm 格式並在瀏覽器中運行。

## 開始使用

### 安裝依賴

確保您已經安裝了 Rust 和 wasm-pack。如果還沒有安裝，請按照 [Rust 官方文檔](https://www.rust-lang.org/) 和 [wasm-pack 官方文檔](https://rustwasm.github.io/wasm-pack/) 進行安裝。

### 構建

使用以下命令構建項目：

```bash
 wasm-pack build libs/simulation-wasm 
```

### 運行

```bash
npm run start -prefix web  
```

## 貢獻

歡迎提交 pull request 來幫助改進或擴展此項目。

## 引用

- [Learning to Fly Pt.1](https://pwy.io/posts/learning-to-fly-pt1/)
- [Shorelark by Patryk27 on GitHub](https://github.com/Patryk27/shorelark)
