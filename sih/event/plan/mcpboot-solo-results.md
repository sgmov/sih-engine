# mcpboot-solo 批结果档：MCP 引入初始化固化——bootstrap／域自举全链命令与台面开域视图

> 批名 mcpboot-solo · 会话 4945cab007f323e7 · 2026-09-10 · 单线形 solo
> 令源：用户 2026-09-10 令「现在司梦引入mcp的初始化还是手工在做，需要固化成司衡引擎的一个固定工具。」承同日三裁定与立名裁决（详见任务包与 DES-015 修订四）

## 一、落地面

- **新模块** `sih-tools/mcpline/src/mcpline/bootstrap.py`：全链五段编排（前置检查、签发位、开域位、镜像位、客户端注册旗标选入位）。开域本体零重实现（复用 init.precheck 与 init.open_domain）；登记册写点唯一经 tokens_mod.append_row；客户端配置件形先验随段一（畸形件在签发与开域前即拒，全链零写入可修复重跑），写仍在验域后（合并写仅设 mcp.servers.sih 固定 payload，其余键保留，幂等）。出参严格 JSON 单对象；退出码循 init 约定 0/1/2。
- **台面开域视图** `web.py`：/tokens 台面增开域表单与 /tokens/open、/tokens/confirm-open 两路由，两步确认循签发先例；薄壳调 bootstrap.run_chain 单一 canonical 路径零二次实现；确认步零写入断言在测试。DES-015 修订三候裁项「开域进视图位」承裁关闭。
- **init.py**：仅模块 docstring 尾加全链编排位指针申报一行，核心零改动（红线二对表）。
- **版本**：mcpline 0.6.0 升 0.7.0（pyproject 与 __init__ 双点同步，description 扩）。
- **检词正登**：bootstrap／域自举 established 入 packs/core/terms.json（registers/2026-09-10-bootstrap.json 在档；首跑缺 state 字段被登记前校验拒一笔，补 state 后过四查；懒波零冲突免升格；pack canonical 测试绿）。
- **DES-015 修订四**：五点在档（全链固化形、台面视图位、init 窄口零改动、语汇判词、版本），管线读数见下节。

## 二、测试读数

- 首跑全套（工地，持锁窗）：**96 passed / 14 failed**，14 笔全数落在 `test_write_gates.py` 一族——实因：该族测试真跑 lease open，其根锚发现回落读真工作区锁台账，与本批在飞 20 锁互撞（主树同测试同红对照在档，判环境干扰非码病，test_commit_wip_passthrough 主树复红实录在 CALL-LOG 可查）。绿读数以收约归并后主树复跑为准（projfix F-1 主树真跑先例），读数入完工报告与 CALL-LOG。
- 本批新增件（工地）：test_bootstrap_domain.py 七件 B-1..B-7 全绿、test_token_console.py 增开域视图两件全绿（合计 16 passed）。
- 零写守卫：bootstrap 写面申报核新测试绿（append_row 单点、write_text 单点、init 复用断言）；套件窗口双仓 git status 全等断言绿。

## 三、管线读数（DES-015 修订四）

- 化格：exit 0（两跑俱零改）。
- 核阅：r1 **exit 1**（C006 全角括号九笔，本批修订四行，红证在档 scrutinator-des015-rev4.json）；重写去括号后 r2 **exit 0** 零 findings。
- 检词：exit 1，五笔 lazy_in_doc——四笔存量（132 行修订三已认证文本）加一笔本批承典用词，处置如实转述候立名程序人节点裁（disposition 在档 nomenclator-des015-rev4-disposition.md）。

## 四、先红留痕（禁清洗申报）

1. B-5 首跑红：畸形件子例复用已开域根被幂等闸先拒——暴露真设计缺陷（客户端配置畸形在段五才拒则域已开不可重跑），设计修正为形先验随段一；红证 tdd-red-b5-malformed-first-run.log 在档。
2. 核阅 r1 红：C006 九笔，红证 scrutinator-des015-rev4.json 在档。
3. 检词登记首跑红：缺 state 字段被拒，红证在 registers 登记件改前形（git 历史）与本档申报。
4. 全套 14 红（环境干扰判明过程）：主树对照复红实录。

## 五、偏差申报

1. 计划期定台面三路由（GET 表单加两 POST），实装循签发/撤销既有两路由形（表单嵌台面页），少一 GET 路由，形变更贴家族模式（任务包范式偏离节预申报在案）。
2. 管线检词 exit 1 非零（lazy 存量加承典，处置在档候人裁）——任务包 F-7「三步俱绿」按字面未达，按检词契约「存量债务如实报告处置在人」条款处置，如实申报。
3. 版本位竞争实况：des016impl 批（并行窗）先落 0.6.0，本批按任务包风险节预案顺延取 0.7.0，零冲突。

## 六、链笔与提交

- intent 笔：92b6d6966bb95393e414d34fe936145a816762c1e6566d8badf44fe640af9840（2026-09-10 引擎链）。
- 认证笔：见当日链 certification_completed 笔（管线与结果档两笔，哈希见链）。
- 双仓 settle：commit 哈希见完工报告（工具工地 msh/mcpboot-solo 归并 integral-stage-build，引擎工地 msh/mcpboot-solo 归并 main）。

## 七、关联

- DES-015 修订四（本批唯一正典修订）；DES-016/pk-084 零触碰申报（意图模板面零改动，plain/ask3 分派兼容）；司梦开域观测报告之双落位与 .zcode 实物形即本批对表证源。
