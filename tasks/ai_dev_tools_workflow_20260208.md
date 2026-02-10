# AI Dev Tools Workflow

## 工具配额与成本

| 工具 | 模型 | 成本 | 配额 | 使用场景 |
|------|------|------|------|---------|
| **GitHub Copilot CLI** | Claude Sonnet 4.5 | 1 request | 300/月 | 简单代码任务 |
| | Claude Opus 4.6 | 3 requests | ~100/月 | 复杂代码任务 |
| **Claude Code** | Claude Opus | 不固定 | - | Planning / Review |
| **Gemini CLI** | Gemini 3 Pro | 免费 | 无限 | Q&A / 研究 / 总结 |

## 核心原则

1. **Planning** → Claude Code Opus (高质量)
2. **Review** → Claude Code Opus (严格审核)
3. **简单执行** → Copilot CLI Sonnet 4.5 (省配额)
4. **研究/Q&A** → Gemini CLI (免费快速)

## 任务分流策略

```
┌─────────────────────────────────────────────────────────────┐
│                     任务类型判断                             │
└─────────────────────────────────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
      ┌──────────┐   ┌───────────┐   ┌──────────────┐
      │ Planning │   │ Execution │   │ Research/Q&A │
      └────┬─────┘   └─────┬─────┘   └──────┬───────┘
           │              │                 │
           ▼              ▼                 ▼
    ┌───────────┐  ┌──────────────┐   ┌──────────────┐
    │ Claude    │  │ 复杂度判断   │   │ Gemini CLI   │
    │ Code Opus │  └──────┬─────┘   │ (免费无限)   │
    └───────────┘         │          └──────────────┘
                         │
            ┌────────────┴────────────┐
            ▼                         ▼
      ┌─────────────┐           ┌──────────────┐
      │ Simple Task │           │ Complex Task│
      │ (Sonnet 4.5)│           │ (Opus 4.6)  │
      │ 1 request   │           │ 3 requests  │
      └─────────────┘           └──────────────┘
```

## 任务复杂度定义

### Simple Task (Copilot CLI Sonnet 4.5)
- 单文件修改
- 明确、简单的重构
- 修复已知 bug
- 添加简单功能
- 文件搜索/替换

### Complex Task (Copilot CLI Opus 4.6 / Claude Code)
- 跨多文件改动
- 新功能实现
- 复杂重构
- 需要理解多个模块
- 架构设计

### Research/Q&A (Gemini CLI)
- 代码解释
- 技术方案调研
- 文档生成
- 快速问答
- 代码总结

## 每日配额管理

```bash
# 每月300 requests = 每天约10 requests

# 建议分配 (每天):
- Simple tasks (Sonnet): 7 requests (7 x 1)
- Complex tasks (Opus): 2 requests (2 x 3 = 6)
- Buffer: 2 requests

# 快速检查剩余配额
alias copilot-quota='gh copilot status'
```

## Workflow 模板

### 场景1: 添加新功能

```bash
# Step 1: Planning (Claude Code)
codex "Plan implementation for [feature]. 
Context: [project path]
Output: Implementation steps, files to modify"

# Step 2: Execution (Copilot CLI)
copilot "Implement [feature] based on the plan.
Steps: [from planning output]
Verify: [test command]"

# Step 3: Review (Claude Code)
codex "Review the changes for [feature].
Files: [list files]
Check: Code quality, edge cases, tests"
```

### 场景2: 修复 Bug

```bash
# Step 1: 研究 (Gemini CLI)
gemini "Explain the bug: [error message]
Context: [relevant code files]
Output: Root cause analysis"

# Step 2: 修复 (Copilot CLI Sonnet)
copilot "Fix the bug at [file:line]
Root cause: [from Gemini analysis]
Verify: [test command]"
```

### 场景3: 代码重构

```bash
# Step 1: Planning (Claude Code)
codex "Plan refactor for [module]
Goal: [what to achieve]
Files: [affected files]"

# Step 2: Execution (根据复杂度)
# Simple: 
copilot "Refactor [file] according to plan"

# Complex:
copilot --model opus "Refactor [module] across [files]"
```

### 场景4: 快速问答

```bash
# 直接用 Gemini CLI (免费)
gemini "How do I [specific question]?"
gemini "Summarize the code in [file]"
gemini "What does [function] do?"
```

## 使用示例

```bash
# 1. 新角色实现 (Complex)
cd /mnt/e/Moltbot/workspace/genshin_artifact
codex "Plan implementation for Escoffier. Output: files, steps"

# 2. 执行
copilot --model opus "Implement Escoffier according to the plan"

# 3. 审核
codex "Review Escoffier implementation. Check: compilation, patterns"

# 4. 验证
cargo build && cargo test


# 1. 查找代码 (Simple)
copilot "Find all uses of CharacterStaticData in the codebase"

# 2. 解释代码 (Gemini)
gemini "Explain the ArtifactDisplay component and its data flow"

# 3. 小修改 (Simple)
copilot "Update ArtifactDisplay to use local images instead of CDN"
```

## Claude Code 特殊用法

### 只用于 Planning 和 Review
```bash
# Planning
codex exec "Plan: Implement character X
Output: markdown with files, steps"

# Review  
codex "Review [file]. Issues?"
```

### 不用 Claude Code 执行复杂任务
- ❌ 不让 Claude Code 直接写代码
- ❌ 不让 Claude Code 执行多步任务
- ✅ 只用于思考和审核

## 成本监控

```bash
# GitHub Copilot 剩余请求数
gh copilot usage

# Claude Code 配额 (官网查看)
# Gemini CLI 免费，无限制
```

## 优先级规则

1. **简单任务** → Copilot Sonnet (省配额)
2. **需要思考** → Claude Code (高质量)
3. **查资料** → Gemini (免费)
4. **复杂执行** → Copilot Opus (必要时)

## 快速参考表

| 场景 | 工具 | 命令 | Request |
|------|------|------|---------|
| 规划新功能 | Claude Code | `codex "Plan..."` | 0 (planning) |
| 单文件修改 | Copilot | `copilot "Fix..."` | 1 |
| 跨文件重构 | Copilot Opus | `copilot --model opus "Refactor..."` | 3 |
| 代码审核 | Claude Code | `codex "Review..."` | 0 (review) |
| 解释代码 | Gemini | `gemini "Explain..."` | 0 |
| 查资料 | Gemini | `gemini "Research..."` | 0 |
| 写测试 | Copilot | `copilot "Write tests for..."` | 1-3 |

## 常见问题

**Q: 什么时候用 Claude Code 而不是 Copilot?**
A: 需要深入理解代码库、复杂的逻辑判断、多方案选择时。

**Q: Gemini CLI 有什么限制?**
A: 目前 Gemini 3 Pro 对话长度有限制，长代码可能需要分段。

**Q: Copilot CLI 失败怎么办?**
A: 尝试简化任务描述，或降级为更小的任务。

**Q: Claude Code quota 不稳定怎么办?**
A: 减少使用频率，只用于关键决策点。
```

```markdown
## 工具对比速查

| 特性 | Claude Code | GitHub Copilot CLI | Gemini CLI |
|------|-------------|-------------------|------------|
| **核心优势** | 深度思考、质量 | 代码执行 | 免费快速 |
| **订阅成本** | 不固定 | 300 req/月 | 免费 |
| **适用场景** | Planning/Review | 代码执行 | 研究/Q&A |
| **多轮对话** | ✅ 强 | ✅ 中等 | ✅ 基础 |
| **代码质量** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **执行速度** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **上下文理解** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
```

## 实战场景

### 场景: 实现新圣遗物效果

```bash
# 1. Research (Gemini - 免费)
gemini "Explain the 'Lunar-Charged' reaction mechanics in Genshin Impact"

# 2. Planning (Claude Code - 质量优先)
codex "Plan implementation for Lunar-Charged reaction
Output: 
- Files to modify
- Data structures needed
- Integration steps"

# 3. Implementation (Copilot - 执行)
copilot --model opus "Implement Lunar-Charged reaction following the plan"

# 4. Review (Claude Code - 质量把关)
codex "Review Lunar-Charged implementation
Check:
- Edge cases
- Performance
- Code patterns"
```

### 场景: 修复 UI 显示问题

```bash
# 1. 定位问题 (Copilot - 简单搜索)
copilot "Find where artifact images are loaded in the Vue components"

# 2. 分析根因 (Gemini - 快速理解)
gemini "Analyze the CDN URL resolution in ArtifactDisplay.vue"

# 3. 修复 (Copilot Sonnet - 简单修复)
copilot "Replace CDN URLs with local image paths in ArtifactDisplay.vue"

# 4. 验证 (手动)
npm run serve
```

## 总结

**核心策略：**

1. **Claude Code** = 思考工具 (Planning + Review)
2. **Copilot CLI** = 执行工具 (代码生成)
3. **Gemini CLI** = 辅助工具 (研究 + 问答)

**成本优化：**
- 简单任务用 Sonnet (1 req)
- 复杂任务才用 Opus (3 req)
- 研究和问答用 Gemini (免费)

**质量保证：**
- 重要改动必须 Review
- Planning 要详细
- 执行后要验证
