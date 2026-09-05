# regula-rename-solo：文规正名称谓并行标注批

> task-packages 治理任务，承立名批（docmath-namefit-solo，机器终签 crosscheck-m-wengui-fit-1）第三段连带改写清单与用户 2026-09-05 多选令「闸门脚本、BATCH-FACE、结果档里的 docmath 称谓改文规/Regula 并行标注（历史件不动）」
> 队形：单线形 solo，零子代理
> 日期：2026-09-05

## 概览 {#overview}

- 批性质：称谓并行标注批，纯注释与称谓替换，零行为变更
- 标注面：合并闸门脚本（docstring 与报文并行标注正名文规加 en Regula 加代号 regula）、BATCH-FACE 闸门调用形节、立名批结果档、docmath 程序立项文档启动记录补正名落定注记
- 历史件不动：docmath 四批台账行与结果档与脚本文件名（docmath_gate_script.py 等）与 docmath-coverage 目录命名均承旧称为既成事实，血统注记不改路径
- 并行标注形：docmath 称谓处加「正名文规，en Regula，代号 regula」注记，不删旧称

## 一、问题陈述 {#problem}

立名已落而称谓未随：在役闸门与登记面仍单称工作名 docmath，正名文规与代号 regula 无在役现身位。本批把正名以并行标注形落进向前件，使称谓可查可溯，历史件保持原样。

## 二、关键设计 {#design}

- 并行标注不替换：旧称 docmath 全部保留，正名以括注与定义行形式并现，避免路径与引用断裂
- 血统注记：闸门脚本报文与 BATCH-FACE 节头加正名行，承立名批机器终签为准
- 程序立项文档启动记录补一行：正名落定注记（文规与 Regula 加代号 regula，承机器终签重放锚），不改历史叙述

## 三、工作清单 {#work}

- [ ] 四标注面逐件标注
- [ ] 化格检词核阅管线（闸门脚本 py 域外如实记）
- [ ] 认证上链三仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| F-1 标注齐 | 治理 | 四标注面逐件含正名并行注记，历史件 diff 零触碰 |
| F-2 零行为变更 | 工程 | 闸门脚本改注后双跑逐字节一致，goldens 零漂移 |
| F-3 管线 | 治理 | 化格与检词过，域外如实记档 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/proposition/topics/2026-09-05-wengui-naming-review.md（正名与代号三件套）
- 必读 2：sih-tools/BATCH-FACE.md 闸门调用形节

## 六、约束 {#constraints}

1. 历史件不动：四批台账与结果档与脚本文件名与目录命名零改写
2. 闸门脚本只改注记与报文文字，判定逻辑零触碰
3. 主树零直写（收约预检双面对表同批次内），词债无新增
4. 上链遇锁即等待不绕行

## 七、请求写入 {#requested-writes}

- sih-math/docs/docmath-coverage-2026-09-04/docmath_gate_script.py（注记）
- sih-tools/BATCH-FACE.md（标注）
- sih-engine/sih/event/plan/docmath-namefit-solo-results.md（标注）
- sih-engine/sih/event/plan/docmath-full-program-v1.md（启动记录注记）
- sih-engine/sih/state/plan/regula-rename-solo.md 与 sih-engine/sih/event/plan/regula-rename-solo-results.md 与批材料目录
- sih-engine/sih/event/trail/2026-09-05.ndjson 与 sih-tools/scribe/reports/ 与 identity/reports/ 与 meter/counts/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，三仓结算收约，对表读数在档
