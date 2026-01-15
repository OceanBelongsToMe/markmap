# 架构摘要

- 责任边界：系统全貌与关键路径摘要。
- 不负责：模块内部实现细节。
- 目录映射：`docs/architecture-summary.md`。
- 交付物：架构概览与关键入口索引。
- 验收指标：5 分钟可理解系统全貌。

## 1) 项目定位

基于 Tauri + Rust 的 Markdown/Markmap 编辑与检索系统，强调可扩展后端与清晰职责边界。

## 2) 关键组件拓扑

- Frontend：界面与交互（`src/`）
- Tauri 入口：运行期装配与命令分发（`src-tauri/src/`）
- Services：业务用例编排（`crates/services/`）
- Storage：持久化与迁移（`crates/storage/`）
- Search：解析/索引/检索（`crates/search/`）
- API/DTO：命令与契约边界（`crates/api/`）
- Plugins：扩展与钩子（`crates/plugins/`）
- Shared：跨端契约（`docs/shared/*`）

## 3) 关键数据流

导入 → 解析 → 索引 → 查询 → 渲染 → 导出

- 导入：文件系统注册 → services::index 入队
- 解析：search::adapters 生成 NodeTree
- 索引：sqlite 写入 node_* 与 FTS
- 查询：services::search 触发 SearchQuery
- 渲染：services::render 生成 markdown/html/markmap
- 导出：plugins::hook::on_export 产出文件

## 4) SRP 边界摘要

- core：领域模型与不变量
- services：业务用例编排
- storage：持久化与迁移
- search：解析与检索能力
- api：命令/DTO 边界
- plugins：扩展钩子

## 5) 入口索引

- 延伸入口：[index.md](index.md)
