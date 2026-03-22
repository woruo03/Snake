# Snake 项目文件目录说明

本文档用于快速说明项目当前的代码组织方式与职责边界，帮助后续维护与扩展。

## 1. 根目录

- `Cargo.toml`：Rust 包配置与依赖声明（当前使用 Bevy 0.18.1）。
- `src/`：项目源代码。
- `docs/`：项目文档（本文件所在目录）。
- `plans/`：设计阶段文档（架构与系统设计草案）。

## 2. 代码入口

- `src/main.rs`
  - 应用入口。
  - 仅负责挂载 `SnakeGamePlugin`，不承载业务逻辑。

## 3. 游戏主模块 `src/game/`

- `src/game/mod.rs`
  - 游戏主插件 `SnakeGamePlugin`。
  - 负责：资源注册、消息注册、状态初始化、系统调度（Startup/Update/FixedUpdate）。

- `src/game/constants.rs`
  - 全局常量：窗口尺寸、网格尺寸、颜色、Z 轴层级、速度参数等。

- `src/game/state.rs`
  - 游戏状态与方向定义：`GameState`、`Direction`。

- `src/game/components.rs`
  - ECS 组件定义：蛇头/蛇身/食物/UI 标记/特效组件等。

- `src/game/resources.rs`
  - ECS 资源定义：配置、运行状态、边界、随机数状态、性能统计、相机抖动等。

- `src/game/messages.rs`
  - Bevy Message 定义：游戏结束、吃食物、分数变化、重置请求、蛇移动事件。

## 4. 系统模块 `src/game/systems/`

- `src/game/systems/mod.rs`
  - 系统子模块总入口。

- `src/game/systems/common.rs`
  - 公共工具函数。
  - 包括：网格坐标转换、游戏实体清理、初始化蛇、随机生成食物。

- `src/game/systems/setup.rs`
  - 启动与界面初始化相关系统。
  - 包括：相机、HUD、菜单、游戏结束文本、网格背景。

- `src/game/systems/input.rs`
  - 输入系统。
  - 包括：全局按键（Esc/F3）、菜单开始输入、重置输入、蛇方向输入。

- `src/game/systems/gameplay.rs`
  - 核心玩法系统。
  - 包括：蛇移动、食物碰撞、边界/自撞检测、游戏结束状态切换。

- `src/game/systems/reset.rs`
  - 回合重置系统。
  - 包括：进入 Playing 时重置、运行中按 R 的重开流程。

- `src/game/systems/ui.rs`
  - UI 数据更新系统。
  - 包括：分数与最高分文本同步。

- `src/game/systems/effects.rs`
  - 视觉反馈系统。
  - 包括：吃食物效果、尾迹、碰撞闪烁、浮动文本、生命周期清理、相机抖动。

- `src/game/systems/perf.rs`
  - 性能采样与显示系统。
  - 包括：FPS/实体统计采样、性能面板显示更新。

## 5. 调度关系（简化）

- `Startup`：初始化相机/UI/背景。
- `OnEnter(Menu)`：显示菜单。
- `OnEnter(Playing)`：重置回合并生成蛇/食物。
- `Update`：输入、UI、视觉效果、性能采样等。
- `FixedUpdate`：蛇移动、碰撞、得分等核心逻辑。

## 6. 扩展建议

- 新增玩法优先放在 `systems/gameplay.rs`，并通过 `messages.rs` 解耦。
- 新增特效优先放在 `systems/effects.rs`，避免污染核心玩法逻辑。
- 新增可调参数优先放在 `constants.rs` 或 `resources.rs`。
