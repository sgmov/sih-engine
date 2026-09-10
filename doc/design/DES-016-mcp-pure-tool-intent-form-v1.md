# DES-016 MCP 纯工具化意图形分离设计

## 概览 {#overview}

- 本设计承 pk084des016-solo 批测量与执契终签，令源用户 2026-09-10 两令即「司衡的哲学不应该侵入其他项目」与「这个不是常识吗？世界上没有任何一个工具承载了哲学仪式」，m-des016-2 三发 stable_clear 经 attractor 裁决通过机器终签 756effb1 在链::[问题陈述](#problem)
- 意图形分派判词取双形中性：plain 形与 ask3 形俱合法过意图闸，闸验结构不验哲学，外部域 plain 形零哲学引文可写链::[意图形分派](#intent-forms)
- 意图闸执法判词取结构校验形：validate_intent 摘除内嵌 scrutinator ask3 规则包与 ask3repeater 双验，改验记录结构面，零语义裁决零 LLM::[意图闸执法](#gate-enforcement)
- ask3 纪律归位判词：三问双门留司衡本域批纪律由 BATCH-FACE 必跑步承载，ask3 工具生命保留归纪律层调用，机械层零哲学代强制::[ask3 纪律归位](#ask3-discipline)
- scribe plain 通道判词：record_intent 对 plain 形豁免 ask3 验证件或以结构自证件等价承载，拒因面相应收窄::[scribe plain 通道](#scribe-channel)
- 契约与正典修订判词：lease CONTRACT 修订载意图形分派与闸序改，DES-014 修订载意图闸描述更新，SPEC-023 读面零触碰::[契约修订](#contract-revisions)
- 外部域接入判词：mcpline init 落 plain 意图模板与用法行，新城即照填即用::[外部域接入](#onboarding)
- 存量处置判词：已落盘外部域哲学引文件留史不洗，链面只存哈希审计不断，替换重申归各域人节点::[存量处置](#legacy)
- 边界申明：本设计零实装，实装批范围与版本排程候用户裁，判定语义变更经本设计测量与终签在案::[边界申明](#boundary)

## 问题陈述 {#problem}

- 病灶：lease open 意图闸 validate_intent 对每会话每域强制跑 scrutinator ask3 规则包与 ask3repeater 双验，后者以司衡工作区根解析引文源头；scribe intent 写位硬校验 ask3 验证件。外部域写一笔必须产出带司衡哲学逐字节引文的 ask3 记录
- 实证：InferServer 域内意图件与验证件载 sih-philosophy 引文与锚在档；链面只存记录哈希与锚数零引文文本，哲学住域内文件不住链
- 用户裁定锚定事实：工具承载哲学仪式违普遍工程常识，世界上没有任何一个工具承载哲学仪式，摘除即履约常识非规范开放题
- 测量在案：m-des016-1 三发 boundary 打回重作红证存对照，m-des016-2 三发合合合 stable_clear 经执契裁决通过机器终签 756effb1

## 意图形分派 {#intent-forms}

- 判词：双形中性分派。意图记录增设形位字段或以结构特征分派，载 anchors 者为 ask3 形，缺省者为 plain 形，两形过同一结构校验通道，闸对两形行为一致
- plain 形必填集：session_id 与 raw_input 与 round 与 intent_contract 即 goal 必填与 domain_contract，零哲学引文零锚要求
- ask3 形必填集：现行 ask3 记录全形不变，司衡本域批继续产出
- 依据：闸是结构校验器非语义裁决器，两形在闸眼内同为合格意图记录，基线一确定性执行者原位

## 意图闸执法 {#gate-enforcement}

- 判词：validate_intent 摘除两段内嵌子进程调用即 scrutinator ask3 包与 ask3repeater，保留 JSON 解析与必填字段校验与 record_sha256 计算三件，校验失败报文教学化即缺何字段逐名列出
- 摘除依据：哲学引文逐字节核验属纪律层执法，机械代强制即工具越界承载哲学仪式，此为用户裁定锚；结构面校验已足挡残缺意图上链
- 五验与链闸与 closeguard 与理由码全部原位，本设计零触碰执法面其余各位

## ask3 纪律归位 {#ask3-discipline}

- 判词：三问双门即 scrutinator ask3 包加 ask3repeater 留司衡本域批纪律位，BATCH-FACE 全链序第一步必跑步不变，双门退出码零方过批的批纪律不松
- 归位依据：纪律的执行者从 lease 机械闸移回批仪式本身，司衡本域批照旧被双门把守，外部域不再被代为强制
- 工具生命：scrutinator 与 ask3repeater 零退役零改动，调用位从 validate_intent 内嵌改回批仪式显式调用

## scribe plain 通道 {#scribe-channel}

- 判词：scribe intent 对 plain 形豁免 --validation 必填，plain 形可缺省验证件上链，意图笔 details 载 validation 缺席标记；ask3 形照旧必填
- 拒因面收窄：plain 形不再触发 ValidationNotOk 与 LineageMismatch 与 FindingsPresent 三拒因，RecordMissingField 收窄至 plain 必填集
- 引擎件改动申报：本条涉 scribe 引擎二进制校验逻辑，实装批含 cargo 改与测试，版本走引擎既立换版程序

## 契约修订 {#contract-revisions}

- lease CONTRACT：修订五十七载意图形分派与 validate_intent 双验摘除与教学报文形，lease 版本 1.40.0 升 1.41.0
- DES-014：修订二载意图闸六位闸序第三位描述更新，即意图闸验结构面、ask3 双门归批纪律位；授权矩阵与身份与审计各节零触碰
- SPEC-023：零触碰，alpha 读面与本设计无交

## 外部域接入 {#onboarding}

- 判词：mcpline init 落地五步增 plain 意图模板件即 TEMPLATE-意图-plain.json，载 plain 必填集骨架与占位教学文本；sih/README.md 用法节增意图填写行
- 域自举其余四步与幂等守卫与前置检查六项零变更，mcpline init 版本随实装批 additive 升

## 存量处置 {#legacy}

- 判词：留史不洗。已落盘外部域哲学引文件即 InferServer 意图件与验证件原样保留，append-only 与审计哈希不断
- 替换或重申归该域人节点裁，司衡侧零代行；新域自此走 plain 形零新增哲学引文

## 边界申明 {#boundary}

- 本设计零实装：lease 与 scribe 与 mcpline 与 scrutinator 代码零改动。实装批范围即 validate_intent 段与 scribe intent 校验段与 CONTRACT 修订五十七与 DES-014 修订二与 init 模板件与三套测试族，版本排程候用户裁
- 判定语义变更在案：m-des016-2 三发 stable_clear 即合合合变卦零，加 attractor 裁决通过，加机器终签 756effb1，加用户裁定锚，本设计候用户最终裁后开实装批
- 测量局限如实申报：同席起草与作答即 ZCode:GLM-5.3-Flash:self-reported 无跨席隔离，3 发低档承用户裁定默认档；座位基线经读档注入与当日计分件确定性装配，temp probe 脚本未入库的工具缺口如实申报候补

## 修订记录 {#revisions}

- 2026-09-10 修订一：随 pk084des016-solo 批起草，令源用户 2026-09-10 两令在档，测量与终签工件在 sih-tools/proposition/DES/m-des016-2/ 在档可复算
