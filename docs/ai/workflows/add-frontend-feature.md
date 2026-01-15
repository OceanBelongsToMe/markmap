# 工作流：新增前端业务组件 (Add Frontend Feature)

- 责任边界：定义在 KnowLattice 前端新增业务功能模块的标准流程。
- 目录映射：`docs/ai/workflows/add-frontend-feature.md`。

## 步骤详解

1.  **明确位置**
    - 决策：确认属于 `features/` (业务域) 还是 `ui/patterns/` (通用模式)。
    - 参考：`docs/frontend/overview.md`。

2.  **检查复用**
    - 动作：查阅 `src/ui/components/`，优先复用现有基础组件（如 Button, Select）。
    - 约束：严禁重复造轮子。

3.  **状态设计**
    - 决策：
        - 局部状态 -> 使用 `createSignal`。
        - 全局/共享状态 -> 放入 `src/state/`。
    - 规范：参考 `docs/ai/rules-frontend.md` (SolidJS Patterns)。

4.  **实现与样式**
    - 规范：使用 CSS Modules 或 `data-*` 属性绑定样式。
    - 约束：严禁直接修改 `@ark-ui` 内部样式，必须通过封装组件接口。

5.  **更新文档**
    - 位置：`docs/frontend/features.md`
    - 动作：更新功能入口索引。

6.  **验证**
    - 动作：进行手动验证，并在 PR/Reply 中列出验证步骤。
