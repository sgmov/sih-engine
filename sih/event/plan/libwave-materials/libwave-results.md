# libwave 结果档（2026-09-23）

批主：sess-zcode-260923-libwave；租约会话 2950679617846018；分支 msh/libwave（对 main）。
意图笔 event_hash 596ae35b；施工四子代理（lw-dec、lw-hash、lw-index、lw-std）工地施工，主会逐项独立核验后采信。

## 交付与验收判词 {#verdict}

- lw-01 三文档落据：DEC-031 哈希与参数解析归库政策、DEC-033 DES-001 适用界澄清、DES-020 AGENTS.md 内部工具索引设计；主会亲验三件核阅 exit 0 加化格 0 加检词 0（DES-020 经一次懒波词微修，见偏差节）。
- lw-02 哈希归并：13 bin 的 fn sha256_hex 逐字等价核验通过（md5 指纹 836350a8 十三份同一）；src/hashutil.rs 新件（头注载 DEC-031 正典指针，附两条既知向量测试）；13 bin 调用位全转换（12 bin 统一形加 projsnap 保留 sha2 导入特例）；主会亲验 grep src/bin 零命中与 hashutil 恰一处、cargo build 全绿、lease 27 passed、event_stream 101 passed、hashutil 2 passed。
- lw-03 索引补全：引擎 AGENTS.md 新增内部工具 bin 索引 36 件（与 src/bin 差集零多零漏）与 lib 12 mod 清单（O2 五件收口标注）与 RANTS 豁免一行；全文 171 行不超 200。
- lw-04 标准件：CONTRIBUTING 25 行摘要件、CHANGELOG 未发布加 0.9.0 候发布两节（内容面对表发布计划原文零编造）、RANTS.md frontmatter 豁免三行正文零动。
- 双跑对表抽检：formatter 引擎件与围堰件同参双跑 cmp 逐字节 IDENTICAL；gauge 以算法逐字等价加调用位不动佐证，全量双跑归融回线验收（对表 SPEC-015 候批）。

## 偏差 {#deviation}

- DES-020 初版照录 sih.rs 头注带入懒波词「开域」致检词 exit 1，编排位裁按 rulenorm 批先例改写规避，微修后三步 exit 0；承载对表 DEC-017 立名纪律。
- projsnap 特例保留 use sha2 导入，因其 sources_hash 在 sha256_hex 之外直用 Digest；范围边界即 DEC-031 src/bin 面，event_stream/certify.rs 的 pub(crate) 同名函数属库面不在本批。
- lw-hash 头注初版字面量含函数名污染 grep 计数返工一次改写消除；对表 DEC-031 验收判据与先红留痕纪律。
- CHANGELOG 节序按 Keep a Changelog 惯例以未发布置顶，0.9.0 候发布节随其后；对表 DEC-033 与 REC-018 执行面。
- AGENTS.md 与 CONTRIBUTING 与 CHANGELOG 与 RANTS 四件在 des-001 域外不跑核阅管线，域外 exit 2 照实记；对表 DEC-033 域界条款。
- parse_args 实况六处在 bin 面一处在库面 pendline 共七处，DEC-031 按实况落据，cliargs 候后批。

## 尾读数 {#tail}

认证、settle、收约、reconcile、链 verify、回锚读数由编排位收约后补记，本档不预书。
