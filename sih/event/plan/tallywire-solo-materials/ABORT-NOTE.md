# tallywire-solo 裁撤申报

- 裁撤令源：主会 2026-09-04。两因：一，执行子代理死于模型请求失败（基础设施故障非治理故障），中断于施工期，三仓工地零源码提交；二，同日并行批 tallywire2-solo（用户窗口）已完成 tally 双载体接线（ORD-006 闭包与 ORD-011 良基，结果档 sih-engine/sih/event/plan/tallywire2-solo-results.md），本批目标被覆盖，续跑即冗余。
- 数据面痕迹：意图笔 6c932104 在链为唯一写入（intent_refined），历史不改写；四笔 certification_completed 与 tallywire2 及 selwire 意图笔同日邻位，非本批产物。
- 残局处置：六锁（materials 与 results 与 trail 与任务包与推导档与 tally 施工面）经 lease unlock 逐把释放；lease close --force 正式收口，会话 3249c546dd2ddb8b 吊销，三仓工地（msh/tallywire-solo）删除；主树未跟踪批件（任务包与本 materials 目录）随本申报一笔入版控。
- 经验一条：并行窗口同抢程序批序时，主会派批前应先扫当日链 intent 面查重——本批若先查 tallywire2 意图 7415d87e 即不会派出。登记入 pk-045 样本库候选类（跨窗口批序撞车）。
