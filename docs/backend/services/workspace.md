# Services / Workspace

- 责任边界：工作区与文件树编排入口与职责说明。
- 不负责：存储与目录扫描细节（由 storage 与 search 模块承担）。
- 目录映射：`crates/services/src/workspace/`。
- 交付物：工作区编排入口说明与相关链接。
- 验收指标：2 分钟内定位工作区编排入口。

## 1) 范围说明

services::workspace 聚焦工作区入口、文件树构建与配置编排。

## 2) 关联文档

- Services 概览：`../services.md`
- Storage 模块：`../storage.md`
