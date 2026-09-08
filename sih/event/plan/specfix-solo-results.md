# specfix-solo 批结果档

## 概览 {#overview}

- 使命：SPEC-021 与 SPEC-022 各补内容充分性节消主树核阅存量红两笔，管线全绿收约::[使命达成形](#mission)
- 停批与复链：检词登记债停批候裁，人节点裁二清债后复链全绿收约::[停批复链](#relink)
- 误差与越线申报：materials 主树直写归位、checkcite 三跑形态、核阅首跑红留痕::[误差申报](#deviation)

## 使命达成形 {#mission}

- 两档文末各增内容充分性节（模板三对表行，内容全切片自两档正文逐行给节锚），既有节零语义改动，diff 纯追加验证在案
- 两档核阅终绿 rc=0 findings 0，化格双 0，检词复链后双 0
- 主树 docmath 存量红两笔（mcpspec-solo 批 first-run-2026-09-09 双跑 cmp 逐字节一致红证源头）消解：认证前 gate 读数如实落档（主树缺节态红两笔双跑一致），收约归并后 gate 双跑绿复证见结算读数节
- 途中按格式归一不改义豁免面修复的既有节存量核阅违规：SPEC-021 116 行粗体与全角括号注改顿号串；SPEC-022 3 至 4 行块引用转普通段与 49 行两处括号注改即字；首跑红全量留痕 materials

## 停批复链 {#relink}

- 停点：管线三步检词腿，SPEC-021 检词 rc=1 一笔「基线管」lazy_in_doc（toolincub-solo 批懒波登记引入的后置登记债，gvec-v2-serial 批成文时检词 0 在链可证），三修复通道全越权即改词违零语义改动铁律、lazy.json 加 exempt 不在 allow 面且契约未正典化、升格立名超批职能，按失败即停停批，停批件 materials/stop-report-2026-09-09.md
- 人节点裁二：先清债后复链；lazyclear-solo 批清偿完毕：「基线管」按检词登记册自身契约升格 established（lazy 区条目消化，terms.json 增 proper 条目 code basemgr，prior_state lazy），双仓已 settle（tools 归并 600e314f、engine 归并 f05b1cb）
- 复链程序：lease heartbeat 刷会话与十锁（lock_rows 10 rc=0）、化格复跑双 0、核阅复跑双 rc=0、检词复跑双 rc=0（读数件 materials/nom-rec链 两件与 scrut-rec链 两件）、认证上链续走

## 误差申报 {#deviation}

- F-1 materials 主树直写：停批期间读数件落主树 sih/event/plan/specfix-solo-materials/（停批可见性需要），复链后复制工地随批提交归并，主树同件同内容由 lease 1.27 机械处置，偏差如实申报
- F-2 checkcite 三跑形态：首跑 fail 留痕（merged 本体对表，书单缺 LIM-007 与 PROB-010 系 recall 缺省语义 K=3 形态 word 命中不入书单的材料缺陷，加 SPEC-015 工程引用与 SPEC-022 档名自名两笔非数学 token）；第二跑 fail 留痕（--word 词面回退正形重召书单 168 条，九数学引用全盖，余两笔非数学 token）；终形 pass（引用面对表 mcpspec-solo 先例同款，cited-face 申报九数学 ID，missing 空 rc=0）；三跑读数件全落 scribe/reports 留痕
- F-3 核阅首跑红三笔与四笔：SPEC-021 首跑 C006 与 F003（116 行存量违规，主树基线复跑同红证实系存量非本批引入）加 N002（新增节行内锚链非行尾形）；SPEC-022 首跑 C006 两笔与 F002 两笔（存量，主树基线同红）；修复后终绿

## 结算读数 {#settlement}

- 双仓 settle 提交号：tools 412ea560（cert 74c56ce3）、engine de93039（cert 74c56ce3）；归并提交号：tools 79a0dff2、engine f63f559；closeguard pre-close 提交 tools aedcf6fd 与 engine 336d706 系 close 工具自动活面提交如实呈报
- 收约补笔前主树复算：两档核阅 rc=0 findings 0、两档检词 rc=0 findings 0
- docmath 闸门归并后双跑绿复证：run1 与 run2 exit 0 双 0，cmp 逐字节一致，verdict green violations 0，读数落 materials/post-merge-2026-09-09/，认证前主树缺节态红两笔读数存 materials/first-run-2026-09-09/ 先红留痕
- reconcile 双仓：unrouted 0 与 cert_missing 0 双零达成；tools 侧 reconcile 退出码 1 系 unbypassed 既有类计数在案（含本批 closeguard pre-close 一笔同 parksi 与 toolhyg 同形），如实转述候人节点
- 链 verify：status valid，133 笔，first_hash 25989caf，last_hash bdea246b
- 收约补笔：本节回填走直改链笔（会话 8daf996a94e659e9 已收约吊销，--no-session-reason 事由在链），补笔后化格与检词复跑俱绿
