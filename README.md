# Novel Game

[![Lint](https://github.com/0918nobita/novel-game/actions/workflows/lint.yml/badge.svg)](https://github.com/0918nobita/novel-game/actions/workflows/lint.yml)  [![Build](https://github.com/0918nobita/novel-game/actions/workflows/test.yml/badge.svg)](https://github.com/0918nobita/novel-game/actions/workflows/test.yml)

## Requirements

- Common to all supported platforms
    - [.NET 5.0 SDK](https://dotnet.microsoft.com/download/dotnet/5.0)
    - [Vulkan SDK](https://vulkan.lunarg.com/)
        - Ubuntu: `sudo apt install libvulkan-dev`
- Only on Windows
    - Visual Studio 2019
- Common to macOS and Linux
    - Make
    - [GLFW3](https://www.glfw.org/)
        - macOS: `brew install glfw`
        - Ubuntu: `sudo apt install libglfw3-dev`
- Only on macOS
    - Apple clang (included in Command Line Tools for Xcode)
- Only on Linux
    - GNU C++ Compiler v10.x

## Windows

### Build / Run

1. Open `/NovelGame.sln` with Visual Studio 2019
2. Install NuGet packages
3. Build `NovelGame` project

## macOS / Linux

Recommended IDEs: Visual Studio Code, CLion

```bash
make
```

### Run

```bash
make run
```

### Test

```bash
make test
```

### Lint

```bash
make lint
```

### Format

```bash
make format
```
