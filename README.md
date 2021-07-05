# ゲーム

ash, winit を用いて開発中

## 環境構築

- マシン: Lenovo ThinkPad X230
- OS: Arch Linux

```bash
sudo pacman -S vulkan-intel vulkan-tools vulkan-validation-layers
```

## ドキュメント生成

```bash
cargo doc -p game --document-private-items
```
