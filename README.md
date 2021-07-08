# ゲーム

ash, glfw-rs を用いて開発中

## 環境構築

- マシン: Lenovo ThinkPad X230
- OS: Arch Linux

```bash
yay -S ninja cmake glfw-wayland \
    vulkan-intel vulkan-tools vulkan-validation-layers \
    glslang
```

## シェーダのコンパイル

GLSL シェーダ ( `.vert` , `.frag` ) をもとに SPIR-V バイナリを生成します。

```bash
cd shaders
ninja
```

### SPIR-V バイナリの削除

```bash
cd shaders
ninja -t clean
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
