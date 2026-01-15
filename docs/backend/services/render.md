# Render 设计

- 责任边界：render 模块职责与输出边界；规范 Markdown/HTML/Markmap 渲染的 SRP 拆分与复用策略。
- 不负责：渲染实现细节与具体算法（详见代码目录索引）。
- 目录映射：`docs/backend/services/render.md`。
- 交付物：render 模块职责定义、SRP 拆分说明、关联文档链接。
- 验收指标：可在 2 分钟内理解 render 模块职责与边界。

## 1) 模块职责

- 将 NodeTree 渲染为 Markdown / HTML / Markmap 的输出结果。
- 统一提供文档级渲染入口（format 路由）。

## 2) SRP 拆分与复用约束

- RenderDocument：仅负责按 format 路由到具体渲染用例。
- RenderHtml：仅负责编排（数据加载 → Markdown→HTML → 后处理 → 可选净化），不包含渲染规则。
- Inline 渲染策略：RenderMarkmap 使用自研 inline 渲染并按 SRP 拆分（文本提取 / 规则格式化 / 遍历引擎 / 渲染编排），以保持性能与规则可控。
- Inline 遍历规则：Markmap 下 ListItem 若由 Paragraph 承载文本，应展开 Paragraph 以产出节点内容。
- Markdown 渲染分层：traits + source + classify + tree + inline + serializer，职责单一且依赖倒置。
- Markmap 渲染分层：traits + pipeline + inline + classify + source + config，职责单一且依赖倒置。
- RenderMarkmap 入口纯化：service 只编排，registry 负责编排装配，模块根仅导出/注册。
- 可选净化开关：通过环境变量 `KNOWLATTICE_RENDER_HTML_SANITIZE` 启用 HTML 净化。
- 待办事项：导出/分享路径显式净化开关见 `../../process/task-log.md`。

## 3) 输出与共享契约

- Markmap 输出协议见 `../shared/markmap-protocol.md`。
  - 懒加载字段与模式约定同上，避免在服务文档重复描述。

## 4) 代码目录索引

- 渲染实现与规则细节见 `../../../crates/services/src/render/`。

## 5) 关联文档

- Services 设计：`services.md`
- 后端架构与数据流：`architecture.md`
- Render 入口索引：`services/render.md`
