# 信息架构

- 责任边界：导航体系、信息层级、命名规则、内容分组。
- 不负责：视觉表现、动效细节、业务流程逻辑。
- 目录映射：`src/ia/`
- 交付物：站点地图、导航结构、信息分组与命名规范。
- 验收指标：任务查找时间、信息定位成功率、导航路径清晰度。
- 说明：信息架构以常量定义为主，作为导航结构与层级模型的基准配置。
  - `src/ia/nav-model.ts`：导航结构与层级模型常量
  - `src/ia/taxonomy.ts`：内容分组与命名规范常量
  - `src/ia/labels.ts`：界面文案与命名一致性字典
  - 说明：导航项使用 `labelKey`，渲染时通过 `t(labelKey, locale)` 获取对应文案。
  - taxonomy 约定：
    - `contentTypes`：业务内容类型（`note`、`outline`、`markmap`）
    - `editorInputs`：输入方式（`markdown-text`、`wysiwyg`、`markmap`）
    - `viewModes`：视图开关（`markmap-preview`），仅作用于非 `markmap` 输入方式
