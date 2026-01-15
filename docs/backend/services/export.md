# Services / Export

- 责任边界：导出用例入口与调度编排说明。
- 不负责：导出格式与管线实现细节（由 export crate 承担）。
- 目录映射：`crates/services/src/export/`。
- 交付物：导出编排入口说明与相关链接。
- 验收指标：2 分钟内定位导出编排入口。

## 1) 范围说明

services::export 提供导出入口并负责调度管线。

## 2) 关联文档

- Services 概览：`../services.md`
- Export 模块：`../export.md`
