# 生而绿条目起草波编排记录

> 编排主代理落盘。波次 / 派单数 / 验件数 / FAIL 数 / 修复方式。零粉饰。
> 工作对象：sih-math 五件已登记未建概念，各一稿。TOP-006 紧集 / TOP-007 连续映射 / PROB-004 大偏差原理 / PROB-005 Bayesian 更新 / ORD-005 链与反链。
> 边界铁律：只允许写 agent-drafts/entry/ 下草稿与认领行，其余一切只读。禁止入仓、跑 lease/scribe/git、改 AGENTS.md、上链。

## 波次登记

### 波 1（起 23:16，讫 23:58）

- 派单子代理：3，簇切分即拓扑簇 TOP-006 + TOP-007（2 件）、概率簇 PROB-004 + PROB-005（2 件）、序簇 ORD-005（1 件）
- 派单数（任务书下发）：3，即 TASK-A-topology.md / TASK-B-probability.md / TASK-C-order.md
- 总验件数（verify-entry.py 通过）：5 件
- 锚条数：10 条
- 逐字验过：10 条（每条引文与源文件对应行 grep -F 逐字节比对命中，行号复核一致）
- FAIL 数：0（主线全量首跑零失败）
- 修复方式：零主线修复。ORD-005 子代理初稿 12 处全角括号违例（front matter pro 标签与正文混用），子代理交稿前自查自修，主线交付时定向复核确认零残留
- 备注：
  - 锚点行号在派单前由主线 grep -n 预验，任务书直接给出已验命中行，故 10 条锚首跑全过，无一换行
  - PROB-004 锚二（08-on-settle.md:102）引文含直引号，front matter 内按 YAML 转义，yaml.safe_load 解析通过，逐字节比对以解析后文本执行
  - 认领文件按簇分件（CLAIMS-A/B/C-*.ndjson），无共享文件竞态
  - 子代理只读上报的存量仓内不一致（不入本波 FAIL）：TOP-004 条目 L17 指针悬空；calculus INDEX 标 LIM-008 已建而 entries 目录当日由 calculus 波补建
  - 未逐源核验项（子代理如实标注）：参考文献页码与历史年份按标准史实书写；Bernstein-von Mises 年份有文献分歧，PROB-005 采用范本既有 1920s 写法回避
  - 状态行不一致待入库裁量：TOP 两件与 ORD 件写「状态：草稿，哲学到工程桥梁条目。」，PROB 两件写「状态：已建，哲学到工程桥梁条目。」，草稿阶段按何态归属由主线入库时裁定
  - 内容验收（主线串行）：数学内容逐件通读，跨子仓引用公式号三件（eq:prob001-wlln-limit / eq:prob003-clt-limit / eq:ord002-complete-lattice）全部解析到已建条目实体，无悬空引用
