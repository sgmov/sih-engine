# idenlane-guard-solo:守卫直改车道收尾与判定

> 令源:用户 2026-09-05 裁定(leaseoptsettle-solo-results.md 第六节) + 用户 2026-09-05 16:42 拆分裁定(idenlane-solo-results.md) + 用户 2026-09-05 17:53 勘误指令(主树已合 2 文件起步不重做) + 用户 2026-09-06 扩面令(轻车道兼容裁定「我认为是需要的」与「批C你自己拉子代理跑」)
> 范式:T6 单线 solo,委外代理亲写零子代理(主会拉子代理执行)
> 拆分定位:idenlane-solo 拆三批之批 C,承载 F-3 守卫直改车道收尾 + F-5 判定语义三裁(合法提交形) + 2026-09-06 扩面三条
> 范式:批 A 落定直改笔形与批 B 落定 human seat 后开(批 B 已收约 be316fe3 2026-09-05T16:06Z)
> 现状:主树 d5cd596f 已合 2 文件实装,本批起步不重做守卫改造,只补判定三裁 + 扩面三条 + 管线认证收约

## 〇、2026-09-06 扩面追记（三条,随本批承载） {#addendum-260906}

- **T-8 reconcile 路由认账**:ok_direct 形提交(守卫 grep 链笔引用在链)在 reconcile 对表视 routed,不再强制 bypass 登记。令源即主会当日四笔 bypass 实证噪声(c8861e9f 台账补录、5cba19e5 补录入版控、fe6af59a 夹具入库、CALL-LOG 补笔——俱为几行小改动被迫走例外通道,bypass 信号价值被稀释),承用户轻车道兼容裁定
- **T-9 轻车道范围白名单冻结登记**:直改车道合法提交的文件类白名单,起步宁窄勿宽——文书类即 lease/ledger 台账、lease 与 scribe 的 CALL-LOG、scribe/identity/tally reports、parking materials 与 PARKING 投影、未跟踪件处置;源代码与引擎件不入选仍走工地全仪式。白名单为冻结架构常量入 CONTRACT,扩面走修订
- **T-10 直改车道锁门语义裁定材料**:主会 2026-09-06 实测 --no-session-reason 直改笔在他会话独占 trail 锁窗口落链畅通(364f2919 与 d3db0787 两笔在 gvec-v2-serial 会话 c41ce2af 独占锁持有期入链)。批内出两案裁定材料呈用户裁:甲案追加面豁免合法化(trail 与台账属追加共享面,直改笔免锁门即现行既成事实的正典化)、乙案直改笔尊重全锁面(他批独占期直改笔拒写排队)。引擎侧实装(若乙案)归后继批,本批只出材料与裁定呈报
- 现状更新:lease 1.25.0(openhyg-solo)在役含按表解析与闸序新序与 PID 探针;BATCH-FACE 2026-09-06 勘误节五条在档;GUARD_VERSION 与守卫源以主树现行版为准,起步先实读

## 一、问题陈述

- commit-msg 守卫只拦四形(settle/wip/merge/direct),主树 d5cd596f 已实装 direct 形(携带直改链笔引用前八位)与当日 trail grep 核验
- 直改提交形判定的合法提交形判定语义三裁未跑
- 本批须跑 lease tests 全测绿零回归 + 管线认证收约

## 二、关键设计

### 2.1 合法提交形(direct,主树已实装)

- **现状(主树 d5cd596f)**:guardcore.py GUARD_VERSION 升 1.14.0,新增 DIRECT_PEN 正则 (`直改链笔[：:]<事件哈希前八位>`),validate_commit_message 首位判定携带引用即形态合规(返回 ok_direct);hooks/commit-msg 接 validate_commit_message 返回 ok_direct 时对当日 trail grep 核验引用前八位真实在链,fail-closed 引用不在链即拒
- **本批补**:判定三裁(facet 合同模式) + 管线认证收约(lease tests 全测绿 + 收口 settle close reconcile verify)

### 2.2 链笔引用核验(主树已实装)

- 守卫在 commit-msg 钩位:取 直改链笔 字段前八位
- 对当日 trail 文件(ROOT_DIR/sih-engine/sih/event/trail/{date.today().isoformat()}.ndjson)grep 前八位
- 命中即放行,未命中即拒并出指引
- grep 退出码零即命中,非零即未命中

### 2.3 判定语义(合法提交形,本批跑三裁)

- 合法提交形四值:settle/wip/merge/direct
- direct 形判别三件:subject 形 / chain-pen 挂接 / 引用 grep 命中
- 判定行为变更过得一裁(facet 合同模式, near_threshold 呈用户确认)

## 三、工作清单

- [x] T-1 commit-msg 红证:无 chain-pen 挂接行即拒（夹具 test_hook_direct_edit_without_pen_rejected，前代守卫两边皆拒项注记在档）
- [x] T-2 commit-msg 红证:chain-pen 引用前八位未在当日 trail 命中即拒（test_hook_direct_pen_not_on_trail_rejected，fail-closed 指引在场）
- [x] T-3 commit-msg 绿:chain-pen 引用在档即放行（test_hook_direct_pen_on_trail_allowed，全角冒号同形过）
- [x] T-4 commit-msg 红证:subject 形不匹配 direct-edit 前缀即拒（形不符挂接四形不入直改车道，钩子位与纯函数位双证）
- [x] T-5 guardcore 直改形态判定函数（主树 d5cd596f 已实装 DIRECT_PEN 与 ok_direct，本批 GUARD_VERSION 升 1.15.0 载白名单常量）
- [x] T-6 钩子接 guardcore（主树 d5cd596f 已实装，本批零改只对表）
- [x] T-7 判定语义三裁(facet 合同模式)（gid m-idenlane-okform-1 九发 stable_clear，9/9 comply 变卦 0 旗 0，check pass 12 verify identical，机器终签链笔 8e97cadc）
- [x] T-8 reconcile 路由认账(ok_direct 视 routed,零 bypass 强制)（新类 routed_direct 计入 routed 汇总，引用不在链落 bypass 对表 fail-visible）
- [x] T-9 轻车道范围白名单冻结登记(架构常量入 CONTRACT)（guardcore.DIRECT_LANE_FILE_WHITELIST 十条目冻结，CONTRACT 修订三十九登记，声明位零执法如实注记）
- [x] T-10 直改车道锁门语义两案裁定材料呈用户(零引擎实装)（两案材料落结果档候裁，链笔 364f2919 与 d3db0787 实证在链）

## 四、可证伪条件

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-3** | 守卫直改车道 | 携带链笔引用直改提交过闸,无引用裸提交仍拒,引用未在链拒 |
| **F-5c** | 判定语义三裁 | 合法提交形分类清楚,facet 合同模式九发 stable_clear 过执契 |

## 五、必读文件

- 设计约束:sih-engine/sih/event/plan/idenlane-solo-results.md
- 批 A 任务包:sih-engine/sih/state/plan/idenlane-envelope-solo.md
- 批 B 任务包:sih-engine/sih/state/plan/idenlane-human-solo.md
- 守卫现状:sih-tools/lease/hooks/ + sih-tools/lease/src/lease/guardcore.py

## 六、约束

1. 判定语义三裁过得一裁(facet 合同模式, near_threshold 呈用户)
2. 链笔引用必核事件真实在链(对当日 trail grep)
3. TDD 先红后绿
4. 新增判定常数零裸奔
5. 合规四形 settle/wip/merge/direct 覆盖完整,既有三形零回归
6. 拦截多余但不可漏(拦截语义同 pre-commit)

## 七、验收标准

- [x] F-3 / F-5c 全过（F-3 四夹具绿证在档，F-5c 九发 stable_clear 过执契）
- [x] 先红后绿夹具在档（现行版红 4 转 149 绿，前代守卫 GUARD 1.13.0 红 7 绿 3，materials/tdd/ 双证）
- [x] 既有 lease tests 全绿零回归（139 基线全绿，合计 149 绿）
- [x] 双仓 settle + close + reconcile + verify（读数见结果档第七节）
- [x] 结果档 sih-engine/sih/event/plan/idenlane-guard-solo-results.md
- [x] CONTRACT 修订与 CALL-LOG 双笔随批（修订三十九升 1.26.0，lease 与 scribe CALL-LOG 各一笔）

## 八、风险点

- 链笔引用 grep 性能:当日 trail 文件可能万行,八位哈希前缀存在碰撞概率(2^32 分之一),碰撞通过 event_type 二次确认
- 守卫放行逻辑放宽可能被滥用:链笔引用校验须核事件真实在链,不核引用即形同虚设

## 九、范式偏离声明

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步;本批为 idenlane-solo 拆三批之批 C。

## 十、关联文件

- 上游决策:sih-engine/sih/event/plan/idenlane-solo-results.md
- 上游任务包:sih-engine/sih/state/plan/idenlane-solo.md
- 上游批 A:idenlane-envelope-solo
- 上游批 B:idenlane-human-solo

## 十一、请求写入

- sih-tools/lease/src/lease/guardcore.py —— 加 direct 形态判定纯函数
- sih-tools/lease/hooks/commit-msg —— 钩子薄壳(零逻辑只接 guardcore)
- sih-tools/lease/CONTRACT.md —— 修订
- sih-tools/lease/tests/ —— 新增 direct 形测试
- sih-engine/sih/event/plan/idenlane-guard-solo-results.md 与 materials/
- sih-tools/lease/CALL-LOG.md 与 sih-tools/scribe/CALL-LOG.md
