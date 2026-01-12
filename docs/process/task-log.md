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
