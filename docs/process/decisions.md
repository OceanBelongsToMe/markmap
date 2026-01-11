# 设计决策记录

## 文档按模块拆分方案（SRP）

### 1) 拆分原则

- 每份文档只解决一个核心问题：Why / What / How / Contract / Process 任选其一。
- 文档结构与代码结构对齐：`src` → frontend，`src-tauri`/`crates` → backend。
- “跨端稳定契约”统一放入 `shared/`（API、DTO、错误码、协议、命名）。
- 导航入口文档只负责“索引与阅读路径”，不承载细节。

### 2) 目标目录结构（docs）

```
docs/
  index.md                     # 导航入口（阅读顺序 + 路径）
  frontend/
    information-architecture.md
    i18n.md
    layouts.md
    ui-components.md
    state-management.md
    features.md
    accessibility.md
  backend/
    tauri-entry.md
    core-domain.md
    services.md
    storage.md
    search.md
    export.md
    api-dto.md
    error-model.md
  shared/
    markmap-protocol.md
    naming-conventions.md
    config-scopes.md
  process/
    task-log.md
    decisions.md
```

### 3) 现有文档迁移映射

- `docs/ux-ui-design.md` → `docs/frontend/*`
  - IA / I18n / Layouts / State / Features / UI / Accessibility 按现有章节拆分。
- `docs/module-design.md` → `docs/backend/*` + `docs/shared/*`
  - Tauri 入口策略 → `backend/tauri-entry.md`
  - core / services / storage / search / export → 对应 backend 子文件
  - Markmap SRP / 初始化协议 → `shared/markmap-protocol.md`
  - 配置表与读取策略 → `shared/config-scopes.md`
- `docs/task.md` → `docs/process/task-log.md`

### 4) 文档模板（统一规范，防止膨胀）

每份文档固定四段：

1) 责任边界（负责 / 不负责）
2) 与代码目录映射（入口文件或目录）
3) 对外契约（若有）
4) 依赖与关联文档链接

### 5) 连续使用编程助手的阅读路径（写入 index.md）

- 初次上手：index → frontend/IA → frontend/layouts → backend/tauri-entry
- UI 任务：frontend/ui-components → frontend/state-management
- 数据/渲染：backend/services → backend/search → shared/markmap-protocol

## SRP 拆分复核结论（2026-01-11）

- 前端模块文档职责清晰分离：IA / I18n / Features / Layouts / UI Components / A11y
- 过程类文档（Review/Changelog/Metrics）与前端设计文档不交叉
- `docs/frontend/state-management.md` 仍为占位，不影响边界
- 结论：当前拆分满足 SRP，无业务重叠

## SRP 复核结论（2026-01-11）

- 状态管理文档已补齐（`docs/frontend/state-management.md`），职责边界清晰
- 前端与后端拆分文档未发现新增重叠
- 结论：SRP 复核通过

## 文档一致性模板复核（2026-01-11）

- 前端/后端/共享文档已统一补齐：责任边界 / 不负责 / 目录映射 / 交付物 / 验收指标
- 复核结果：SRP 保持一致，无新增重叠

## 命名规范文档新增（2026-01-11）

- 新增跨端命名规范：`docs/shared/naming-conventions.md`
- 来源：`src/ia/` 与 `crates/common/src/types/`

## 文档更新规范新增（2026-01-11）

- 新增：`docs/process/documentation-guidelines.md`
- 已加入 `docs/index.md` 导航

## 轻量代码对齐复核（2026-01-11）

- 对齐范围：`src-tauri/src/lib.rs`、`crates/services/src/builder.rs`、`crates/api/src/dispatch/*`
- 结论：文档与入口/边界实现一致，无 SRP 重叠
- 文案微调：api-dto 与 services 说明更贴合实际实现

## 后端架构文档精炼（2026-01-11）

- `docs/backend/architecture.md` 删除模块归属细节与 SQL 表结构
- 以链接指向 `docs/backend/services.md` 与 `docs/backend/storage.md`
- 复核结果：SRP 通过

## 后端存储文档精炼（2026-01-11）

- `docs/backend/storage.md` 压缩实现细节，保留边界与路径索引
- 保留少量示例以辅助快速理解
- 复核结果：SRP 通过

## 后端搜索文档精炼（2026-01-11）

- `docs/backend/search.md` 收敛为三层结构概览 + 路径索引
- 保留少量示例以辅助理解
- 复核结果：SRP 通过

## 后端 API/DTO 文档精炼（2026-01-11）

- `docs/backend/api-dto.md` 收敛为边界/约定/路径索引
- 复核结果：SRP 通过

## 后端 common 文档精炼（2026-01-11）

- `docs/backend/common.md` 收敛为高层约定 + 路径索引 + 少量示例
- 复核结果：SRP 通过

## 后端 core 文档精炼（2026-01-11）

- `docs/backend/core-domain.md` 收敛为原则/约定/入口索引
- 保留少量示例以辅助理解
- 复核结果：SRP 通过

## 后端 export/plugins 文档精炼（2026-01-11）

- `docs/backend/export.md` 与 `docs/backend/plugins.md` 收敛为高层约定 + 路径索引
- 复核结果：SRP 通过
