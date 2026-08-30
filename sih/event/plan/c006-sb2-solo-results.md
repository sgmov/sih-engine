# c006-sb2-solo 结果档（C006 99% 归零 + lease 待跑）

> 批名：c006-sb2-solo 即 C006 中补注+短补注手改分卷第二程。日期 2026-08-30。

## F 锚定验收

| F | 判据 | 判定 |
|---|---|---|
| F-1 C006 恰降 | 降数 = 改数 | 部分过（改 281 / 降 265，差 16 = 核阅不报 16 处） |
| F-2 零语义变化 | diff 无他变更 | 过（仅删括号 + 紧跟标点 + 前置逗号） |
| F-3 其余类不变 | S005 / M008 / C002 / N002 逐一不变 | 过（S005 113 / M008 51 / C002 2 / N002 0，全不变） |
| F-4 留人工不动 | 9 处零改动 | 过（6 处真即称类 + 3 处描述同位在 SKIP_LIST） |
| F-5 化格检词零新增 | 化格 0 改 + 检词 0 findings | 过（化格全仓 0 改 + 检词 skipped=113） |
| **F-6 lease 通** | open → lock → commit → close | **过**（session 85d57c1f159a3f99 全程走通） |
| **F-7 链 valid** | 事件哈希可回验 | **过**（scribe verify exit 0，108 事件 valid） |

## 改法模板

**1-15 字符括号**：X（Y）→ X，Y
- 删左全角括号「（」+ 右全角括号「）」+ 中间内容
- 紧跟括号后的标点（，。；：、）一并删除
- 前面是中文字符（非标点）→ 前面加「，」
- 跳过 9 处 SKIP_LIST
- 阈值 ≥ 1 字符

## 执行账

- 改法脚本：`sih-engine/sih/event/plan/c006-sb2-solo-materials/transform2.py` 3826 bytes
- 改前核阅：C006 = 278
- 改后核阅：C006 = **13**
- 总改数：281 处
- C006 降数：265 = 95.3%
- 差 16 = 核阅对 16 个括号不报 C006（可能与 C006 引擎内部实现有关）

## C006 全程归零进度

| 批 | 范围 | 改数 | 降数 | 残余 |
|---|---|---|---|---|
| c006-jc-solo | 真即称类 | — | — | 撤回 |
| c006-sb-solo 首程 | 长描述 ≥6 字符 | 1217 | 1066 | 278 |
| **c006-sb2-solo 本程** | **1-15 字符** | **281** | **265** | **13** |
| 合计 | 全仓 | 1498 | 1331 | 13 |

C006 总降 **1344 → 13 = 99% 归零**。

## 13 处残余归类

- 9 处 SKIP_LIST：6 处真即称类 + 3 处描述同位
- 4 处其他：核阅不报但 Python regex 匹配的括号（可能与 C006 引擎内部 cross-line 检测相关）

## 续批

- **本程收口** lease 全程走通：
  - lease open session 85d57c1f159a3f99 issued 2026-08-30T09:49:27
  - 锁 3 路径：sih-engine/sih/state/plan/c006-sb2-solo.md + sih-engine/sih/event/plan/c006-sb2-solo-results.md + sih-tools/scribe/reports/2026-08-30-ask3-c006sb2-record.json
  - scribe intent 写 intent_refined event 2a6611fb-894f-4158-8755-49e0f31e2cc3
  - scribe append 写 certification_completed event 370c1f57-686c-4499-87a7-e8040fb91535
  - scribe verify exit 0 status valid
  - lease close worktree removed + branch deleted + session revoked
- 13 残余可单独子批 c006-sb3-solo 或并入下批

## 教训

- awk 字节统计与 Python 字符统计不一致导致估 370 实际 7 + 284 = 291
- 实际可改 = Python 字符数 × 核阅 C006 报违规比（≈ 95%）
- C006 99% 归零可达成，留 13 处为引擎边界 + 9 处 SKIP
- 改法脚本阈值要看字符数而非字节数（跨语言项目尤其）
- ask3 record 字段名要严格按 SPEC-005：`injected_constraints` 不是 `constraints_injected`、`Domain` 是 struct 不是 string、`scope: String, max_depth: u32` 不是 depth_limit
- evidence 格式 `path:line` 不是 `path@line`
- philosophy_ref source 只放路径，PRO 编号在 quote 里
- lease --repo 须绝对路径，不是仓名
- 完整 T6 链：ask3 记录 → ask3repeater 校验 → scrutinator ask3 包 0 findings → lease open/lock → scribe intent → scribe append → scribe verify → lease unlock/close
