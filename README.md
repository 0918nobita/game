# ノベルゲーム

[![Lint](https://github.com/0918nobita/novel-game/actions/workflows/lint.yml/badge.svg)](https://github.com/0918nobita/novel-game/actions/workflows/lint.yml)  [![Format](https://github.com/0918nobita/novel-game/actions/workflows/format.yml/badge.svg)](https://github.com/0918nobita/novel-game/actions/workflows/format.yml)  [![Test](https://github.com/0918nobita/novel-game/actions/workflows/test.yml/badge.svg)](https://github.com/0918nobita/novel-game/actions/workflows/test.yml)

## Requirements

- Common
    - Vulkan SDK
- Windows
    - Visual Studio
- macOS / Linux
    - GLFW3

## Build

### Windows

Open `/NovelGame.sln` with Visual Studio

### macOS / Linux

```bash
make -j
```

## Lint

```bash
cpplint --recursive --quiet .
```

## Vulkan わからん

### パイプライン

- 描画したい物体の形状が与えられる
- 配置したい位置の位置の情報をもとに、画面上での形状を決める
- 描画範囲内にあるピクセルを選択する (ラスタライズ)
- ピクセルそれぞれに対して、物体の材質や光の当たり方といった情報をもとに色を決定する

パイプラインの各段階を**ステージ**という

- 3Dグラフィックスを描画するための**グラフィックパイプライン**
- 汎用的な計算を行うための**コンピュートパイプライン**

が存在する。


### プリミティブ

直線や三角形といった、GPU がラスタライズできる基本的な図形

### シェーダ

ステージで実行されるプログラム

#### 頂点シェーダ

入力：１つの頂点  
出力：１つの頂点

#### テッセレーション制御シェーダ (省略可能)

入力：複数の頂点  
出力：↑で構成される多角形がどの程度細かく分割されるべきか

#### テッセレーション評価シェーダ (省略可能)

テッセレーションによってできた頂点に座標やその他の頂点属性の値を与える

#### ジオメトリシェーダ (省略可能)

入力：１つのプリミティブ  
出力：任意の個数のプリミティブ

#### フラグメントシェーダ

ラスタライザによって色を求める必要があると判断された各ピクセルに対して１回ずつ実行される

#### コンピュートシェーダ

任意のスレッド数で任意のデータを入出力できる

### スワップチェーン

レンダリング結果のイメージをを２つ以上保持して、自転車のチェーンのように順番に表示するものと書き換えるものを取り替えていく仕組み

## リンク集

- [Vulkan in 30 minutes](https://renderdoc.org/vulkan-in-30-minutes.html)
- [Getting Started with the macOS Vulkan SDK](https://vulkan.lunarg.com/doc/sdk/1.2.162.1/mac/getting_started.html)
