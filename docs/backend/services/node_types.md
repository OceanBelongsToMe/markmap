# Services / Node Types

- 责任边界：节点类型初始化与映射入口说明。
- 不负责：领域模型定义与存储细节（由 core/storage 承担）。
- 目录映射：`crates/services/src/node_types/`。
- 交付物：节点类型入口说明与相关链接。
- 验收指标：2 分钟内定位节点类型入口。

## 1) 范围说明

services::node_types 负责 NodeType 相关初始化与映射入口。

## 2) 关联文档

- Services 概览：`../services.md`
- Core 模块：`../core-domain.md`
