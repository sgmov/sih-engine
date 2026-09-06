# toolincub-solo 结果档

> 批机械链全序实录，单线形 solo 委外执行（DEC-018 选形制，与 redkeep-solo 同一委外令串行，本批在前），2026-09-06。任务包 sih-engine/sih/state/plan/toolincub-solo.md 唯一规格源，机械链 sih-tools/BATCH-FACE.md 含勘误节。会话 e4c2ba0472b71891（lease 1.30.0）。

## 一、批机械链各步退出码

| 步 | 命令要点 | 退出码 | 读数 |
|---|---|---|---|
| 守卫启动检 | 双仓 core.hooksPath | 0/0 | 均指 sih-tools/lease/hooks |
| watch 对表 | watchcheck check | 1 | 无主清单 1 件呈报（billwire 实测残件，零代行零触碰） |
| 泊界心跳 | selector route 两线 | 0/0 | sih-tools 21 主线 1 侧线 0 告警；engine 46 主线 6 丢弃轨 0 告警 |
| 例行读数 | gauge record 三维 | 0 | convergence 0.375、adoption 0.75、mergeback 0.04 落链 |
| 排队候位 | 锁台账轮询 | 0 | 在途 regulamath-solo 独占链与 worktrees 期间候位，23:18 其收约放锁后开工 |
| 三问双门 | scrutinator ask3 包＋ask3repeater | 0/0 | findings 0；status ok 三锚 |
| 叩问 | elicit check＋digest | 1/0 | 20 轻信号全 unregistered，digest passed 全覆盖 20 |
| 正身 | identity verify | 0 | anomalies 空 |
| 租约开工 | lease open | 0 | 双仓工地，十二锁全 exit 0 |
| 书简意图 | scribe intent | 0 | 事件 b47bc8cb |
| 登记件管线 | 化格→核阅→检词 | 0/2/1→0 | 化格零改动；核阅 des-001 域外 exit-2 如实记档；检词首跑 2 笔死档词红证入 materials（词面见红证件），改「收录」复跑零违例 |
| 懒波登记 | lazy.json 三工作名 | 0 | word、since、source 三必填；化格归一 exit 0；canonical 测试绿 |
| facet 标定 | temp_probe export/score | 0/0 | 席位当日基线本席自跑，判定可用，体温 0.0 |
| facet 出题回填计分 | measure.py 三段 | 0/0/0 | 九发 9/9 comply、变卦 0%、闸判 stable_clear |
| 得一三步 | attractor check/verify/sign | 0/0/0 | 裁决通过；verify identical；终签落链 f76b7ed2 |
| 例扫 | rev3 双跑＋cmp×4＋checkmath | 1/1/0×4/0 | 双跑一致 2 findings（lit_gone 2），checkmath zero_drift 红零灰六 |
| 温故 recall | retriever recall | 0 | 5 命中入 materials |
| 书单对表 | recall＋checkcite | 0/1→0 | 首跑 fail 红证入 materials；换载体检索词后 pass missing 空 |
| 认证 | scribe append 五笔 | 0×5 | 见认证节 |

## 二、排队候位与并行对表

在途 regulamath-solo（会话 49a165708effaf5e）开工时仍持链 exclusive 与 worktrees 独占锁，scribe 链闸与 lease open 预检均会拦本批，按委外令「撞锁即排队如实呈报」以锁台账轮询候位（23:08 起，零写入零触碰 sih-math），23:18 对方收约放锁后开工。候位窗内只做只读准备（上游设计稿、孵化纪律、登记形先例、SPEC-021、五红侦察）与只读启动项（watch、泊界心跳）。全程零 sih-math 写入，零锁冲突。

## 三、孵化立项登记件摘要

登记件落 sih-tools/incubation/regula-line-tools-2026-09-06.md（带日期不可变文件名，零改名面），形照化格 INCUBATION.md 先例裁剪。三工具各一节即外切判定（判据四条逐条对表全过）、本质段草案（一句话定义、不变职责、可变扩展、边界逐条、认识论状态已裁待裁分列）、契约草案（机器形态、三值退出码冻结、报告 JSON 形、锁步语义、D-4 必携位、CLI 最小形）、验收判据（逐条机械可验带检验程序与期望退出码）。命名次序裁定节承工作名先行与连带改写面明示。待裁注记节只注记六项不预判即 pk-061、pk-062、pk-064、名脉、译段工具、pk-069。契约与判据内容全部源自 regula-design-draft-2026-09-06.md §二§三§四§八§九与 SPEC-021，零设计变更。三工作名按懒波登记入检词 lazy 台账。

## 四、一裁 m-toolincub-contract-1

命题单锚即可验证性：契约条款与验收判据逐条只问能不能机械验。facet 合同模式：标定本席自跑（ZCode:GLM-5.3-Flash:self-reported，四命题二十发，判定可用）→ 出题合同（零 LLM 零网络）→ 同席回填九发（谱系披露双声明见命题件 authored 行即同席起草与同席采样、判对席有利方向即成本加重）→ 计分九发全 comply、basis_regulation 全 baseline_4、变卦 0%、谨慎信号 0/9、闸判 stable_clear、飞轮 9 行落工地 DES。得一三步（引擎件正典）：check R1 至 R7 全过裁决通过、verify identical、sign 落链 event f76b7ed2b009bbd0（event_id 6bc06f90），重放锚 proposition/DES/m-toolincub-contract-1/m-toolincub-contract-1-signcheck.json。

## 五、完成度表

| 任务包项 | 结果 | 证据 |
|---|---|---|
| Cluster 1 前置读数 | 完成 | §一前五行 |
| Cluster 2 登记件成文 | 完成 | 登记件＋lazy 三词条＋管线绿 |
| Cluster 3 facet 与得一 | 完成 | §四，stable_clear 终签落链 |
| Cluster 4 管线与结算 | 完成 | 本档＋双仓 settle＋收约对账 |
| 收约后确认串行下一批 | 完成 | redkeep-solo 已开工（同令串行） |

## 六、F 验证表

| F 锚定 | 判据 | 结果 |
|---|---|---|
| F-1 登记件落位 | 在位、三节齐、判据机械可验 | 过：sih-tools/incubation/regula-line-tools-2026-09-06.md，三工具节形齐，验收判据逐条带检验程序与期望退出码 |
| F-2 终签在链 | stable_clear 入当日链 verify valid | 过：f76b7ed2，signcheck 在档 |
| F-3 写入仅 allow | 仅请求写入节所列，零 sih-math 写入 | 过：allow 十二径全锁定，sih-math 只读（例扫两脚本 stdout 输出落本批 materials） |
| F-4 委外不越权 | 零代码改动、零人节点代行、零 plain commit、待裁只注记、在泊件零触碰 | 过：登记件纯新增文档；待裁六项只注记；watch 无主清单呈报未代清 |
| F-5 并行无撞 | 零锁冲突、候位如实呈报 | 过：§二排队候位实录，真分叉无 |

## 七、认证清单

五笔 scribe append 全 exit 0，会话闸三双带参逐笔验证：ask3 记录 548b8608、ask3 验证 b94b2f2f、正身 91f3ea51、管线报告 ffbf0569、计分材料 94f06dfe。settle cert 取首笔前八位 548b8608。

## 八、批材料清单（落 sih/event/plan/toolincub-solo-materials/）

first-run-nomenclator-deadban.json（检词首跑红证）、first-run-checkcite-fail.json（书单对表首跑红证）、scrutinator-des001-domain2.json、formatter-exit0.json、nomenclator-final-exit0.json、pipeline-report.json、changed-files.json、checkcite-registration.json（pass）、recall-topic-2026-09-06.json（温故底稿）、exscan-first-run-2026-09-06/（例扫双跑四路比对与 checkmath 读数）。

## 九、误差与越线申报

- 排队候位 10 分钟：在途数学批独占链与 worktrees，候位期间零写入（非越线，委外令预设通道，如实呈报）。
- 检词首跑 2 笔死档词：笔在核前管线首跑违例红证归档入 materials（词面见红证件），改「收录」后复跑零违例，判据零改动（承先红留痕纪律）。
- 书单对表首跑 fail：检索词未命中载体重跑后 pass，红证归档（承先红留痕纪律）。
- 例扫读数 rev3 退出码 1/1（双跑逐字节一致）报 2 findings 即 lit_gone 2，checkmath zero_drift 红零灰六：红零不过停批线，灰项属数学仓字面层声明滞后与数学批归并后变化，非本批处置面（零 sih-math 写入），如实转述候数学线复核。
- watch 对表无主清单 1 件：sih-tools/lease/.sih-tools/scribe/reports/2026-09-06-ask3-billwire-live-test-record.json（billwire 实测残件，未跟踪），按零 token 二值协议呈人节点裁决，本批零代行零触碰。
- score 计分走围堰 measure.py（双模并存采样腿），check、verify、sign 走引擎 attractor 正典，采样与判定两腿分工如实申报。
- 标定台账两行：首跑席位串 ZCode:GLM-5.3 与出题席位串对齐修正后补跑 ZCode:GLM-5.3-Flash 一行，append-only 台账两行均保留，基线取尾行。

## 十、收口读数

链 verify 与 reconcile 双仓读数见完工报告（收约后实跑）；本档 settle 于认证五笔之后提交，cert 548b8608。
