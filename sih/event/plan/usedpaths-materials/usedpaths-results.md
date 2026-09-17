# usedpaths 批结果档

## 验收判词 {#verdict}

- 新套件 `tests/lease_usedpaths_penalty.rs` 六件绿：过罚退罚主路径、allow 内未写转罚钉形、目录前缀覆盖、斜杠归一、非 git 回退如实标注俱绿；主会移植后工地复跑 6 passed 复证。
- 回执面零扰动：lease_mergeback t2 close 回执金向量逐字节一致，罚金口径改形不碰回执 schema。
- 改动面：`src/bin/lease/closegate.rs` +132/-9 加新测试文件，与任务包声明面一致。

## 偏差申报 {#deviations}

- 口径钉形从正典：罚 = 锁面且未被实改集覆盖（faceprecise-analysis §2.5 与 §2.3 类三转罚面）；任务文案矛盾例已弃，若人节点改裁翻转 entry_covered 一处即可。
- 引擎位无开工计费面（open_face_bill 仅冻结围堰），path_count 去重留解冻批；rstrip 归一已施于引擎位全部集合运算。
- 取样点前移归并前：罚金块原位在删支拆本后分支已灭不可 diff，行为等价时序必需。
- 会话事故披露：首会话 b2a65416071e1f06 因主会先拆工地致 open 空正身空意图签发，即开即废（作废链两笔 95d18d30 与 31b39f27 加撤销行），同时暴露引擎 open 对 identity 与 intent 件缺席零校验即空哈希签发的缺陷，候另批修复。

## 处置记录 {#dispositions}

承 2026-09-18 用户令「全都推进，你是主编排，调用多子代理进行并行操作」候令簿第二件，承载 pk-074 faceprecise 出泊裁定实装面。子代理施工、主会结算。本批 close 即吃新罚口径（dogfood），收约读数随批在案。
