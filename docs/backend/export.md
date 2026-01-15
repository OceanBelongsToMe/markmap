### 3.8 export（导出能力）

- 责任边界：导出协议与导出管线入口（格式能力在 export crate 内聚）。
- 不负责：业务调度之外的具体 UI 交互与前端行为。
- 目录映射：`crates/export/`、`crates/services/src/export/`。
- 交付物：导出协议、导出管线骨架与服务入口。
- 验收指标：导出接口可扩展性与格式解耦度。

## 设计概要

- export 仅保留导出协议与调度接口。
- `crates/export` 内聚格式与管线（format/pipeline/theme）。
- `services::export` 负责导出入口与调度（当前为占位实现）。

## 路径索引（实现入口）

- 导出协议与管线：`crates/export/src/`
- 导出服务入口：`crates/services/src/export/`
