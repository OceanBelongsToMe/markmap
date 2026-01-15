# Services / Document

- 责任边界：文档用例编排入口与职责说明。
- 不负责：渲染与检索实现细节（由 render/search 模块承担）。
- 目录映射：`crates/services/src/document/`。
- 交付物：文档编排入口说明与相关链接。
- 验收指标：2 分钟内定位文档编排入口。

## 1) 范围说明

services::document 提供文档级用例入口，与 render/search 协作完成具体处理。

## 2) 关联文档

- Services 概览：`../services.md`
- Render 模块：`render.md`
