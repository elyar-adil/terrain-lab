# 工程约定

新增任何生成器（物体、材质、图层）之前先读这页。目的只有一个：**同一件事只在一个地方实现**。

## 共用代码放哪里

| 需要 | 用 | 不要 |
| --- | --- | --- |
| `lerp` / `clamp01` / `smoothstep`（三次）/ `smootherstep`（五次） | `worldgen_core::{lerp, clamp01, smoothstep, smootherstep}`，对 `f32` 和 `f64` 通用 | 自己写一份，或写一个叫 `smoothstep` 的五次曲线 |
| 格点哈希 | `worldgen_core::hash::{cell01, avalanche01, mix32}` | 在 crate 里再写 `hash01` |
| 种子、可复现随机流、边标签 | `worldgen_core::{Seed, Rng}`、`hash::*` | `rand`、全局随机状态 |
| 图层、依赖、缓存 | `worldgen_core::{Layer, Engine}` | 在 crate 内部自建调度 |
| 跨模块的数据类型 | `worldgen-contracts` | 让一个生成器直接依赖另一个的实现 |

选哈希：`cell01` 很快，但相邻整数输入的输出相关（连续 8 个格子可能只出现 3 种值）；
凡是"在几个离散选项里选一个"（招牌颜色、树种、立面款式），用 `avalanche01`。

新的数学或哈希原语一律加到 `worldgen-core`（只依赖标准库），并附测试；不要加在使用它的那个 crate 里。

## 保持行为不变的重构

改共用基础会让很多输出同时变化，所以要先证明"没变"：

1. 在基线提交上用 `git worktree` 生成一组确定性输出（几张地形 PNG、两个种子的城市场景 JSON）；
2. 在改动后重新生成，比较 `sha256sum`；
3. 有意的变化要在提交信息里逐条说明，其余必须逐位相同。

## 检查

提交前本地跑（CI 跑同样的）：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --exclude wind-water-terrain-lab --all-targets -- -D warnings
cargo test --workspace --exclude wind-water-terrain-lab
npx tsc --noEmit
```

- lint 策略只在根 `Cargo.toml` 的 `[workspace.lints]` 里，每个 crate 用 `[lints] workspace = true`。
  允许的两条（`too_many_arguments`、`needless_range_loop`）在那里写明了理由。
- `src-tauri` 需要 GTK/WebKit 系统库，只在桌面环境构建；所有库 crate 是纯 Rust，CI 里直接检查。
- 编辑器设置见 `.editorconfig` 与 `rustfmt.toml`。

## 渲染检查

每个部件都要从多个角度看过。清单在 `docs/render-validation/matrix.json`（部件 × 角度，加一行即可新增）：

```bash
cargo run --release -p city-scene --example dump_scene -- --lite --out public/city-lite.json
npm run build:wasm            # 树的生长器
CHROMIUM_PATH=/path/to/chrome node scripts/render-matrix.mjs --only trees   # 或 city / facades / ground / prototypes
python3 scripts/contact-sheet.py render-out/trees render-out/sheets/trees.jpg
```

- 任何页面都接受 `?cam=x,y,z,tx,ty,tz[,fov]`，`city-audit.mjs` 用 `--cam` / `--query` 传入。
- 软件渲染很慢，城市镜头每张约 10 分钟；树和画廊每张几秒到一分钟。先跑后者。
- 输出在 `render-out/`（已忽略）。脚本会报告缺失材质和过短的 UV 缓冲。
- 画廊里的测试平面必须满足真实几何的契约，否则会"看起来是渲染坏了"：立面材质需要顶点色；
  烘焙墙面的 V 自上往下数（`facade/*` 以贴图高度计，`ground/*` 以米计）；树原型是单位高度。

## 测试的写法

- "任意两个东西不能太像"（调色板、树形轮廓、立面）要一次列出**所有**不合格的对子，而不是遇到第一对就 panic。
- 断言的量要有物理含义（米、反射率），阈值旁边写出为什么是这个数。
- 测试里测的坐标要用相对量（离墙多远），不要读绝对坐标。

## 已知、尚未统一的重复

这些是**有意留下**的：统一它们会改变现有输出，需要单独评审。

- 三种随机数发生器：`worldgen_core::Rng`（64 位计数器）、`city-scene::math::Rng`（mulberry32）、
  `procedural::SplitMix64`。
- 向量类型：`city-scene::{Vec2, Vec3}`、`worldgen-trees::V3`、`worldgen-contracts::V2`、`urban::Point`。
- 四份相互独立的 value noise 实现（terrain、geology、procedural、city-scene 的立面）。
- 三套树：`procedural::vegetation`、`worldgen-trees`、`city-scene::trees`。
- 材质仍是字符串（`"trim.dark"`），`BakedTexture` 有两个同名结构体。
- sRGB 编码/解码函数在 city-scene 内有多份。
