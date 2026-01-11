# 文档更新规范

- 责任边界：文档维护与更新约束，避免 SRP 漂移与重复描述。
- 不负责：业务实现细节与代码改动说明。
- 目录映射：`docs/process/documentation-guidelines.md`。
- 交付物：文档更新规范与复核清单。
- 验收指标：文档职责清晰度、重复内容最小化。

## 1) 目标与适用范围

- 目标：防止 SRP 漂移、重复描述、职责混淆；保证文档可发现、可维护。
- 适用范围：`docs/frontend/*`、`docs/backend/*`、`docs/shared/*`、`docs/process/*`。

## 2) 核心原则（SRP 保障）

- 一个文档只解决一个核心问题：Why / What / How / Contract / Process 之一。
- 跨模块视角只允许出现在 `docs/backend/architecture.md` 与 `docs/process/*`。
- 共享契约仅允许出现在 `docs/shared/*`。

## 3) 必备模板

每个模块文档开头必须包含：

- 责任边界
- 不负责
- 目录映射
- 交付物
- 验收指标

**例外**：以下过程日志类文档可免模板：

- `docs/process/decisions.md`
- `docs/process/ux-ui-design-changelog.md`

## 4) 内容放置规则（强约束）

- 业务流程 → `docs/frontend/features.md` 或 `docs/backend/architecture.md`
- 组件实现规范 → `docs/frontend/ui-components.md`
- 布局/排版/主题 → `docs/frontend/layouts.md`
- 状态职责与边界 → `docs/frontend/state-management.md`
- 共享协议/命名/配置 → `docs/shared/*`
- 过程与记录（决策、任务、度量、评审） → `docs/process/*`

## 5) 前端专属规则

- UI 组件规范只放 `docs/frontend/ui-components.md`。
- 布局与排版只放 `docs/frontend/layouts.md`。
- 状态职责只放 `docs/frontend/state-management.md`。
- 业务流程与交互只放 `docs/frontend/features.md`。

## 6) 后端专属规则

- 模块职责只放各自模块文档（core/services/storage/search/api/plugins/export）。
- 跨模块流程只放 `docs/backend/architecture.md`。
- 共享契约（协议/命名/配置）只放 `docs/shared/*`。

## 7) 链接与路径规则（强约束）

- `docs/` 内文档必须使用**相对路径**链接（避免出现 `docs/docs/...` 形式）。
- 只在短期排查文档中使用行号链接，长期文档禁止固定行号。

## 8) 更新流程

1) 先确定内容归属模块（责任归属）。
2) 若涉及跨模块引用，只能写链接，不复制内容。
3) 更新后执行一次“SRP 快速复核”。
4) 重要变更追加到 `docs/process/decisions.md`。

## 9) SRP 快速复核清单

- 是否出现“同一职责在多个文档重复描述”？
- 是否有“实现细节”落入“规范文档”？
- 是否有新文档缺少五段模板？
- 跨模块是否用链接而非复制内容？
- 文档内链接是否使用相对路径？

## 10) Code Alignment Checklist 维护

- 更新触发：目录结构变更、新增核心模块/入口、文档重命名。
- 维护范围：仅列“入口级文件”，不覆盖所有子文件。
- 更新方式：新增/变更路径 → 更新清单 → 记录到 `docs/process/decisions.md`。
- 禁止事项：不写实现细节、不重复模块文档内容。
