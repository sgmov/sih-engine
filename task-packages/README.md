# task-packages/ — sih-engine 治理任务包目录

> 范式与命名约定（2026-08-17 立）

## 范式：T6-D 双子代理 + 主线串行验证

**T6-D 是工作流范式**（Twin-agent 双子代理 + Mainline 主线串行验证 + Dependent 依赖收敛），不是任务编号。

来源：用户记忆 + ai-ex/T6-PARADIGMS.md（双子代理 + 主线编排范式谱系）。

### 范式流程

```
1. 主线立任务包（含 F 锚定 + 约束 + 真源）
2. 主线派双子代理（run_in_background=true）：
   - X1 写 A1+A2（独立 + 互不依赖）
   - X2 写 A3+A4（独立 + 互不依赖）
3. 主线写 A5（最复杂，需完整上下文）
4. 主线验收：跑 baseline 验 F 锚定
5. 主线写 Cluster 2（依赖 Cluster 1）
6. 主线 commit 全部
```

### 范式 vs 任务编号

- **T6-D** = 范式名
- **T6D-XX** = ❌ 错（把范式当编号）
- **mechanism-scripts-t6d** = ✅ 范式作为后缀，主题作为前缀

## 命名约定

格式：`<主题>-<范式>.md`

| 范式 | 后缀 | 说明 |
|---|---|---|
| T6-D 双子代理 + 主线验证 | `-t6d` | 当前用的范式 |
| T7-X 单子代理 + 主线验证 | `-t7x` | 未来范式 |
| T8-S 顺序子代理（依赖链） | `-t8s` | 未来范式 |

主题命名：
- 主题在前（具体做什么）
- 范式在后（怎么编排）
- 全部小写 + `-` 分隔

## 请求写入段

任务包可带「请求写入」段即二级标题含请求写入字样，下列表项为工作区根相对路径或目录，条目精确或目录前缀即会话写入范围。工地外壳开工时机械读取该段为会话档 allow 第一源，两把锁的范围验以此为准，2026-08-25 用户裁定即 worktree 和两把锁要和任务包匹配。无该段即范围空由调用方显式补充。

## 现有任务包

| 任务包 | 范式 | 状态 |
|---|---|---|
| `mechanism-scripts-t6d.md` | T6-D | ✅ 完成（5 脚本 + B1 + B2） |
| `mechanism-scripts-t6d-results.md` | T6-D | ✅ 完成 |

## 与 commit message 的关系

任务包名前缀不在 commit message 重复——commit message 只描述**做了什么**，不重复范式名。

例：
- 任务包：`mechanism-scripts-t6d.md`
- commit 1：`feat: 5 mechanical scripts (T6-D Cluster 1 complete)`
- commit 2：`feat: B1 commit hook + B2 audit pipeline (T6-D Cluster 2)`
- commit 3：`docs: results`

## 错位修正历史

- 2026-08-17 初版用 `t6d-01-*.md` 命名——错把 T6-D 范式当任务编号
- 2026-08-17 修正为 `<主题>-t6d.md` 格式
- git history 保留 `t6d-01-` 命名作为修订痕迹

## 不做

- ❌ 不在任务包名加范式编号（如 `t6d-01` / `t6d-002`）
- ❌ 不混用范式后缀（一个任务包只用一种范式）
- ❌ 不省略范式后缀（范式是任务包的核心属性）
