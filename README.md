# ゲーム

ash, glfw-rs を用いて開発中

## 環境構築

- マシン: Lenovo ThinkPad X230
- OS: Arch Linux

```bash
sudo pacman -S cmake glfw-wayland vulkan-intel vulkan-tools vulkan-validation-layers
```

## 実行

```bash
cargo run
```

### バリデーションレイヤを無効化して実行

```bash
cargo run --no-default-features
```

## ドキュメント生成

```bash
cargo doc -p game --document-private-items
```
