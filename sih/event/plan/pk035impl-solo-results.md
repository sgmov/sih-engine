# pk035impl-solo：席位身份入正身实现批结果档

> 承任务包 sih/state/plan/pk035impl-solo.md 即 2026-08-30 链事件 01c1d9e4
> 队形：单线形 solo
> 日期：2026-08-30

## 概览 {#overview}

- facet 材料腿即计分材料 version 2 强制携带身份哈希与规范席位串三段式、缺报即拒、先红后绿::[facet](#facet)
- 探针腿即标定账本增身份列、漂移配对哈希优先串回退即当前带哈希只认哈希相等行::[probe](#probe)
- 执契腿即 R5 双带哈希比对、哈希异按漂移挂起、任一缺席回退旧件重放同判、assemble 透传::[tally](#tally)
- pk-035 出泊 promoted 即名录五项、两契约修订留痕、正身本体零改::[exit](#exit)

## 一、facet 材料腿 {#facet}

canonical_seat 即框架:模型:版本三段式大小写敏感、空段含冒号含空白拒、缺版本补 self-reported。write_score_material 增 identity_hash 必挂参即缺席或非六十四位十六进制拒、材料 version 2 增 identity_hash 与 seat_string 双字段。measure score 增 --identity-report 与 --identity-hash 二选一必挂、报告读 identity.hash。红三枚即 canonical 缺席与必挂拒与携带形、转绿修两处即必挂形参 TypeError 亦算拒与测试预期放宽。

## 二、探针腿 {#probe}

_pair_hist 纯函数即双方带哈希按哈希相等配对、当前带哈希只认哈希相等行即串同哈希缺亦不混比、当前无哈希回退席位串即旧行为保底。score 增 identity_hash 必挂参、账本行增 identity_hash 列。三测即哈希优先与回退与账本行形。

## 三、执契腿 {#tally}

R5 块双带哈希即比对、一致续判注记哈希一致、不一致置 suspend 走优先级映射即挂起、任一缺席回退现行为即旧材料与旧基线重放逐字节同判。assemble 自计分材料透传 identity_hash。三测即一致过与异挂起与无哈希回退。

## 四、出泊与契约 {#exit}

pk-035 exit promoted 入链即出泊条件用户令开工满足、名录当前在泊五项、历史住户十四项。tally CONTRACT v1.2.0 即 R5 哈希优先与透传与版本三源连带对齐、identity CONTRACT 修订五即外部携带界留痕正身本体零改、facet-measure SKILL 修订七加投影同步即采样标定前置正身 verify 与必挂参纪律。身份报告实测 identity.hash 六十四位十六进制即本会话正身件直读。

## 五、验收判定 {#acceptance}

- F-1 强制携带：过。材料与账本行带双字段、缺报拒、facet 四套件绿
- F-2 哈希优先：过。三态测试绿、回退路径旧件同判
- F-3 界与出泊：过。identity 组件代码 git diff 零、exit promoted 入链、认证在链

## 六、偏离与期票 {#deviation}

偏离一：PARKING 名录投影更新在主树未提交块内随既况、工作树分支不触名录避并行批归并冲突。偏离二：并行批 fmtc2b-solo 在跑、共享日链追加交错全程 valid、收约按备份还原模式处置。偏离三：任务包请求写入漏 tally pyproject 与 identity CALL-LOG 两行、范围闸二次拦截即按两程模式修正包后第二程补齐、版本三源在本批窗口内瞬时不对齐后即齐。
期票一：标定账本既有行无身份列、次日基线重量自然补齐、历史行不回填。期票二：合同 emit 半不携带身份即出题与验身分离、如需绑定另批裁定。

## 七、关联 {#related}

- 得一终签四件 3706bac0 17250813 86abafbb 01bf2697 即设计输入、pk-035 泊件、identity CONTRACT 修订五、tally CONTRACT v1.2.0
- 三问记录与验货与信号件与召回面均在 scribe/reports 2026-08-30-ask3-pk035impl 与 pk035impl 各件
