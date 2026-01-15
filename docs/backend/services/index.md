# Services / Index

- 责任边界：索引编排用例入口与职责说明。
- 不负责：解析/检索算法细节（由 search 模块承担）。
- 目录映射：`crates/services/src/index/`。
- 交付物：索引编排入口说明与相关链接。
- 验收指标：2 分钟内理解索引编排入口。

## 1) 范围说明

services::index 负责索引编排与任务调度，解析与索引构建由 search 模块实现。

## 2) 关联文档

- Services 概览：`../services.md`
- Search 模块：`../search.md`
