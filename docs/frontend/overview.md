# 前端 UX/UI 模块化设计概览

- 责任边界：前端文档导航与总体说明。
- 不负责：各模块实现细节。
- 目录映射：`docs/frontend/`。
- 交付物：入口索引与高层结构说明。
- 验收指标：可发现性与导航完整性。

## 目标

将 UX/UI 设计按单一职责原则拆分为可独立交付与验收的模块，并映射到前端目录结构，保证设计资产、实现代码与验收指标一致。

## 模块总览

- 信息架构
- 任务流与交互
- 视觉与布局
- 组件与模式
- 可用性与无障碍
- 度量与验证
- 国际化（I18n）
- 状态管理（State）

---

## 前端目录规划建议

- `src/a11y/`：无障碍策略与工具
- `src/features/`：业务功能与交互流程（sidebar, workspace）
- `src/i18n/`：国际化资源与策略
- `src/ia/`：信息架构常量
- `src/layouts/`：布局骨架与规则
- `src/pages/`：页面级入口
- `src/routes/`：路由定义
- `src/state/`：全局与共享状态
- `src/ui/ark/`：Ark UI 行为层实现
- `src/ui/components/`：通用 UI 组件
- `src/ui/icons/`：图标资产
- `src/ui/patterns/`：组合式交互模式
- `src/ui/styles/`：组件样式与全局 CSS
- `src/ui/theme/`：主题 Tokens 与变量
- `src/ui/typography/`：排版系统
- `src/ui/virtual/`：虚拟滚动实现
