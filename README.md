# 风与水 · 无限地貌实验室

桌面优先的程序化世界模型。当前阶段使用原生 Rust 生成地貌、气候、河网和地表覆盖，并合成可重复的正射自然色影像；后续世界层将在这些语义数据上生成道路、城市、桥梁和建筑。

项目的最终视觉验收标准见 [`QUALITY_BAR.md`](QUALITY_BAR.md)：生成的无人自然区域与同尺度真实卫星影像进行盲测时，人类观察者应无法可靠区分。

## 当前能力

- 干旱高山、温带湿润和高寒冰川三种地貌预设
- 山脉、丘陵、平原、高原、海岸和群岛六类独立地貌骨架
- 512²、1024²、2048² 原生模拟网格
- 固定随机种子与可重复输出
- 降水、蒸散、风向、太阳和大气参数
- 高程、坡度、森林、水体与积雪统计
- 可旋转缩放的 3D 地形、卫星纹理和动态程序化云层
- 2D 正射/3D 地形切换与全屏查看
- 2048、4096、8192 像素 PNG 卫星影像导出
- 完全本地计算，不需要账号、网络或遥测

## 技术结构

- `crates/terrain-core`：不依赖 Tauri 的纯 Rust 计算与影像合成核心
- `src-tauri`：桌面命令、后台任务和本地文件导出
- `src`：React/TypeScript 专业工作台

该结构允许未来为 `terrain-core` 增加 WASM 适配器，但当前版本只开发桌面客户端。

## 开发

需要 Node.js、npm、Rust stable、Windows WebView2 和 Visual Studio C++ Build Tools。

```bash
npm install
npm run desktop:dev
```

已编译的免安装程序位于 `target/release/wind-water-terrain-lab.exe`。创建正式安装包：

```bash
npm run desktop:build
```

构建前端与运行核心测试：

```bash
npm run build
cargo test -p terrain-core
```

生成一张不启动界面的测试影像：

```bash
cargo run -p terrain-core --example terrain_generate -- terrain-preview.png 1024
```

## 项目状态

当前为 Desktop Alpha。项目尚未选择开源许可证，在许可证确定前请勿公开分发或接收外部贡献。
