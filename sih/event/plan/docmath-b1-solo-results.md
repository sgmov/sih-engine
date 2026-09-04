# docmath-b1-solo 结果档：规则分拣账本批

> 批：docmath-b1-solo（docmath 程序批一，纯脚本零行为变更）
> 会话：5e29dd1e50e64b14
> 日期：2026-09-04
> 队形：单线形 solo，零子代理

## 概览 {#overview}

- 三族规则包全部规则机械枚举 467 件入分拣账本：核阅 des-001 十五条、化格 general-v1 一条、检词 core 登记面四百五十一件（manifest 域声明面一加 terms 一百七十加 candidates 二十一加 lazy 七加 dead 二百五十二）
- 分拣读数：判定性四百三十八（已挂锚与已实例化四百三十四、候选四）、纯约定八、待确认二十一
- 双跑复算逐字节一致，面一致性三查全过，零人工填项
- 判定性候选锚四件：S002 与 S006（ALG-012）、S004（ALG-012 加 ORD-008）、N002（ALG-012 加 ORD-008），批三逐条挂锚的输入
- 纯约定八件入环境参数登记面 env-params.json：C001 与 C002 与 C006 与 S005 与 F000 与 F002 与 F003 与 F005，判定性扰动缺席只改观感如实标注为约定
- 待确认二十一件全数如实申报：检词 candidates 面词条，判定位消费面零检出（check 与 elicit 均不消费），缺省从严判判定性挂候选锚位 ALG-002 加 ORD-024

## 分拣判据声明 {#criteria}

- 包内注记已携载体引用与推导档指针的规则判已挂锚：注释块点名规则号或紧邻注记块邻近归属，载体 ID 与推导档路径机械提取，命中 C007 与 C008 与 C009（锚 ORD-008，推导档 gatecap-derivation-2026-09-04.md 与命名族声明）
- header_structure 结构查项与 nav_format 判定性，候选锚 ALG-012，锚点与导航引用加 ORD-008
- header_structure 节序查项与字符面与标点面与排版元素禁用面判纯约定，判定性扰动缺席
- 化格 line_ops 判定性候选锚 ORD-023，源码接线实存判已实例化（carrwire-solo 接线）
- 检词在册词条与死档与懒波判定性，候选锚 ALG-002 加 ORD-024，源码接线实存判已实例化
- 检词 candidates 零判定位消费面如实申报判待确认
- 未命中上述任一声明判据的规则判待确认，缺省从严判判定性

## 验收读数 {#acceptance}

- F-1 账本成立：过。四百六十七件逐件三态入账，分拣判据声明形机械准则承载零逐件手填，双跑 ledger.json 与 env-params.json 与 summary.md 三件 cmp 逐字节一致
- F-2 对表零漏项：过。枚举计数与规则包解析计数逐族相等（十五与一与四百五十一），面一致性三查全过
- F-3 登记随批：过。叩问三信号全数消解，分拣账本与纯约定与候选锚位三词经检词 register 通道登记，词表化格归一 exit 0，检词测试族三十测全绿含 canonical 形测试

## 惰性残留申报 {#residents}

- sih-tools/nomenclator/packs/core/entries-ordwire/ 五件（01-撞锁.json 至 05-金向量.json）：包装载器固定文件名装载面之外的过程归档件，装载器不消费，如实入账不删
- sih-tools/nomenclator/packs/core/.-tmp-entry-programslicing.json：零字节隐藏临时残留件，装载器不消费，如实入账，清理归后继词表卫生批

## 例扫读数 {#exscan}

- rev3 双跑退出码 1/1 同形，四路 cmp 逐字节一致；退出码一来自漏项核对线一笔发现：sih-engine/scrutinator 语料命中 ALG-002 与 ORD-019 与 PROB-010 与 PROB-011 与 TOP-008 五枚载体 ID 未入 rev3 脚本机制映射表，来源件为 sih-engine/src/scrutinator/tests.rs 测试语料，属 mathpipe 覆盖脚本声明滞后灰项，只记不代修如实转述
- checkmath 退出码零，verdict zero_drift，reds 零、greys 四（声明滞后灰项）

## 管线与认证读数 {#pipeline}

- 化格：summary.md 与章程与结果档与 ledger.json 与 env-params.json 五目标全 exit 0
- 核阅：三 md 目标均 des-001 域外 exit 2 如实记入档不属违规（des-001 域只盖 sih-engine/doc）
- 检词：三 md 目标零违例
- checkcite 书单对表：召回书单与引用件对表读数见批材料，引用 ID 全落书单并集
- 认证四笔：管线与账本与变更件与 checkcite 全 meter 包裹闸三双带参落链

## 写入面实录 {#writes}

- sih-math/docs/docmath-coverage-2026-09-04/：docmath_sort_script.py 与 ledger.json 与 env-params.json 与 summary.md
- sih-engine/sih/state/plan/docmath-b1-solo.md 与本档与批材料目录
- sih-tools/nomenclator/packs/core/terms.json：register 通道三词（零行为变更，登记面纯数据）
- sih-tools 五件 CALL-LOG 随批留痕
- 当日链：例行读数三笔与意图一笔与认证四笔
