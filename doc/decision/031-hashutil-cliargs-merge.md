# DEC-031 哈希与参数解析归库政策

本决策处置内部工具 bin 重复函数的归库界。问题即 sih-engine 的 13 个内部工具 bin 各自手抄 fn sha256_hex 且 13 份实现逐字相同；另 fn parse_args 全仓七处，六处在 bin 面，一处在库面 pendline，七处签名与返回形与旗标语义互不相同。判词：逐字副本即漂移温床，任一副本被单独改写即产出静默分叉，消除副本即消除分叉面；签名各异的函数不是重复而是待迁移面，硬并一形即改在役行为，须逐批对表候人节点裁。背景即评估报告代码质量维度 W2 申报 sha256_hex 过度重复，本批实测 13 处在案。令源即 libwave 批任务书。

## 概览 {#overview}

- 本批立 src/hashutil.rs 载一函数，13 bin 改引库形，验收三判据全机械::[本批裁定](#ruling)
- cliargs 候后批，七处签名与语义各异涉迁移行为，逐批对表::[后批界](#deferral)
- 在役双跑件 gauge 与 formatter 转换须行为保真，同参双跑抽检为证::[双跑对表条款](#dualrun)
- 转换前提即 13 份实现逐字等价核验，有异文即停批上报::[转换前提](#precondition)

## 本批裁定 {#ruling}

归库宿
: 本批立 src/hashutil.rs，载 pub fn sha256_hex 恰一函数，函数体取 13 份手抄副本的公共正文，lib 面哈希实现自此单源。

受辖件
: 十三件即 acceptor 与 basemgr 与 cascade 与 confledger 与 formatter 与 gauge 与 identity 与 incubation 与 lease 与 locator 与 parser 与 projsnap 与 tally。逐件删本地 fn sha256_hex，改写 use sih_engine::hashutil::sha256_hex。

验收判据
: 判定命令一：grep fn sha256_hex src/bin 零命中。判定命令二：fn sha256_hex 在 lib 面恰一处。判定命令三：cargo build 全量绿。三判据同时成立即验收过，缺一即不收。

当前状态判词
: 文档收口批核日 2026-09-29。判定命令一实测命中零，bin 面 13 份手抄副本已全数归库，达标。判定命令二实测不达标：lib 面 fn sha256_hex 现两处。一处即 src/hashutil.rs:17 收归宿，签名 pub fn sha256_hex, 入参 bytes: &[u8], 返回 String。另一处即 src/event_stream/certify.rs:19 同算法镜像，签名 pub(crate) fn sha256_hex, 入参 text: &str, 返回 String。两份实现逐字节等价，同为 SHA-256 加 hex::encode 小写输出，certify 版多一步 text.as_bytes() 调用。判别为重复代码债非行为分叉，certify.rs 一份实装待后续批收归，本决策档验收令按两处镜像形态记入并存期判词。判定命令三实测绿。验收判词暂以「达标加一枚像镜像未收归」形态记录，全收归批候人节点立项排期。

## 后批界 {#deferral}

cliargs 归库
: fn parse_args 七处不归本批，候后批。理由：七处签名与语义各异，入参形有零参与 argv 切片两形，返回形有命名结构体与元组与 Result 各形，旗标语义随各 bin 职能分化；归库此处不是消除重复而是迁移行为。后批逐批推进，每批限一处或一组同形处，转换前对表判词在档，行为差异显式申报候人节点裁。

## 双跑对表条款 {#dualrun}

行为保真
: 在役双跑件 gauge 与 formatter 的哈希归库转换须行为保真：转换前后同参双跑，出参一致为过，抽检判词落批结果档，缺抽检判词即验收不闭合。

## 转换前提 {#precondition}

逐字等价核验
: 转换前提即 13 份 fn sha256_hex 实现逐字等价核验，核验形即逐件抽取函数正文对哈希同一，本批实测十三份哈希同一在案。有异文即停批上报：异文可能是未察觉的行为分叉，也可能是手抄时被修掉的缺陷，归库择一杀一，裁归人节点，施工位禁自择。

## 关联 {#relation}

lib 模块布局锚即 DES-007#cargo-layout；归库对象即 src/bin 十三件，归库宿即 src/hashutil.rs 新件；重复实证即评估报告 doc/assessment/2026-09-22-sih-engine-evaluation-report.md 代码质量维度 W2；双跑对表纪律承 AGENTS.md 工具层静态审计节；本批载体即 libwave 批。
