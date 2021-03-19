# Development Guide

## Developing on Windows

### Setup development environment

Download and install Vulkan SDK from [here](https://vulkan.lunarg.com/sdk/home#windows)

Install Visual Studio 2019 with .NET 5.0 SDK, then you can open and use this solution in Visual Studio:

```powershell
.\NovelGame.sln
```

Restore NuGet packages using Visual Studio

## Developing on macOS or Linux

### Setup development environment

(Only on macOS) Install Command Line Tools for Xcode

```bash
xcode-select --install
```

Install GLFW library

```bash
# macOS
brew install glfw
```

```bash
# Ubuntu 20.04
sudo apt update
sudo apt install libglfw3-dev
```

### Build

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
