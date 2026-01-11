# 项目模块设计文档（Rust 后端）

- 责任边界：后端文档导航与总体说明。
- 不负责：各模块实现细节。
- 目录映射：`docs/backend/`。
- 交付物：入口索引与高层结构说明。
- 验收指标：可发现性与导航完整性。

## 1. 目标与范围

- 目标：为基于 markmap 的 Markdown 编辑软件提供可维护、可扩展的后端结构
- 范围：Rust 后端（Tauri 侧），采用 workspace + 多 crate 组织

## 2. 总体架构

- 采用 Rust workspace，Tauri 入口仅做 wiring
- 业务逻辑与存储、索引、导出解耦
- 以 Workspace 为核心边界（聚合根），支持多个根目录（Folder）

## 3. 目录结构管理（高层）

```
.
├─ docs/
├─ src/
├─ src-tauri/             # Tauri 入口与后端适配
├─ crates/                # Rust workspace
│  ├─ core/               # 领域模型
│  ├─ services/           # 用例编排
│  ├─ storage/            # 持久化
│  ├─ search/             # 索引与检索
│  ├─ export/             # 导出协议
│  ├─ api/                # 命令与 DTO 边界
│  ├─ plugins/            # 扩展与钩子
│  └─ common/             # 共享基础设施
└─ ...
```

## 4. 模块入口

- core：`docs/backend/core-domain.md`
- services：`docs/backend/services.md`
- storage：`docs/backend/storage.md`
- search：`docs/backend/search.md`
- export：`docs/backend/export.md`
- api/dto：`docs/backend/api-dto.md`
- common：`docs/backend/common.md`
- plugins：`docs/backend/plugins.md`
- tauri 入口：`docs/backend/tauri-entry.md`
- 依赖/数据流/用户流程：`docs/backend/architecture.md`
- Markmap 协议：`docs/shared/markmap-protocol.md`
- 配置策略：`docs/shared/config-scopes.md`
