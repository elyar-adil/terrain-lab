> **注意：** 本文描述的是早期面向地形→城市单一流水线的结构。项目已转向统一的生成式世界系统，
> 新的设计见 [`docs/architecture/world-system.md`](docs/architecture/world-system.md)。

# Procedural World Architecture

当前阶段实现一个完整的程序化世界 baseline。它必须独立生成地形、水文、生态、道路、城市、桥梁、建筑和最终影像，并提供确定性输出与可量化指标。

1. `terrain-core`：高程、坡度、曲率、海岸、河网、土壤、气候和生态覆盖。
2. `world-core`：世界种子、区域坐标、图层版本、任务编排和项目快照。
3. `infrastructure`：通行成本场、道路层级、桥梁与隧道候选点。
4. `settlement`：水源、地形、资源和交通驱动的聚落与城市生长。
5. `world-render`：卫星正射、3D 地表、建筑和大气渲染。
6. `procedural-render`：程序化卫星正射、3D 地表、建筑和大气渲染。
7. `ai-backends`：未来为每个模块分别提供可替换的 AI 后端，例如地形 AI、道路 AI、城市 AI、建筑 AI和遥感渲染 AI。

每个模块的程序化与 AI 后端遵循相同的输入输出协议，因此一个世界可以混合使用程序化地形、AI 道路、程序化城市和 AI 建筑。程序化 baseline 的长期用途包括训练数据生成、质量基线、确定性测试、约束参考和无模型回退。

当前版本只实现第一层和部分程序化渲染层。`terrain-core` 保持无 Tauri、无 DOM、无文件系统依赖。
