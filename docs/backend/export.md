### 3.8 export（导出能力）

- 责任边界：导出协议与调度接口定义。
- 不负责：具体格式实现（由插件承担）。
- 目录映射：`crates/export/`、`crates/plugins/`。
- 交付物：导出流程与扩展点定义。
- 验收指标：导出可扩展性、格式解耦度。

## 设计概要

- export 仅保留导出协议与调度接口。
- 具体格式由 `plugins::hook::on_export` 提供。
- `services::export` 负责调度插件与产物打包。

## 路径索引（实现入口）

- 导出协议：`crates/export/src/`
- 插件导出钩子：`crates/plugins/src/hook/`
