# 命名规范（跨端一致性）

- 责任边界：跨前后端的命名与约束规则。
- 不负责：业务流程与实现细节。
- 目录映射：`docs/shared/naming-conventions.md`、`src/ia/`、`crates/common/src/types/`。
- 交付物：统一命名规则与约束清单。
- 验收指标：命名一致性、可检索性、跨端可读性。

## 1) 信息架构命名（前端）

来源：`src/ia/taxonomy.ts`、`src/ia/labels.ts`。

### 1.1 taxonomy（业务类型与模式）

- `contentTypes`：`note` | `outline` | `markmap`
- `editorInputs`：`markdown-text` | `wysiwyg` | `markmap`
- `viewModes`：`markmap-preview`
- `tags.system`：`important` | `todo` | `starred`
- `tags.domain`：`product` | `tech` | `design`

### 1.2 labels（UI 文案键）

- 命名采用 `camelCase`，语义清晰、可翻译：
  - 示例：`language`、`workspace`、`floatingEditorPanelTitle`
- 强制使用 `labelKey` 访问，不直接硬编码字符串。

## 2) 标识与路径规范（后端通用）

来源：`crates/common/src/types/normalize.rs`、`crates/common/src/uuid.rs`。

### 2.1 Relative Path

- 必须是相对路径，不允许：
  - 绝对路径（以 `/` 或 `//` 开头）
  - Windows 盘符（如 `C:`）
  - `..` 路径穿越
- 规则：
  - 统一分隔符为 `/`
  - 去除空段与 `.`

### 2.2 Tag

- 统一小写
- 空白折叠为单个空格
- 长度上限：64
- 仅允许：字母数字、`-`、`_`、空格

### 2.3 UUID

- 解析：`parse_uuid_str` 负责字符串解析与错误包装
- 存储：16 字节 BLOB（`uuid_to_blob` / `blob_to_uuid`）

## 3) 配置命名（跨端）

- 命名建议：统一 `snake_case` 作为 key（例如 `initial_expand_level`）
- 命名空间：使用 `namespace` 区分领域（例如 `markmap`）
- 具体配置覆盖规则见 `docs/shared/config-scopes.md`
