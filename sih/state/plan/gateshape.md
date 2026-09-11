# gateshape：判据形借入——裁判面封印查己与缺证硬闸与复核独立性

> 治理任务包（实装类，单线形 solo，委外代理亲写零子代理，DEC-018）
> 承接：用户 2026-09-11 令「你能立批进行改造吗？」——对象即同日主会话对 zcode workflow 三测试包（case-01 银行财报、case-26 企业投标、jsonl-db 性能优化）对抗性拆解后收敛的判据形借入清单
> 日期：2026-09-11
> 术语位：「闸形」词甲表认领（--claim-zh 闸形、--claim-code 无承、--claim-derivation 闸:gate,形:shape）候立名程序人节点终裁，本批正文用语以「判据形」承载；数学根据 PROB-007 期望、PROB-010 假设检验、PROB 信念更新、PROB-012 相关、PROB-014 期望信息增益；零命中申报：数据处理不等式、博弈占优（不承重不入文）

## 一、问题陈述 {#problem}

- 裁判面参数运行时可调不留痕：critsweep --threshold 可覆调（默认三日），传 30 即沉底判据全隐，改判据与改读数无异且零登记零验身零落链——与 jsonl-db 包 JQL_UPDATE_GOLDENS 逃逸阀同病（README:146 荣誉规则无机械封印）
- tally R1-R7 核对无证据状态位：证据齐备与否靠散文声明，缺证不机械拒——case-26 缺证闸门形（缺证不得标 ready）已示范布尔化
- facet 复核输入独立性无成文条款：复核方输入应为原始证据而非被复核方结论，现行 CONTRACT 未钉
- 无登记逃逸阀清单零盘点：自家判据参数面（阈值、预算、包覆写、环境阀）从未按封印三件套（持锁验身、显式登记、落链留痕）照过镜子

## 二、关键设计 {#design}

- F1 critsweep 阈值来源自申报：输出 JSON 增 threshold_source 字段（default|cli），显式阈值时附 threshold_notice 告警串并给默认阈值重算提示；TDD 红先，CONTRACT 与 registry 注记同步
- F2 tally 缺证硬闸：核对输入接受 evidence 数组（逐项 id、status provided|missing|synthetic、required 布尔），任一 missing 且 required 即判定拒绝、退出码循既有约定；TDD 红先
- F3 facet 复核输入独立性条款：CONTRACT.md 钉「facetor 输入锚原始材料，compiler 输出不得回流为 facetor 依据」，docstring 随一行
- F4 裁判面查己清单：全 sih-tools 盘点判据参数面（critsweep 阈值、gauge 预算、selector 包、nomenclator 包、tally 基线、lease ROOTANCHOR_DISABLE_SELF_BOOT 环境阀活体标本），逐件按封印三件套分类 sealed|unsealed，unsealed 各配封印建议与后继批候令；落 sih-engine/sih/event/plan/gateshape-solo-materials/referee-audit.md

## 三、工作清单 {#work}

- [ ] F1 critsweep（sweep.py、tests、CONTRACT、registry 注记）
- [ ] F2 tally（核对面、tests）
- [ ] F3 facet（CONTRACT、docstring）
- [ ] F4 查己清单落档
- [ ] 结果档起草并走管线三步（化格、核阅、检词）
- [ ] settle 认证落链、双零对账、链 verify、close、回锚五行

## 四、可证伪条件（跑前立文） {#falsifiable}

- V1 critsweep 默认跑输出 threshold_source=="default"；--threshold 30 跑输出 threshold_source=="cli" 且 threshold_notice 非空；测试先红后绿
- V2 tally 输入载 evidence:[{id,status:"missing",required:true}] 时判定拒绝且退出码非零；全 provided 时通过；测试先红后绿
- V3 facet CONTRACT 含复核输入独立性条款且检词零违例
- V4 查己清单覆盖 sih-tools 全部子工具判据参数面，每件三态其一，ROOTANCHOR 环境阀活体标本在档
- V5 结算链 verify valid、reconcile 双零；码面写全部经锁面，主树零直写不涉

## 五、请求写入面 {#scope}

- sih-tools/critsweep/（sweep.py、tests、CONTRACT.md、registry.json）
- sih-tools/tally/（源、tests）
- sih-tools/facet/（CONTRACT.md、docstring 所在源）
- sih-engine/sih/state/plan/gateshape.md（本包）
- sih-engine/sih/event/plan/gateshape-solo-materials/（查己清单、结果档、归档本包）
