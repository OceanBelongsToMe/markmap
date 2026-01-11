### 3.3 storage（持久化接口与实现）

- 责任边界：仓储接口与持久化实现。
- 不负责：业务规则与用例编排。
- 目录映射：`crates/storage/`。
- 交付物：Repository/Mapper、迁移与存储适配。
- 验收指标：数据一致性、迁移可重复性。

## 设计目标

- 提供 Repository trait + 具体实现（fs/sqlite）。
- 仅处理持久化与序列化，不承载业务规则。

## 子模块与组成（高层）

- `storage::repo`：仓储接口（建议 async）
- `storage::mapper`：Record ↔ Domain 映射
- `storage::fs`：文件系统读写（Markdown 原文）
- `storage::sqlite`：SQLite 实现与连接管理
- `storage::migrate`：迁移与版本管理（`crates/storage/migrations/`）

## 可见性与错误边界

- 对外仅暴露 `storage::repo` 与 `storage::factory`。
- `storage::mapper`、`storage::fs`、`storage::sqlite` 仅 crate 内可见。
- `storage::error::map_sqlx_error` 统一将 sqlx 错误映射为 AppError。

## 路径索引（实现入口）

- 仓储接口：`crates/storage/src/repo/`
- SQLite 实现：`crates/storage/src/sqlite/`
- 映射层：`crates/storage/src/mapper/`
- 迁移脚本：`crates/storage/migrations/`

## 少量示例（方便快速理解）

- Repository：`DocumentRepository::get/save/delete/list_by_folder`
- Data Mapper：`crates/storage/src/sqlite` 中的手写 SQL ↔ 领域对象映射
- 迁移执行：由 `ServicesBuilder` 装配时触发（见 `crates/services/src/builder.rs`）
