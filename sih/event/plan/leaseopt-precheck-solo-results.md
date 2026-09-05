# leaseopt-precheck-solo 结果档

> 批：leaseopt-precheck-solo（leaseopt 线批三：开工施工面交集预检）
> 会话：7060fdd0e38a1c3e。日期：2026-09-05。单线形 solo，主会亲写。
> 令源：用户 2026-09-05「直接收了批3吧，未来怕忘」。

## 一句话结论

开工预检闸完成：open 前置对本批 allow 声明面与活跃会话独占持锁面只读交集探测，谓词与锁库同义，施工面真交集即拒开载持有者清单与 wait-turn/takeover 提示，治理共享白名单六路与 append 持位豁免即误伤域闭合；105 件测试全绿（104 旧零回归 + 1 新三态件），CONTRACT 1.22.0 修订三十四；得一裁 near_threshold 挂起待人复核，批保持开位主树零归并。

## 一、F 表

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 交集拒开 | 施工面交集载持有者清单 | 过 | test_lockdb_precheck_conflict_and_exempt：doc/x.md/sub 对 sid1 持锁拒 open_precheck_conflict |
| F-2 豁免域 | 白名单与 append 豁免 | 过 | 同测试：只 allow meter/counts 放行 |
| F-3 清场放行 | 无持锁不拦 | 过 | 解锁后同 allow 再开预检不拒 |
| F-4 回归 | 全族零回归 | 过 | 105 passed；open --locks 覆写增参为兼容增补 |
| F-5 得一裁 | near_threshold 挂起 | 挂起 | m-precheck-switch-1 九发全 comply 稳、边界旗零、依据族基线四×6 基线一×2，sign 未执行待人复核 |

## 二、越线申报

1. near_threshold 为第四次同形态（批二批四批五先例 confirmation 三件在链），依据族双源仍为机制真实面如实作答。
2. open --locks 增参为预检读数面必需（多环境读数指向），缺省主树账既有形态零变化。
3. recall 零命中如实记；checkcite pass 在档；SCOPE_SHARED_SURFACE 架构常量冻结登记 CONTRACT 修订三十四。

## 五、待复核后收口路径

同批四批五：confirmation 落链 → 执契 → 三仓 settle → close → reconcile → 主树复验 → 收口附记。收口后租约线六批全毕（含用户裁定的批三独立实施），线纲批三节同步更新。

## 六、收口附记（close 后回填）

- 人节点复核：用户 2026-09-05 会话令「同意」落链（confirmation c22896446f3f0873），near_threshold 挂起位放行，开工预检生效。sign 未执行系 near_threshold 机制仅 stable_clear 可签（四批先例同形），check pass 与 verify identical 在档。
- 收约实录：放锁后 close 一次通过（新机制协同第三个活体，零拦截零 bypass）。
- reconcile 三仓 unrouted 全零（tools cert_missing 1 与 math 2 皆批前既有），链 verify valid 15 事件；主树亲跑全族 105 件绿；1.22.0 三源主树对齐。
- 线纲批三节更新承载用户裁定与本批实施，随本笔入版控。
