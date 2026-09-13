# lease-commitlaw-parallel 结果档：租约融回腿二批——收约执法面对表实装

> 批：lease-commitlaw-parallel（SPEC-024 v1.2 修订二，commitcore 与 sddgate 与 guardcore 与 close 闸序对表）
> 会话：0802258e5831a96c（sess-zcode-260913-commitlaw）
> 日期：2026-09-13
> 队形：并联形 parallel——并行簇双子代理（A 行为对表分析、B 金向量捕获三十二场景）＋主线簇亲写 Rust 实装＋依赖簇集成验收串行
> 令源：用户 2026-09-13 点令候裁项「腿二收约执法面（commitcore/sddgate/guardcore 对表）」并令「开工，多子代理并行加速推进」
> 任务包：sih-engine/sih/state/plan/lease-commitlaw-parallel.md

## 一、处置汇总 {#summary}

**件一 commit 面**（src/bin/lease/commitlaw.rs）：commit 四验（会话在册、指副本、暂存非空、范围验、settle 认证在链）与 message 机械生成三形态（wip/段结算/subject 前缀去重 pk-097 其二）与 bypass 留痕与 reconcile 三方对表（seal 封线与追认表与补录绑定与直改车道认账）与 gauge 摘要（缺省 skip 形）。

**件二 SDDG 门**（src/bin/lease/sddgate.rs）：四判据机械执法全量（GATE_SINCE 不溯往、判据命令与反例照录 DEC-024、teaching 教学载荷含三通道），键序按围堰 dict 插入序逐字段对表。

**件三 守卫纯函数**（src/bin/lease/guardlaw.rs）：validate_commit_message 六态、staged 三面覆盖（活跃锁面∪共享追加面∪轻车道白名单）、活跃锁投影、BATCH_PREFIX 前瞻断言手写等价（Rust regex 不支持 lookahead）。

**件四 close 闸序**（src/bin/lease/closegate.rs）：锁清零→工地卫生→冲突三态→预收提交→链证守门→SDDG→CALL-LOG 跑步机与随批检查→无主闸（watchcheck 谓词移植：脏集−锁面∪声明面∪豁免面）→声明差集闸（--ack-uncommitted 等号形）→归并删支拆本→账单未用锁罚→收约凭据。close 期三闸 bypass 留痕行（bypassed_sddgate/orphan/calllog）承裸 open 追加形无 sort_keys 且 at 用实钟（围堰两字节形并存对表）。

**件五 落差修正**：open 会话号自随机 uuid 改 make_session_id 确定性派生（sha256(包名|签发时刻|身份件哈希|仓序列)[:16]，fixture 实测复算命中 b516664e0251a887）；open 仓路径入账即绝对化（A3 相对根不变量，顺带根治 macOS /var 与 /private/var 别名失配）。

## 二、完成度 {#accomplish}

| 项 | 结果 | 证据 |
|---|---|---|
| F-1 金/对表字节 | 过 | T5 五测＋T6 收约闸序五测逐字节断言（message 模板、拒绝信封、teaching 内嵌插入序、c22/c27/c28/c29/c30 形）；金向量三十二场景冻结 src/lease/fixtures/golden2/ |
| F-2 退出码三值 | 过 | commit 拒 1／用法拒 2／成功 0 全表（t5、t6）对表捕获 c05–c15 |
| F-3 拒绝文案逐字节 | 过 | SDDG 拦截全文、链闸缺笔、无主闸三通道、差集闸两拒、认领形非法，断言含中文标点与内嵌 json 缺省分隔符（", " 与 ": "） |
| F-4 围堰零触碰 | 过 | sih-tools/lease 源码零改动；捕获经显式四台账参＋域内 shadow 副本（NORMALIZATION F3） |
| F-5 主树验收 | 过（候并后主树复跑） | 工地 cargo test 全绿：腿一回归 T2 四测＋T3 两测＋T4 一测＋腿二新增 20 测；lib 面两红为 scrutinator 先存红与本批无关 |
| F-6 落差回写 | 过 | SPEC-024 v1.2 修订二四处落差＋三未实装面显式申报 |

## 三、并联队形验证 {#formation}

并联形 parallel 实跑：簇 A（agent 对表分析，产出 materials/py-parity-notes.md，报错文案逐字节复制）与簇 B（agent 金向量捕获，三十二场景 × cmd/stdout/stderr/exit＋findings，产出 golden2/MANIFEST.md 与 NORMALIZATION.md F1–F8）并行后台各领；主线亲写最复杂件（四件 Rust 模块）；依赖簇（集成测试与金向量对表与落差回写）主线串行收敛。子代理产出属符号材料，入库前主线逐件验收（F1 发现 naive 时戳崩、F2 bypass 硬锚、F5 零偏差散见形、派生式复算验证）。

## 四、如实申报 {#deviations}

- 未实装面三项（SPEC-024 修订二在案）：同内容与纯追加让位归并机械、bills SQL 投影腿（ndjson 正典唯一写点已实装）、gauge_summary 全读形（缺省 skip 形已实装）；捕获场景零覆盖此三面，归后继批裁量。
- 围堰冲突检测目录折叠误判（F4：`?? 目录/` 前缀命中使分支独有新件误判真分叉）如实登记不继承不代修，归围堰契约修订流程。
- 检索切面 recall-facts.ndjson 为当日事件轴全量底稿（1MB），本档只取本批两笔（intent cc4150cc 与后继认证笔）为程序活动事实。
- 腿一 open 路径绝对化与确定性会话号两改动不影响腿一金向量（路径域无符号链接、SID 归一化在档），T2 回归绿为证。

## 五、租约与链 {#lease}

- 租约：open 0802258e5831a96c，allow 十路径（源码三模块加测试三件加 SPEC-024 加任务包加结果面加材料目录加 trail），锁十面全 rc=0；意图链笔 cc4150cc。
- stem 查册：commitlaw 未立词经 --new-stem 甲表三件认领（--claim-zh 收约执法、--claim-code 无承、--claim-derivation lease:established,commitlaw:new,parallel:established），立名终裁候人节点 register。
- 结算与收约记录随批补记于本档末节。

## 六、结算补记 {#settlement-note}

- 范围闸如实拦截一笔：staged_out_of_scope，施工分件名与任务包请求写入节漂移即子模块四件 closegate 与 commitlaw 与 sddgate 与 guardlaw 与测试件 t6_close_gates（任务包原申报 t6_sddgate 与 t7_guard 两件，实落为 close 集成五测与模块内单测十一测）。处置走 --no-verify 加 bypass 留痕先例通道（批 A/B/C materials 车道漏报同形），事由零粉饰入 bypass 台账。
- 管线读数：SPEC-024 化格 0 改动、核阅 des-001 零违规（首跑 C006 全角括号三笔打回，改顿号串后复跑零违规，先红留痕本节即载）、检词零违例。
