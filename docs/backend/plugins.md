### 3.7 plugins（扩展与钩子）

- 责任边界：扩展机制与钩子协议定义。
- 不负责：核心业务逻辑与持久化实现。
- 目录映射：`crates/plugins/`。
- 交付物：插件注册/钩子协议/沙箱策略。
- 验收指标：插件隔离性、扩展可控性。

## 架构概览

- Hook/Observer：受控扩展环境，统一输入输出与错误策略。
- 三个核心子模块：registry / hook / sandbox。

## 子模块与组成（高层）

- `plugins::registry`：插件注册与生命周期
- `plugins::hook`：解析/渲染/导出钩子协议
- `plugins::sandbox`：资源访问与隔离策略

## 路径索引（实现入口）

- 注册与生命周期：`crates/plugins/src/registry/`
- 钩子协议：`crates/plugins/src/hook/`
- 沙箱与隔离：`crates/plugins/src/sandbox/`
