# AI 测试指南 (Testing Guide)

- 责任边界：定义 AI 在代码生成过程中必须遵守的测试策略与验证步骤。
- 不负责：具体的测试用例实现。
- 目录映射：`docs/ai/testing-guide.md`。

## 1. 后端测试 (Backend - Rust)

后端拥有成熟的测试基础设施，AI **必须**在修改逻辑后编写或更新测试。

### 1.1 单元测试 (Unit Tests)
- **位置**: 源码文件底部的 `mod tests` 模块，或 `src/` 下的独立测试文件。
- **原则**: 覆盖核心领域逻辑（Core Domain）和纯函数。
- **示例**:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      #[test]
      fn test_domain_logic() { ... }
  }
  ```

### 1.2 集成测试 (Integration Tests)
- **位置**: `crates/<crate_name>/tests/` 目录。
- **原则**: 覆盖 Service 层用例、Database 交互和 Search 流程。
- **命令**: `cargo test -p <crate_name>`

## 2. 前端测试 (Frontend - SolidJS)

**现状**: 项目当前**未配置**自动化测试框架 (Vitest/Jest)。

### 2.1 行为准则
- **禁止幻觉**: 严禁生成无法运行的 `.test.ts` 或 `.spec.ts` 文件。
- **手动验证**: 在输出 Plan 时，必须包含“手动验证步骤（Manual Verification Steps）”。
- **组件预览**: 对于复杂组件，建议生成临时预览代码以便在页面中挂载测试。

## 3. 验证清单 (Verification Checklist)

在提交任务前，必须执行：
1.  [Backend] `cargo check` 确保无编译错误。
2.  [Backend] `cargo test` 确保逻辑正确。
3.  [Frontend] `bun run type-check` (如果有) 或确保 TS 无报错。
