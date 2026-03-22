# Snake Game (Bevy 0.18.1)

一个使用Rust语言和Bevy游戏引擎开发的贪吃蛇游戏，采用ECS架构，具有现代视觉效果和流畅的游戏体验。

## 特性

- 🎮 **经典贪吃蛇玩法** - 控制蛇移动、吃食物、避免碰撞
- ⚡ **高性能** - 使用Bevy ECS架构，优化性能配置
- 🎨 **现代视觉效果** - 粒子效果、摄像机震动、UI动画
- 📊 **性能监控** - 实时显示FPS和系统性能
- 🎯 **状态管理** - 清晰的游戏状态（菜单、游戏中、游戏结束）
- 🔧 **模块化设计** - 代码结构清晰，易于维护和扩展

## 系统要求

- Rust 1.70+ (推荐使用最新稳定版)
- Cargo包管理器

## 安装与运行

### 克隆项目
```bash
git clone <repository-url>
cd Snake
```

### 运行游戏
```bash
# 开发模式（带优化）
cargo run

# 发布模式（极致优化）
cargo run --release
```

### 构建
```bash
# 开发构建
cargo build

# 发布构建
cargo build --release
```

## 游戏控制

- **方向键** - 控制蛇的移动方向
- **空格键** - 在菜单中开始游戏，在游戏结束时重新开始
- **ESC键** - 退出游戏
- **F3键** - 显示/隐藏性能监控面板

## 项目结构

```
Snake/
├── Cargo.toml          # Rust项目配置
├── Cargo.lock          # 依赖锁定
├── README.md           # 项目说明文档
├── docs/               # 文档目录
│   └── file_structure.md
└── src/                # 源代码
    ├── main.rs         # 程序入口
    └── game/           # 游戏逻辑模块
        ├── mod.rs      # 主插件定义
        ├── components.rs   # ECS组件定义
        ├── constants.rs    # 游戏常量
        ├── messages.rs     # 事件消息
        ├── resources.rs    # 游戏资源
        ├── state.rs        # 游戏状态
        └── systems/        # 系统模块
            ├── common.rs   # 通用工具
            ├── effects.rs  # 特效系统
            ├── gameplay.rs # 核心游戏逻辑
            ├── input.rs    # 输入处理
            ├── perf.rs     # 性能监控
            ├── reset.rs    # 重置系统
            ├── setup.rs    # 初始化系统
            └── ui.rs       # UI系统
```

## 技术架构

### ECS架构

项目采用Bevy的ECS（Entity Component System）架构：

- **组件（Components）**：定义实体属性（位置、方向、颜色等）
- **系统（Systems）**：处理游戏逻辑（移动、碰撞、渲染等）
- **资源（Resources）**：全局数据（分数、配置、随机状态等）
- **状态（States）**：游戏状态机（菜单、游戏中、游戏结束）

### 主要组件

- `SnakeHead` - 蛇头组件，包含当前和下一个方向
- `SnakeSegment` - 蛇身段组件
- `Food` - 食物组件
- `GridPos` - 网格位置组件
- `Lifetime` - 特效生命周期组件
- `PulseEffect`, `FadeEffect`, `UiFloatEffect` - 动画效果组件

### 游戏状态

- `GameState::Menu` - 菜单界面
- `GameState::Playing` - 游戏进行中
- `GameState::GameOver` - 游戏结束

### 消息系统

- `GameOverMsg` - 游戏结束事件
- `FoodEatenMsg` - 食物被吃事件
- `ScoreChangedMsg` - 分数变化事件
- `ResetRequestedMsg` - 重置请求事件
- `SnakeMovedMsg` - 蛇移动事件

## 开发配置

### 性能优化

项目配置了多级优化：

```toml
# 开发模式优化
[profile.dev.package."*"]
opt-level = 3  # 第三方依赖优化

[profile.dev]
opt-level = 1  # 项目代码优化

# 发布模式优化
[profile.release]
lto = true          # 链接时优化
codegen-units = 1   # 提高运行速度
```

### 依赖

- **bevy = "0.18.1"** - 游戏引擎

## 视觉效果

- **网格背景** - 深色主题网格线
- **蛇身渐变** - 蛇头和蛇身使用不同颜色
- **食物特效** - 食物被吃时产生粒子效果
- **移动轨迹** - 蛇移动时留下淡出轨迹
- **摄像机震动** - 碰撞时产生震动效果
- **UI动画** - 分数变化时的浮动效果

## 性能特性

- **固定时间步长** - 蛇移动使用5Hz固定更新
- **性能监控** - 实时显示FPS、帧时间、系统数量
- **资源清理** - 自动清理过期特效实体
- **高效碰撞检测** - 基于网格位置的快速碰撞检测

## 扩展与修改

### 添加新功能

1. **新组件**：在[`src/game/components.rs`](src/game/components.rs)中添加
2. **新系统**：在[`src/game/systems/`](src/game/systems/)中创建新模块
3. **新资源**：在[`src/game/resources.rs`](src/game/resources.rs)中定义
4. **新状态**：在[`src/game/state.rs`](src/game/state.rs)中扩展

### 修改游戏参数

- 窗口尺寸：修改[`src/game/constants.rs`](src/game/constants.rs)中的`WINDOW_WIDTH`和`WINDOW_HEIGHT`
- 网格大小：修改`GRID_SIZE`常量
- 蛇移动速度：修改`SNAKE_FIXED_HZ`常量
- 颜色主题：修改颜色常量

## 故障排除

### 常见问题

1. **编译错误**：确保Rust版本为1.70+，运行`rustup update`
2. **运行缓慢**：使用`cargo run --release`获得最佳性能
3. **依赖问题**：删除`Cargo.lock`并重新运行`cargo build`

### 调试

- 按F1键显示性能监控面板
- 查看控制台日志输出
- 使用`cargo check`检查代码错误

## 贡献

欢迎提交Issue和Pull Request来改进这个项目。

## 许可证

本项目采用MIT许可证。详见LICENSE文件（如有）。

## 致谢

- [Bevy Engine](https://bevyengine.org/) - 优秀的Rust游戏引擎
- Rust社区 - 提供丰富的库和工具

---

**开始游戏**：运行`cargo run`，使用方向键控制蛇的移动！
