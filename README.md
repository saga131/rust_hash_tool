# Rust 哈希工具箱

这是一个使用 Rust 和 `egui` 编写的哈希计算工具，支持多种常见哈希算法及国密 SM3。

## 功能特点

*   **多算法支持**: MD5, SHA-1, SHA-2 (224/256/384/512), SHA-3, SM3 (商密), RIPEMD-160, Whirlpool, BLAKE2/3。
*   **加盐计算**: 支持前缀、后缀、前后缀加盐模式。
*   **实时计算**: 输入内容即时显示结果。
*   **中文界面**: 自动加载系统字体（Windows 下优先使用微软雅黑）。

## 运行方法

确保已安装 Rust 环境。

```bash
cargo run --release
```

## 依赖库

*   `eframe` / `egui`: GUI 框架
*   `md-5`, `sha-1`, `sha2`, `sha3`: 标准哈希算法
*   `sm3`: 国密算法
*   `hex`: 十六进制编码

## 注意事项

如果在非 Windows 系统上运行且出现中文乱码，请修改 `src/main.rs` 中的 `setup_custom_fonts` 函数，指向您系统中的有效中文字体路径。
