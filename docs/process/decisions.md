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

## 维护检查清单新增（2026-01-11）

- 新增：`docs/process/maintenance-checklist.md`
- 已加入 `docs/index.md` 导航

## 架构摘要新增（2026-01-11）

- 新增：`docs/architecture-summary.md`
- 已加入 `docs/index.md` 快速入口

## 文档更新规范补充（2026-01-11）

- 在 `docs/process/documentation-guidelines.md` 增加 Code Alignment Checklist 维护规则

## 文档更新规范补充（2026-01-11）

- 增加前端专属规则与后端专属规则

## 项目入口文档新增（2026-01-11）

- 新增：`project-context.md`
- 已加入 `docs/index.md` 快速入口

## 渲染服务 SRP 拆分设计（2026-01-11）

- services::render 设计约束：RenderHtml 仅负责编排，数据加载、Markdown→HTML、HTML 后处理、安全净化分别独立
- Inline 渲染策略复用：RenderHtml 与 RenderMarkmap 共享 inline 规则以保证预览一致性

## Render 文档归属调整（2026-01-11）

- 归属修正：`docs/backend/render.md` 调整为 `docs/backend/services/render.md`
- 导航更新：`docs/index.md`、`docs/backend/overview.md`、`docs/architecture-summary.md`、`docs/backend/services.md`

## HTML 渲染净化开关（2026-01-11）

- 新增环境变量：`KNOWLATTICE_RENDER_HTML_SANITIZE`，用于开启 HTML 输出净化
- 默认关闭：预览一致性优先，导出/分享可按需开启

## Inline 渲染 SRP 拆分（2026-01-11）

- 自研 inline 渲染保持性能优先，不使用 comrak 片段渲染
- 拆分职责：文本提取 / Markdown 序列化 / HTML 序列化

## Inline 渲染接口化（2026-01-11）

- 抽象接口：InlineContext / InlineFormat / InlineRenderEngine
- 规则与遍历解耦：RenderEngine 与 Markmap 复用同一规则接口

## Markdown 渲染分层落地（2026-01-11）

- 分层目录：traits / source / classify / tree / inline / serializer
- RenderMarkdown 仅依赖 traits，具体实现下沉到各层

## Markmap 渲染分层落地（2026-01-12）

- 分层目录：traits / pipeline / inline / classify / source / config
- RenderMarkmap 仅依赖 traits，pipeline 与适配层各自解耦

## RenderMarkmap 入口纯化（2026-01-12）

- 入口拆分：mod.rs 仅导出/注册，registry 负责装配，service 仅编排
- RenderMarkmap 字段仅保留运行期端口（input/options/transformer/initializer/folder）

## Markmap 表格节点输出 HTML（2026-01-12）

- 表格节点 `content` 直接输出 HTML `<table>` 片段
- RenderHtml 与 Markmap 复用 MarkdownSerializer + Comrak，保持预览一致性

## Markmap 懒加载协议扩展（2026-01-14）

- 新增懒加载字段约定：`payload.has_children`、`payload.children_loaded`、`payload.children_count?`
- 新增配置项：`markmap.load_mode`（`"full" | "lazy"`），见 `docs/shared/config-scopes.md`
- 协议归属：`docs/shared/markmap-protocol.md`，服务文档仅保留链接引用

## Markmap Outline 模式与指示器后端化（2026-01-14）

- 新增加载模式：`outline`，用于输出仅包含 heading 的大纲树（见 `docs/shared/markmap-protocol.md`）。
- 新增字段：`payload.show_children_indicator`，由后端计算，前端只读。
- 配置更新：`markmap.load_mode.root`/`markmap.load_mode.child` 支持 `"full" | "lazy" | "outline"`（见 `docs/shared/config-scopes.md`）。

## Markmap 子节点数量展示（2026-01-14）

- `payload.children_count` 用于折叠圆中的子节点数量展示（协议见 `docs/shared/markmap-protocol.md`）。
- 视觉渲染仅消费协议字段，不新增前端推导逻辑。

## Markmap ListItem 空文本修复（2026-01-14）

- Inline 引擎需展开 Paragraph 作为 inline 容器，保证 ListItem 文本可渲染。

## Markmap ListItem Paragraph 承载规则（2026-01-14）

- Markmap 的 ListItem 内容由 Paragraph 子节点承载时，需在 markmap inline 适配层展开 Paragraph。
