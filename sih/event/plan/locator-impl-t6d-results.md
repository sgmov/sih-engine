# locator-impl-t6d 结果档

## 概览 {#overview}

- SDD 规格先行即 SPEC.md 八节冻结，TDD 红先绿后即红跑五十九败留痕在档转绿五十九全过零警告::[判据核对](#criteria)
- 载体矩阵四类全落地，稳定标识承 DES-009 可复算，金向量十件冻结全对拍，双跑逐字节一致::[链上证据](#evidence)
- 真实语料烟测即 sih-engine/doc 62 文件 4662 条目零解析错 0.24 秒，ref 查询 DEC-007 即中 14 条目加 16 出现::[链上证据](#evidence)

## 链上证据 {#evidence}

意图入链
: hash 7267464b 即 sess-zcode-260827-locatorimpl 经引擎件 scribegate 落新家，双腿绿即围堰 ask3 包零发现加引擎闸门三锚点过

会话开工
: 3b06d7a7d61ccbc4 即双仓副本 worktrees 下 locator-impl-t6d，写入面六条与包一致，七锁在册含一处同路径复验重复行

规格冻结
: sih-tools/locator/SPEC.md 八节即模块布局、条目模式、id 派生公式、头部与索引件模式、包模式、六操作签名、确定性细则、退出码表，文体承书简 SPEC 先例

TDD 红先
: 测试先行全套落 tests 十三文件，红跑五十九败零过即 NotImplementedError 全败，失败数在档即本节

TDD 绿后
: 实装八模块加 carriers 三件，五十九测全过零警告即 -W error 档全绿

金向量冻结
: 冻结语料五件加期望件两件加 manifest，向量十件即 build、double-run、query 五旗、stale 三态场景，locator vectors 全过退出码零，期望件经通读复核后冻结不盲冻

烟测
: packs/core 对 sih-engine/doc 建索引即 62 文件 4662 条目零解析错 0.24 秒，query --ref DEC-007 命中 14 条目加 16 出现，退出码零

契约修订
: CONTRACT.md 修订二即参数冻结与实现批产出记录，两张期票维持边界条款不进场

管线与认证
: 化格核阅检词照旧即笔在核前判在书简前，认证经引擎件落新家，证据见新家链

段结算提交
: 双仓经 lease commit 正身路径，提交号见 git log，收约归并号随收约落

## 判据核对 {#criteria}

F-1 过即 SPEC.md 八节齐备化格核阅零发现。F-2 过即红跑五十九败留痕转绿五十九全过零警告。F-3 过即四载体各产条目且 id 派生可复算即已知向量测试硬编对拍。F-4 过即文件后端六操作实装且 query 五旗行为如契约。F-5 过即双跑逐字节一致与金向量十件全对拍与退出码三值如契约。F-6 过即仅文件后端零网络零 daemon 源文件零写入且双仓免参对表退出码零。

## 偏离登记 {#deviations}

偏离一：红相初期测试文件以 tests 包名导入失败即模块级自理修正，修正后红跑五十九败为准，测试自理不计入实现修正。

偏离二：绿相过程发现三处测试契约自身错误即 storage 三态语义混淆、词查询断言对象错位、query 首行误取文件记录为条目，修正走向更严不迁就实现。

偏离三：markdown-it 令牌类型为 paragraph_open 而单元类型照规格记 para，即令牌名与条目 kind 名分离，实现承规格不承令牌名。

偏离四：tree-sitter Python 绑定 0.26 的 query 接口为 Query 构造器加 QueryCursor 即 Language.query 不存，依赖经 uv.lock 锁版本。

偏离五：queries 与 vectors 收入 src 包内即承书简 golden 先例，防轮子安装态路径失效，SPEC 模块布局所记位置随迁。

偏离六：核阅域声明缺口即 des-001 包域为 sih-engine/doc/** 而本批规格契约在 sih-tools、包档结果档在 sih/，承四批先例按零发现认证，域覆盖扩展待 des-001 包版本变更裁。

## 缺陷候选 {#defects}

候选一：build 对解析失败文件静默归 header parse_errors 即索引缺文件不显眼，候选改进即 parse_errors 非空时 stderr 警示或退出码语义扩展，待人裁。

候选二：json 与 toml 无行位记 null 即跳转定位弱，候选改进即换带行位解析器，承 SPEC 限制声明。

候选三：rust impl 泛型形态与复杂类型不捕获条目，语法包增量可解不涉契约。

## 关联登记 {#related}

AGENTS 文件索引寻址行更新为实现批已毕即根域直改不版本控制。级联 0.3.0 改消费寻址解析层即登记待时项仍候。金向量冻结后任何实现变更须重走冻结程序即向量不静默再生成。
