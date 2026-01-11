## 4. 依赖与层次约束

- 责任边界：依赖约束、数据流与跨模块流程视角。
- 不负责：模块职责细节与实现规范。
- 目录映射：`docs/backend/architecture.md`。
- 交付物：依赖规则、数据流与用户流程拆分。
- 验收指标：依赖关系可验证性、流程可追溯性。

- common 不依赖其他模块
- core 依赖 common
- storage 依赖 core、common
- search 依赖 core、common
- export 依赖 core、common
- plugins 依赖 core、common
- services 依赖 core、storage、search、plugins、common
- api 依赖 services、common
- 禁止 core 依赖 m 上层模块

## 5. 核心数据流

- 导入：文件系统 -> Document/Folder 注册 -> services::index 入队解析
- 解析：search::adapters::markdown 生成 NodeTree，高亮范围由 node_range 派生
- 索引：search::adapters::sqlite 写入 node\_\* 表与 FTS
- 查询：services::search 调用 search::application::SearchQuery/QueryEngine 返回命中/高亮
- 渲染：services::render 将 NodeTree 渲染为 markdown/html/markmap
- 导出：通过 plugins::hook::on_export 产出文件

---

## 6. 用户流程与功能拆分（SRP）

### 7.1 启动加载

- `GetCurrentWorkspace`：读取当前工作区指针（workspace_state）。
- `ListWorkspace`：列出所有工作区。
- `ListWorkspaceFiles`：按工作区展示文件列表/文件树。

### 7.2 打开文件

- `OpenDocument`：读取文档内容。
- `RecordRecentFile`：记录最近打开文件。

### 7.3 渲染展示

- `GetDocumentContent`：返回正文内容。
- `GetDocumentMeta`：返回标题/路径/更新时间等元信息。

### 7.4 模块归属（建议）

- 用例与模块归属以 `docs/backend/services.md` 为准；存储细节见 `docs/backend/storage.md`。

## 7. 扩展点

- 解析前后钩子（自定义语法）
- 渲染前后钩子（主题、节点样式）
- 导出前后钩子（模板、打包）

## 8. 可选演进

- 多用户协作：权限、审计、同步协议
- 云同步：storage 增加远端实现
- 增量更新：search 支持局部重建

---

## 9. 状态建模（以“变化”为单位）

### 9.1 单一变化：Workspace 变化

只关心与 Workspace 相关的变化：

- 当前 workspace 切换
- 最近打开 workspace
- 最近打开文件（可视为 workspace 范围内的行为）

### 9.2 对应的数据结构（建议）

- 具体表结构与持久化细节见 `docs/backend/storage.md`。

### 9.3 设计原则与扩展

- `workspace_state` 只跟 workspace 变化有关。
- `workspace_recent_files` 只跟 workspace 内文件访问变化有关。
- 状态模型属于应用状态层，不进入 core 领域模型；由 services 负责用例编排，storage 负责持久化。
- 不使用 `AppState` 聚合多种变化，避免吞噬窗口/主题/权限等其他维度。
- 后续新增非 workspace 变化时，单独建表：
  - `theme_state`
  - `window_state`
  - `permission_state`
