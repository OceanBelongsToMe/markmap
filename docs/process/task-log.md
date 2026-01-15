# 任务记录

## NodeType 内容时间戳
- [ ] 说明：为 NodeType 增加内容层时间戳，用于区分语义变更与解析时间。
- [ ] 原因：Node 的 updated_at 反映解析时间，内容变更应单独追踪。
- [ ] 方案：引入 NodeTypeMeta（组合方式），挂载到各个 NodeType 变体。
- [ ] 状态：等待确定结构调整方案。

## TaskListMarker 与 Task 节点同步策略
- [ ] 需要确认：TaskListMarker 出现时是否“覆盖 ListItem 为 Task”，还是“保留 ListItem + 追加 Task 记录”。
- [ ] 当前问题：仅更新 NodeBase.node_type_id，未同步 NodeType 流，导致 node_task 不入库。

## core 领域模型构造器/工厂（3.1）
- [x] 为 Workspace/Folder/Document/Node/NodeType 增加构造器，集中校验不变量（纯模型无 IO）。
- [x] 为 RelativePath/Tag 复用 common::types normalizer，避免重复逻辑。

## services 层构造器使用点预留
- [x] 在 services 创建/更新实体的流程中统一使用 core::model::*::new 构造器（禁止直接 struct literal）。

## 全局配置改为 JSON 文件
- [ ] 讨论并实现全局配置使用 JSON 文件存储（替代数据库方案）。

## 时间字段存储格式
- [ ] SQLite 中时间字段采用 INTEGER（Unix 时间戳），读写时统一转换为 ISO-8601。

## HTML 导出/分享显式净化开关
- [ ] 说明：导出/分享路径增加显式 Sanitizer 开关，不依赖环境变量。
- [ ] 原因：避免 raw HTML 在导出场景误用，安全策略由调用方控制。
- [ ] 依赖：完成 “Inline HTML 规则规范化”。

## Inline HTML 规则规范化
- [ ] 说明：明确 Math inline / FootnoteReference / Wiki 的 HTML 输出规范。
- [ ] 原因：避免渲染规则不一致导致预览差异。

## Markdown 渲染模块 SRP + DIP 重构
- [ ] 说明：按阶段迁移 markdown 渲染目录结构（traits / source / classify / tree / inline / serializer）。
- [ ] 迁移阶段：0) 新建目录与 traits 占位；1) service 依赖 traits；2) inline 内核下沉；3) serializer profile/policy 拆分；4) tree/classify/source 独立；5) 清理旧路径。
- [ ] 验收：每阶段测试全绿，渲染行为不变。

## Markmap 渲染模块 SRP + DIP 重构
- [ ] 说明：按阶段迁移 markmap 渲染目录结构（traits / pipeline / inline / classify / source / config）。
- [ ] 迁移阶段：1) 增加 traits 端口；2) pipeline 依赖 traits（transform/init/fold）；3) 增加 inline/classify/source/config 适配层；4) service 纯编排；5) 清理旧依赖与路径。
- [ ] 验收：每阶段测试全绿，输出一致。

## Markmap Outline 加载模式
- [x] 说明：新增仅输出 heading 的大纲模式（outline），补齐协议字段并由后端负责指示器状态。

## Markmap 混合加载策略（root/child 分离）
- [x] 说明：支持 root/child 两级加载策略（root=outline + child=lazy 默认），并更新配置键与协议。

## Markmap 子节点数量圆点显示
- [x] 说明：在折叠圆中显示 `payload.children_count`（实心/空心一致）。

## Markmap ListItem 文本渲染
- [x] 说明：inline 遍历需展开 Paragraph 作为容器，避免 ListItem 为空。

## Markmap ListItem Paragraph 承载
- [x] 说明：ListItem 文本由 Paragraph 承载时，由 markmap inline 适配层展开。

## Recent 最近打开接入
- [ ] 说明：Recent 与 Files 平级，扁平列表，触底分页加载；跨设备同步列为待办。
  - 回归：选择 Files 内任一非首位文件 → Recent 立即置顶；重复点击同一文件不再发 `workspace_recent_file_record`；滚动条出现时折叠箭头不被遮挡。

## Recent 滚动触底 createRoot 警告
- [ ] 说明：滚动触底时出现 “computations created outside a createRoot or render”，需定位调用栈并确认是否仍会复现；当前使用 runWithOwner 包装但未做根因确认。
- [ ] 状态：挂起，待复现与堆栈信息。

## 界面布局与交互优化 (2026-01-15)
- [x] 说明：优化 Sash 拖拽体验，放宽布局尺寸限制，调整默认视图状态。
- [x] Sash 修复：使用 offset 修正点击坐标，解决跳变问题。
- [x] 约束放宽：Editor min 200px / no max; Preview max 1500px。
- [x] 默认状态：默认单栏，悬浮面板按需渲染（消除空容器背景）。
- [x] 侧边栏交互：在工具栏增加侧边栏切换按钮，支持“完全隐藏 + 边缘悬停弹出”模式。
- [x] 架构重构：将 `usePaneSizes` 改造为基于 Key 的管理模式，实现 Pane 动态增删时的状态持久化与解耦。
- [ ] 侧边栏/分栏宽度持久化：将用户拖拽后的 pane sizes 存入 user_settings 表，实现重启后恢复布局。
