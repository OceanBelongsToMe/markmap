## 2.4 Markmap 渲染的 SRP 设计（后端）

- 责任边界：Markmap 输出协议与结构约定。
- 不负责：渲染实现细节与调用时机。
- 目录映射：`docs/shared/markmap-protocol.md`、`crates/services/src/render/markmap/`。
- 交付物：协议结构与字段约定。
- 验收指标：前后端契约一致性、兼容性。

目标：将 markmap 的初始化逻辑迁移到后端，同时保持单一职责，避免 transformer 过载。

职责拆分建议（落地版）：

- `pipeline/transformer.rs`：仅负责结构转换（`NodeTree -> PureNode`），不写 state/payload。
- `pipeline/initializer.rs`：`PureNode -> INode`，补齐 `state/payload`（id/path/key/depth/rect/size）。
- `pipeline/folder.rs`：折叠策略（fold=2 递归折叠 + initialExpandLevel）。
- `config/options.rs`：`MarkmapOptions`（默认值 + 用户配置入口）。
- `types.rs`：后端输出的 INode 结构定义。

推荐流水线（实现管线）：

```
RenderMarkmap::execute
  -> MarkmapTransforming::transform (PureNode)
  -> MarkmapInitializing::initialize (INode)
  -> MarkmapFolding::apply (INode, options)
  -> JSON 输出
```

关键约定：

- `state.id` 为遍历自增数字（满足前端 markmap-view 运行期索引需求）。
- `state.key` 使用后端 `node_id`（UUID 字符串），保证稳定性。
- `state.path` 为数字路径（例如 `1.2.3`）。
- `payload.path` 与 `state.path` 保持一致。
- `initialExpandLevel` 通过 `MarkmapOptions` 进入（默认 -1）。

前端约定：

- 不再使用 `setData/_initializeData`。
- 只写入 `mm.state.data` 并调用 `renderData()`。
- 不依赖 `markmap-common` 的运行期工具。
