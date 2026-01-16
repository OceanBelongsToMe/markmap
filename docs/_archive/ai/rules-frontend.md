# AI 前端开发指南 (Frontend Guide for AI)

- 责任边界：定义 AI 在进行前端开发时必须遵守的 SolidJS 特有模式、Ark UI 使用规范及状态管理约束。
- 不负责：具体的 UI 业务逻辑（见 `docs/frontend/features.md`）。
- 目录映射：`docs/ai/rules-frontend.md`。

## 1. SolidJS 核心模式 (SolidJS Core Patterns)

### 1.1 响应式原语 (Primitives)
- **Signals**: 用于局部状态。
  ```tsx
  const [count, setCount] = createSignal(0);
  ```
- **Effects**: 仅用于副作用（同步 DOM、写 Storage）。**严禁**在 effect 中写入信号导致无限循环。
  ```tsx
  createEffect(() => {
    console.log(count()); // 追踪依赖
  });
  ```
- **Derived State**: 尽量使用派生状态（函数），而非 `createEffect` + `setSignal`。
  ```tsx
  // ✅ Good
  const double = () => count() * 2;
  
  // ❌ Bad
  const [double, setDouble] = createSignal(0);
  createEffect(() => setDouble(count() * 2));
  ```

### 1.2 控制流组件 (Control Flow)
- 严禁使用 `Array.map` 渲染列表，必须使用 `<For>` 或 `<Index>`。
- **`<For>`**: 用于 ID 稳定的对象列表（DOM 移动，不销毁）。
- **`<Index>`**: 用于原始值列表或索引敏感场景（DOM 不移动，仅内容更新）。

## 2. Ark UI 使用规范 (Ark UI Usage)

### 2.1 封装原则
- **禁止裸用**：严禁在 `src/features/` 中直接引入 `@ark-ui/solid`。
- **必须封装**：所有 Ark 组件必须先在 `src/ui/ark/` 封装，再在 `src/ui/components/` 暴露接口。

### 2.2 样式绑定
- 使用 `data-*` 属性进行样式绑定，禁止操作 `style` 属性。
  ```css
  /* ✅ Good */
  &[data-state="open"] { ... }
  ```

## 3. 状态管理 (State Management)
- **全局状态**：查找 `src/state/`。
- **URL 状态**：优先将可分享状态（如 filter, sort）同步到 URL Search Params。

## 4. 常见陷阱 (Common Pitfalls)
- **解构丢失响应性**：严禁解构 `props`。
  ```tsx
  // ❌ Bad
  const { title } = props;
  
  // ✅ Good
  <div>{props.title}</div>
  ```
