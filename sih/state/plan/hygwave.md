# hygwave：泊界材料卫生加 watchcheck 台账卫生两件

> 令源：用户 2026-09-18 令「那你调2个子代理，把2波都做了」第二波承载
> 病灶出处：材料侧线冗余系判据扫泊界路由长期读数；重复 acquire 系 faceprecise-analysis.md §1.4 申报
> stem 认领：hygwave，甲表三件即 zh 泊界材料与台账卫生波、code 无承、派生 hygwave:new

## 问题陈述 {#problem}

两件卫生债：一即泊界材料面堆积致心跳路由常报侧线冗余告警（引擎线 surplus 12 加工具线 4，阈值 2），侧线 16 加 4 与废件道 23 加 1 待逐件诊断归位；二即 locks 台账历史会话存在同路径重复 acquire（idwire-solo 28d326f924916cd0 一会话 123 笔仅 11 路径），哨兵字典语义无害但台账卫生无人盯。

## 关键设计 {#design}

材料件保守序处置：selector route 包语义为正典逐件诊断，出泊已结材料归位或补元数据回主线，真死件入废件道，零删除，拿不准留候裁；PARKING-v1.md 名册零触碰。watchcheck 加冗余重复 acquire 卫生读数：回放统计同 path 加 session 重复 acquired，阈值常数 >3 呈报不代裁，输出向后兼容可选键，引擎位实装围堰只读。

## 工作清单 {#work}

- [ ] hw-01：材料逐件诊断与保守归位
- [ ] hw-02：两目录路由告警清零验收与卫生报告
- [ ] hw-03：watchcheck 冗余 acquire 读数红转绿
- [ ] hw-04：真台账验证加 settle 加 close 加结果档

## 验收 {#acceptance}

两目录 selector route 告警清零或残留如实申报；watchcheck 新读数红转绿加真台账 idwire-solo 在列；既有读数向后兼容；名册零改；当日链 valid；reconcile 零新增。

## 请求写入 {#requested-writes}

- sih-engine/sih/state/parking/materials/
- sih-engine/src/bin/watchcheck.rs
- sih-engine/tests/（新测试文件）
- sih-engine/sih/event/plan/hygwave-materials/
- sih-tools/parking/materials/
