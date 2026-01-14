## 2.5 配置表设计（方案 A：K/V）

- 责任边界：配置结构、命名空间与覆盖优先级约定。
- 不负责：具体存储实现与界面交互。
- 目录映射：`docs/shared/config-scopes.md`、`crates/storage/`、`crates/services/src/config/`。
- 交付物：配置表结构与读取策略。
- 验收指标：配置解析一致性、覆盖规则可预测性。

适用：配置项频繁变动、前后端共享、需要灵活扩展。

表：`user_settings`

- `id` TEXT PRIMARY KEY
- `user_id` TEXT NULL      // NULL 表示全局默认
- `scope` TEXT NOT NULL     // "global" | "workspace" | "document"
- `scope_id` TEXT NULL      // workspace_id / document_id
- `namespace` TEXT NOT NULL // e.g. "markmap"
- `key` TEXT NOT NULL       // e.g. "initial_expand_level"
- `value_json` TEXT NOT NULL // JSON 序列化
- `updated_at` INTEGER NOT NULL // 毫秒时间戳

约束与索引：

- UNIQUE(`user_id`, `scope`, `scope_id`, `namespace`, `key`)
- INDEX(`namespace`, `scope`, `scope_id`)

读取优先级：

1) document 级覆盖
2) workspace 级覆盖
3) user global
4) system default

## 2.6 MarkmapOptions 读取设计（基于 user_settings）

命名空间与键：

- namespace: `markmap`
- key: `initial_expand_level`
- key: `load_mode.root`  // "full" | "lazy" | "outline"
- key: `load_mode.child` // "full" | "lazy" | "outline"

scope 约定：

- `document`：scope_id = document_id
- `workspace`：scope_id = workspace_id
- `global`：scope_id = NULL

读取流程（高优先级优先命中即返回）：

```
defaults = MarkmapOptions::default()
scopes = [
  ("document", document_id),
  ("workspace", workspace_id),
  ("global", None)
]
for scope in scopes:
  setting = repo.get(user_id, scope, namespace, key)
  if setting exists:
    options.initial_expand_level = parse(setting.value_json)
    break
return options
```

失败处理：

- JSON 解析失败：记录 error，回退默认值
- 设置不存在：继续向下一级
