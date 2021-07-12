# ゲーム

Rust + `ash` + `glfw-rs` で開発中

## 環境構築

### Arch Linux の場合

```bash
yay -S ninja cmake glfw-wayland \
    vulkan-intel vulkan-tools vulkan-validation-layers \
    glslang
```

### Windows の場合

1. [Vulkan SDK](https://vulkan.lunarg.com/) をインストールする

2. [MSYS2](https://www.msys2.org/) をインストールする

3. MSYS2 ターミナル上で `gcc` , `migw32-make` , `cmake` コマンドを使えるようにする

```bash
pacman -Syuu
pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-make mingw-w64-x86_64-cmake
```

4. PowerShell から↑のコマンドを呼び出せるように、MSYS2 の bin ディレクトリにPATHを通す  
例: `C:\msys64\mingw64\bin` (MSYS2 インストーラで設定を変更しなければこのパスになっているはず)

5. PowerShell 上で Rust のツールチェーンを変更する

```powershell
rustup target add x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
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

### ログ出力を有効にして実行

#### Linux

```bash
RUST_LOG=game cargo run
```

#### Windows

```powershell
$env:RUST_LOG = "game"
cargo run
```

### バリデーションレイヤを無効化して実行

```bash
cargo run --no-default-features
```

## コードフォーマット

```bash
cargo fmt
```

## ドキュメント生成

```bash
cargo doc -p game --document-private-items --open
```

## 参考文献

- [初めてのvulkanプログラム第１章 ②Vulkan プログラムでの処理の流れ](https://qiita.com/tositada_nakada/items/a2522fa249c61ef3b7de) tositada_nakada, 2019
- [Vulkan Programming Vol.1](https://booth.pm/ja/items/1286100) すらりんラボ, 2019
- [Vulkan Tutorial](https://vulkan-tutorial.com/)
- [vulkan_samples_2019_rust](https://github.com/aosoft/vulkan_samples_2019_rust) aosoft, 2019
- [vulkan-triangle-rs](https://github.com/adrien-ben/vulkan-triangle-rs) adrien-ben, 2021
