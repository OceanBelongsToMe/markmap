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

## 4) 内容放置规则（强约束）

- 业务流程 → `docs/frontend/features.md` 或 `docs/backend/architecture.md`
- 组件实现规范 → `docs/frontend/ui-components.md`
- 布局/排版/主题 → `docs/frontend/layouts.md`
- 状态职责与边界 → `docs/frontend/state-management.md`
- 共享协议/命名/配置 → `docs/shared/*`
- 过程与记录（决策、任务、度量、评审） → `docs/process/*`

## 5) 更新流程

1. 先确定内容归属模块（责任归属）。
2. 若涉及跨模块引用，只能写链接，不复制内容。
3. 更新后执行一次“SRP 快速复核”。
4. 重要变更追加到 `docs/process/decisions.md`。

## 6) SRP 快速复核清单

- 是否出现“同一职责在多个文档重复描述”？
- 是否出现相同业务，跨模块表述？
- 是否有“实现细节”落入“规范文档”？
- 是否有新文档缺少五段模板？
- 是否有跨模块内容未用链接而是重复正文？
