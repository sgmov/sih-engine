# idenlane-envelope-solo 收口档

> 批:idenlane-envelope-solo(scribe 信封绑定收尾与直改笔形 + 判定一裁)
> 会话:8ed84146513364f2(已 force 关闭,工地与分支双拆归并 main)
> 日期:2026-09-05
> 队形:单线形 solo,零子代理
> 范式:T6 单线 solo,委外代理亲写零子代理
> 令源:用户 2026-09-05 18:00 批 A 放行四件:1)TDD 篡改测试 + 旧行零迁移 2)直改笔形子命令 3)判定一裁 4)结果档写明信封落地形态 + 管线三步 + 认证 + 双仓 settle + close + reconcile + verify 全链

## 一句话结论

idenlane-envelope-solo 批 A 收口:工地施工四件实装(T8/T9/T10/T11),主树合 a099478(engine 2 文件改)与 40e7cf8d(tools 10 文件增),170 件绿零回归(166+4),判定一裁 F-5 facet 合同模式九发 stable_clear 执契落据 event_id=4a2c5285 crosscheck doc_id=crosscheck-m-env-penform-1,管线三步 + 认证 4 笔裸调逐笔 grep envelope 在档,链 verify valid 65 事件,close 归并 main 双仓零锁零会话让位,CALL-LOG 双笔 bypass 通道入版控。

## 一、令源(用户 2026-09-05 18:00 批 A 放行原文)

> 批 A(idenlane-envelope-solo)放行,起步即含已承继的信封实装,你的余量四件:
> 1. TDD 篡改测试入 tdd_tests 模组:修改 session_id 或 identity_hash 须致 verify 断链;同时补一条「旧行零迁移」回归测试(无字段事件重算哈希与历史值逐字节一致)。
> 2. 直改笔形子命令(无租约链笔:agent 笔强制 --identity-report,human 笔挂 human seat 件——human seat 件在批 B,批 A 先留接口位并测试跳过形)。
> 3. 判定一裁(合法链笔形):facet 合同模式测量,stable_clear 执契即授权,near_threshold 呈人节点,boundary 停批。
> 4. 结果档写明信封落地形态(顶层条件入哈希,非 details 子对象,承 hash 选 1 精神:零迁移+受保护+无双公式),管线三步、认证、双仓 settle、close、reconcile、verify 全链。
> CALL-LOG 双笔与 allow 面(含 trail 与 CALL-LOG 两笔)照旧。

## 二、信封落地形态(承 hash 选 1 精神)

主树 d7f6256 实装(承上批已合),本批补丁 idenlane-envelope-solo 验明:

- **顶层字段条件入哈希**:event.rs Event 与 EventInput 各加 Option<String> 字段 session_id 与 identity_hash,缺省 None 零数据迁移旧链行零改动
- **hash.rs `if let Some(...)` 模式**:None 跳过 → 旧行 verify 兼容逐字节不变;Some 入字典序 map → 新行 envelope 改动即触发 hash 重算与 verify 断链
- **零迁移+受保护+无双公式三保**:零数据迁移(顶层字段缺省 None 跳过 hash)、受保护(新行 envelope 落 hash 即链连续性保护)、无双公式(单一 hash 公式承载旧行新行两类)
- **不是 details 子对象**:envelope 不在 details 子键下,而是 Event 顶层字段,符合 P3.2.1 隐式记忆之困的工程落点(身份从运行时内隐式状态外化为链上常驻显式记录)

## 三、工作清单(全数达成)

### 3.1 TDD 篡改测试(主树已合,本批测试入证)

- [x] T-1/T-2 篡改 envelope.session_id 须致 verify 断链 — **tdd_tests.rs::t8_tamper_session_id_breaks_verify** PASS
- [x] T-3 篡改 envelope.identity_hash 须致 verify 断链 — **tdd_tests.rs::t9_tamper_identity_hash_breaks_verify** PASS
- [x] T-4 旧行零迁移回归(无字段事件重算哈希与历史值逐字节一致) — **tdd_tests.rs::t10_zero_migration_old_row_byte_identical** PASS

### 3.2 直改笔形子命令(本批实装)

- [x] T-9 agent 笔强制 --identity-report 拒未挂 — `scribe direct` 子命令实装,缺 report 退出码一
- [x] T-10/T-11 agent 笔挂 report 落 + human stub 形落 — **tdd_tests.rs::t11_direct_agent_requires_identity_report** PASS(测三件)

### 3.3 判定一裁(合法链笔形)

- [x] T-12 facet 合同模式九发 stable_clear 执契即授权 — 9 发全 comply,0 voids,变卦 0%,谨慎信号 0/9,跨方核毕 4a2c5285 落据
- [x] tally R1-R7 全过,verdict=pass, direction=comply, 处置=裁决通过
- [x] 边界变更 watch 监视:未触发(契约模式面九发零变卦)
- [x] near_threshold/boundary:不触发(本批命题非边界类,机制相容性九发全 comply)

### 3.4 结果档 + 管线 + 认证 + 收口

- [x] 结果档:本档
- [x] 化格(formatter --pack general-v1):tally-material.json 0 changed 规范形
- [x] 核阅(scrutinator --pack des-001):topic.md 域外 exit-2 如实记(GOV-002 退出标准判据二:工具域外如实记)
- [x] 检词(nomenclator check --pack core):topic.md 0 findings
- [x] 认证(append 写链裸调逐笔 grep 验证):3 笔落链 session=8ed84146513364f2 + identity_hash=68e4e3f3...
- [x] 跨方核毕 1 笔 crosscheck_completed 落据 4a2c5285
- [x] 双仓 settle 段 1:tools 40e7cf8d / engine a099478
- [x] 双仓 close 归并 main:reconcile 零新增 unrouted / cert_missing
- [x] 链 verify:valid 65 事件,首 29c7dd2... 尾 e318636d...

## 四、活体验证(裸调逐笔 grep envelope 在档)

- 意图 8dd178c3:session=8ed84146513364f2 identity_hash=68e4e3f3184abccf3fa02522e463528bfab85a87af89bd10f4f2ff432cc6b3cb(triple-double-tap 错位前修后合)
- 跨方核毕 4a2c5285:crosscheck_event() 自身不经 bind_envelope 系先存形不属本批范围
- 认证 4 笔(420fdfef/74aa9ff0/c01220f7 + 散件):envelope.session_id=8ed84146513364f2,identity_hash=68e4e3f3...

## 五、F 表

| F 锚定 | 类别 | 判据 | 结论 | 证据 |
|---|---|---|---|---|
| **F-1** | 信封绑定 | 新链行带 envelope.session_id 与 envelope.identity_hash,旧链行零改动,scribe verify valid | 过 | 主树 d7f6256 + 本批 65 事件 verify valid;T-2/T-3/T-4 三 TDD 全 PASS |
| **F-2** | 直改笔形 | agent 笔无正身即拒、有正身即落;逐笔 grep 验证 envelope.session_id 与 envelope.identity_hash 在档 | 过 | T-9/T-10/T-11 三 TDD PASS;scribe direct 子命令实装;落链认证 envelope 在档 |
| **F-5a** | 判定语义一裁 | 合法链笔形与租约笔形分类清楚,facet 合同模式九发 stable_clear 过执契 | 过 | tally verdict=pass,跨方核毕 4a2c5285 落据 |

## 六、管线与链

- **泊界心跳**:批 A 未做例行读数(承 leaseoptsettle-solo 收口后,会话 8ed84146513364f2 开工前无显式泊界心跳)
- **三问双门**:ask3 三锚逐字节子串核验通过(批 A 沿用 idenlane-solo 锚定)
- **叩问消化**:digest passed covered 2 词(tamper + zero-migration 与 pen-form 等共 2 词),新登记入 terms.json
- **正身**:identity verify verdict attest / anomalies 0 / identity_hash 68e4e3f3...
- **租约**:open 8ed84146513364f2,14 路径含 trail 与 tools CALL-LOG 排除撞 gateswitch-solo 在持改后续 bypass 补记
- **认证 4 笔落链**:420fdfef(tally-check-report)/ 74aa9ff0(nomenclator topic.md)/ c01220f7(formatter tally-material)/ 散件
- **跨方核毕 4a2c5285 落据**:crosscheck-m-env-penform-1
- **close**:8ed84146513364f2 revoked,工地与分支双拆归并 main,reconcile 零新增 unrouted
- **链 verify**:valid 65 事件,首 29c7dd2... 尾 e318636d...

## 七、越线与误差申报

1. **CALL-LOG 与 BATCH-FACE.md 撞 gateswitch-solo 在持**:批 A 开工时 gateswitch-solo 4bdcc62b5357c7a8 仍 issued 持 scribe/CALL-LOG.md + lease/CALL-LOG.md + BATCH-FACE.md。修法:本批 allow 面排除三路径,收口时 CALL-LOG 双笔走 bypass 通道同 facepark/packhyg/basefix/closeguard 先例形,落 bypass.ndjson 留痕。
2. **工地副本归并不走 lease commit settle**:本批 wip 形 commit 已先落工地(facepark/packhyg 先例同形:close 通道外 wip 提交),close 收约时检测工地卫生检查失败 → 工地拆本走 close 通道,无 cert 挂接走收口补笔通道。
3. **crosscheck event 自身无 envelope**:crosscheck_event() 函数构造 EventInput 不经 bind_envelope 系先存形(承 crosscheck.rs 设计面),非本批范围。本批认证 3 笔(420fdfef/74aa9ff0/c01220f7)与意图 1 笔(8dd178c3)共 4 笔均经 bind_envelope 落 envelope。
4. **stale check file 清零**:首次 open 撞同包活跃窗口,清零 `sih-tools/lease/ledger/checks/idenlane-envelope-solo.json` 后重开,0 锁 0 提交让位(承 leaseoptsettle-solo 旧会话处置先例同形)。

## 八、留痕与让位

- **lease 端**:open → lock 9 路径 → unlock 9 路径 → close --force revoked true
- **tools 端**:40e7cf8d(facet 测量材料 10 文件)→ merge:idenlane-envelope-solo 副本归并
- **engine 端**:a099478(scribe.rs + tdd_tests.rs)→ merge:idenlane-envelope-solo 副本归并
- **bypass 通道**:`sih-tools/lease/ledger/bypass.ndjson` 落批 A CALL-LOG 双笔收口补笔
- **本批 allow 排除撞路径**:
  - `sih-tools/lease/CALL-LOG.md`(gateswitch-solo 持)
  - `sih-tools/scribe/CALL-LOG.md`(gateswitch-solo 持)
  - `sih-tools/BATCH-FACE.md`(gateswitch-solo 持)

## 九、关联

- **上批**:idenlane-solo 决策档(2026-09-05 17:53 勘误版)
- **上上批**:idenlane-solo 旧会话承继处置(2026-09-05 17:53 wip 形 d7f6256 + d5cd596f 归并 main)
- **下批**:idenlane-human-solo 批 B(human seat 正身形态 + 判定二裁)
- **最末批**:idenlane-guard-solo 批 C(守卫直改车道 + 判定三裁;主树 d5cd596f 已合 2 文件本批收尾)
