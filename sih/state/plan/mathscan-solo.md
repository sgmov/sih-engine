# mathscan-solo：数学仓全盘扫描 rev3 与管线机械管理四核对器

- 承接：主会 2026-09-04 盘点——rev2 后十批落地（carrwire 四接线、pk037impl 语义层加 PROB-017/ALG-011、pk050sw 切换等），账面再度漂移；rev2 已知两缺陷（calculus 目录约定例外误报、165/171 别名口径）未修入扫描器。
- 日期：以开工实日为准（trail 用当日链，会话号 sess-zcode-<实日>-mathscan）｜ 温故检索：materials/recall-mathscan.json 如实记 ｜ pk-045 参与者
- 队形：单线形 solo，零子代理；单飞（sih-math 扫描面独占，工程仓零触碰）。

## 批件

- 件一 rev3 全盘扫描：rev2_script.py 出 rev3_script.py，修两已知缺陷（子仓条目目录约定枚举实态不作统一假设；双概念号别名记账规范化输出）；三态表全量重算（白名单 171 联动落 rev3 本体）；双跑逐字节一致。
- 件二 四表一致性核对器：mapping 行 ↔ INDEX 行 ↔ entry 文件 ↔ 白名单概念号，机械核对脚本（checkmath.py）随批入版控，零漂移或漂移逐件清单。
- 件三 书架层 M-4 盘点：缺可证伪节条目全量清单（批时实数约 127），**只盘点不补节**——可证伪节是数学内容，补节归载体管线批（newcarr 同款过得一）。
- 件四 跨条目定义复用对表：多消费概念（良基 ORD-011/016/023、闭包、幂等等）定义指针与复用关系逐对核对，不一致清单出档。
- 件五 例扫挂点提案（唯一裁决点，过得一后泊界呈报）：全盘扫描的例行化挂点两案——A 案挂 gauge 日扫旁（每日例行读数后同跑）、B 案挂批尾扫（每批 close 前跑）——facet 测量三态分流，呈泊界不扰人。

## F 清单

- F-1 rev3 双跑逐字节一致，三态表与源码扫描对表零漏项，白名单 171 联动落本体。
- F-2 核对器可复算，四表零漂移或漂移清单完整。
- F-3 书架层清单完整（总数对账 166+ 文件）。
- F-4 复用对表逐对在档。
- F-5 例扫挂点提案过得一三态分流在案。
- F-6 写入仅 allow：sih-math/docs/mathpipe-coverage-*/（rev3 件）、核对器与盘点清单、materials、结果档、当日链；工程仓源码零触碰。

## 管线与机械链

ask3 → 双门 → 叩问 → 正身 → lease open --package mathscan-solo → 锁（sih-math 面 exclusive；共享追加面 append 短持）→ intent 上链（闸三 --sessions 带）→ 工地施工 → 化格→核阅→检词 → 认证逐笔 → settle 前拷工地 → 双仓 settle --cert → close → reconcile → 当日链 verify。uv 调用 env -u PYTHONHOME -u PYTHONPATH；禁管道掩退出码；close 外提交 --no-verify + bypass。

## 禁区

工程仓源码零触碰；可证伪节零补写（只出清单）；rev2 件只读；主树零直写。
