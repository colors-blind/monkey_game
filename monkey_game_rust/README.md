# Monkey Game (Rust 版)

使用 macroquad 2D 游戏库重构的"金箍棒过桥"小游戏。

## 依赖

- Rust toolchain (rustup 安装)
- Linux 下需要: `libx11-dev libxi-dev libgl1-mesa-dev` (大多数桌面发行版已预装)

```bash
# Ubuntu/Debian
sudo apt install libx11-dev libxi-dev libgl1-mesa-dev
```

## 构建与运行

```bash
cd monkey_game_rust

# Debug 模式运行
cargo run

# Release 模式 (性能更好)
cargo run --release
```

## 玩法

- **按住鼠标左键**: 金箍棒向上生长
- **松开鼠标左键**: 金箍棒向右倒下形成桥
- 如果棒子够到对面平台，猴子走过去得分
- 如果棒子太短或太长（超出平台），猴子掉落 Game Over
- **Game Over 后点击**: 重新开始
