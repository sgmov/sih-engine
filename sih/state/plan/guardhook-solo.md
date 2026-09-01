# guardhook-solo：拒直提守卫实装批

> task-packages 治理任务
> 承接：用户 2026-09-02 批准令、adjudisp-solo 狗粮裁决 stable_clear 即链事件 2ada7331、DEC-021 分派令、goldfix 六笔直提病灶即 sealwin3 追认档
> 队形：单线形 solo——委外代理亲写零子代理
> 日期：2026-09-02

## 一、问题陈述 {#problem}

plain git commit 绕 lease 直提主线已四度露形（viewfix/viewrider、tdfix、ordfound、goldfix 六笔），settle 前后对表失效、归并无前缀、认证缺席，两度封窗为同一通道买单。得一裁决材料在链支持增设守卫，用户已批准开权。

## 二、关键设计 {#design}

四件。一即守卫本体：lease 增 install-hooks 子命令即向两仓写 core.hooksPath 指仓内 lease/hooks 目录，pre-commit 钩子机械校验提交信息即合规两形为「正文含 session: 与 cert: 挂接行」（lease commit wip/settle 形）或「首行为 merge: <批名> 副本归并」（收约归并形），两形皆非即拒并出指引（走 lease commit 或显式 --no-verify 留痕）；钩子文件由 lease 版控承载即 hooks 脚本入 sih-tools/lease/hooks/ 随仓版本化。二即绕行留痕：--no-verify 绕行是显式授权通道，reconcile 增检测即无模板提交在 report 里单列 bypass 类（有 no-verify 特征即尾行 Gerber-format 痕迹不可靠，改用绕行台账）——形态定为 lease 增 bypass 子命令即 --no-verify 提交后人工登记一笔 bypass.ndjson（sha 加事由加会话），reconcile 对无模板提交逐笔对台账，未登记即 unbypassed 告警类。三即自载豁免：lease 仓自身提交（lease 源码批）与守卫安装批自身走 --no-verify 加 bypass 登记即狗粮首绕。四即测试与升版：钩子逻辑纯函数化测试两形合规加三形拒（无 session、无 cert、无 merge 前缀）加 bypass 对表，lease 升 1.12.0 三源对齐与 CONTRACT 修订，双仓安装即 git config core.hooksPath 落 config（不入版控的本地配置属环境态，卸载即 uninstall-hooks 子命令）。

BATCH-FACE 两处顺带：核阅腿 --pack 裸名勘误（引擎件按内嵌名解析，$ROOT/sih-tools 路径形报未知包）与新增「直提守卫与 bypass 登记」坑位行。

## 三、工作清单 {#work}

- [ ] install-hooks 与钩子脚本与纯函数校验
- [ ] bypass 子命令与 reconcile 对表
- [ ] 测试两形合规三形拒加 bypass
- [ ] 双仓安装与首绕狗粮
- [ ] CONTRACT 修订升版与 BATCH-FACE 两处
- [ ] 收口对表

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 守卫 | 工程治理 | install-hooks 后无模板 plain commit 被拒即退出码一与指引在场，两形合规放行，测试全绿 |
| **F-2** 绕行留痕 | 工程治理 | bypass 登记后 reconcile 不告警，未登记无模板提交出 unbypassed 类 |
| **F-3** 安装态 | 工程治理 | 双仓 git config core.hooksPath 在位即钩子实跑生效，uninstall 可逆 |
| **F-4** 收口 | 链上治理 | lease 1.12.0 三源对齐、CONTRACT 修订、BATCH-FACE 两处、双仓 settle 归并、reconcile 双零、链 valid、全量入版控 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/lease/src/lease/commitcore.py 即 build_message 模板与 reconcile
- 必读 2：sih-engine/sih/event/plan/sealwin3-solo-results.md 即追认档病灶清单
- 必读 3：sih-tools/facet/facet_task_packages/adisp-guard-1/topic.md 即裁决命题全文

## 六、约束 {#constraints}

1. 守卫只拦提交信息形态不判内容即拦多不拦漏边界如实入 CONTRACT（红线）
2. 引擎仓 sih-engine 的钩子同样安装即双仓同治（红线）
3. 历史不可改即存量直提笔不回溯重裁（红线）
4. 正规路径红线与链尾对表与撞锁显式 --session 照旧

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-4 全过

## 八、风险点 {#risks}

钩子误伤合法非常规形提交（如 merge --squash）即拦多，防御即合规形覆盖 lease 全部 message 形态加豁免登记通道。hooksPath 属本地 config 即换机器须重装，防御即 install-hooks 入会话启动检与 BATCH-FACE 前置节。

## 九、队形声明 {#formation}

单线形即委外代理亲写零子代理。

## 十、关联文件 {#related}

- 任务包源：用户 2026-09-02 批准令
- 链件：随批意图入当日链
- 关联：adjudisp-solo 裁决 2ada7331、DEC-021、sealwin3 追认档、goldfix 六笔

## 十二、叩问处置 {#elicit-dispositions}

叩问处置[守卫]: 消解 即大白话直述即拦截机制，非登记术语
叩问处置[绕行留痕]: 消解 即大白话直述即显式绕过登记，非登记术语
叩问处置[狗粮首绕]: 消解 即大白话直述即本批自身首次绕行实测，非登记术语

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/cli.py
- sih-tools/lease/src/lease/commitcore.py
- sih-tools/lease/src/lease/guardcore.py
- sih-tools/lease/src/lease/__init__.py
- sih-tools/lease/pyproject.toml
- sih-tools/lease/CONTRACT.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/lease/hooks/pre-commit
- sih-tools/lease/tests/
- sih-tools/BATCH-FACE.md
- sih-engine/sih/state/plan/guardhook-solo.md
- sih-engine/sih/event/plan/guardhook-solo-results.md
- sih-engine/sih/event/plan/guardhook-solo-materials/
- sih-engine/sih/event/trail/2026-09-02.ndjson
- sih-engine/sih/event/inputlog/2026-09-02.ndjson
- sih-tools/scribe/reports/
- sih-tools/scribe/CALL-LOG.md
- sih-tools/formatter/CALL-LOG.md
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/nomenclator/CALL-LOG.md
- sih-tools/meter/counts/
