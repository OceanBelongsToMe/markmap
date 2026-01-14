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
- `payload.heading_level`：仅 heading 节点填入 `1-6`，其它节点为 `null/缺省`（前端用于图标/样式）。
- `initialExpandLevel` 通过 `MarkmapOptions` 进入（默认 -1）。
- 表格节点 `content` 输出 HTML `<table>` 片段（与 RenderHtml 一致）。

前端约定：

- 不再使用 `setData/_initializeData`。
- 只写入 `mm.state.data` 并调用 `renderData()`。
- 不依赖 `markmap-common` 的运行期工具。

## 2.5 Markmap 懒加载扩展（协议）

- 责任边界：懒加载所需字段与模式定义。
- 不负责：加载策略、缓存实现、渲染触发时机。
- 目录映射：`docs/shared/markmap-protocol.md`、`docs/shared/config-scopes.md`。
- 交付物：字段约定、模式定义、前后端契约说明。
- 验收指标：懒加载前后端行为一致、字段语义不冲突。

字段扩展（payload）：

- `payload.has_children: boolean`：节点是否存在子节点（用于展示折叠圆）。
- `payload.children_loaded: boolean`：子节点是否已加载（用于避免重复加载）。
- `payload.children_count?: number`：可选，子节点数量提示（用于 UX 或调试）。
- `payload.show_children_indicator: boolean`：折叠圆的“实心/空心”状态，由后端统一计算。

模式约定（配置）：

- namespace: `markmap`
- key: `load_mode.root`
- value: `"full" | "lazy" | "outline"`
- key: `load_mode.child`
- value: `"full" | "lazy" | "outline"`

前端行为约定：

- `has_children = true` 且 `children_loaded = false` 时可触发加载。
- 加载完成后，前端必须将 `children_loaded` 置为 `true`。
- 未加载子节点时，前端不应假设 `children` 已完整。
- `show_children_indicator` 由后端计算，前端只读，不得推导或兜底。

Outline 模式约定：

- 仅输出 heading 节点组成的大纲树（非 heading 节点不出现在输出中）。
- `children_loaded = false`（只输出 heading 子树，完整子树需按 child 策略加载）。
- 仍保留 `has_children/children_count` 以标示大纲层级是否有子标题。
- 叶子 heading 规则：若原始子树存在但无子 heading，则视为“折叠未加载”，圆点实心。
- 非叶子 heading 规则：已有子 heading 展示，圆点空心。

混合策略约定（root/child 分离）：

- 根渲染使用 `load_mode.root`，子树加载使用 `load_mode.child`。
- 组合示例：`root=outline` + `child=lazy`（默认）。
- `child=outline` 时仅保留子树内 heading。
