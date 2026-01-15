# Services / Search

- 责任边界：检索用例编排入口与职责说明。
- 不负责：检索算法与索引实现细节（由 search 模块承担）。
- 目录映射：`crates/services/src/search/`。
- 交付物：检索编排入口说明与相关链接。
- 验收指标：2 分钟内定位检索编排入口。

## 1) 范围说明

services::search 聚焦用例编排与查询入口，具体检索实现位于 `crates/search/`。

## 2) 关联文档

- Services 概览：`../services.md`
- Search 模块：`../search.md`
