# 工作流：新增后端命令 (Add Backend Command)

- 责任边界：定义在 KnowLattice 后端新增 Tauri Command 的标准流程。
- 目录映射：`docs/ai/workflows/add-backend-command.md`。

## 步骤详解

1.  **定义 DTO**
    - 位置：`crates/api/src/dto/`
    - 动作：定义请求 (`DtoRequest`) 和响应 (`DtoResponse`) 结构体，确保派生 `Serialize, Deserialize, Type`。

2.  **定义 Command 路由**
    - 位置：`crates/api/src/dispatch/`
    - 动作：在 `CommandRouter` 或相关模块中注册新的命令枚举变体。

3.  **实现 Service 逻辑**
    - 位置：`crates/services/src/`
    - 动作：编写具体的业务逻辑。需遵守 DIP，依赖 Repository Trait 而非具体实现。

4.  **在 Tauri 注册**
    - 位置：`src-tauri/src/commands/`
    - 动作：创建 Tauri command wrapper，并调用 `api` 层的分发逻辑。
    - 注意：Tauri 层仅做透传，不写业务逻辑。

5.  **更新文档**
    - 位置：`docs/backend/api-dto.md`
    - 动作：记录新命令的契约。

6.  **验证**
    - 运行 `cargo check` 和相关测试。
