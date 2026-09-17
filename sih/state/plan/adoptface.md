# adoptface：采纳面四件——README 重写与接入指南与 sih init 薄壳与 DEC-026 缺省 stdio

> 令源：用户 2026-09-17 开批裁定（采纳面批，动态工作流结算前置链立约开工）
> 范式：T6 单线 solo，主代理亲写零子代理

## 一、问题陈述 {#problem}

- 病灶一：引擎仓 README（sih-engine/README.md，49 行）未覆盖现役动线——MCP 双面由 Rust sihmcp 现役承载（DEC-023）、生产调用面切引擎 bin 位（2026-09-14 切换批），README 停于旧形态，新用户首跑动线断。
- 病灶二：接入零成文指南——域自举（sih 树落位）、客户端配置、会话启动五件、租约治理动线散落正典档，无一份面向采纳者的单页动线；AGENTS.md 配置无模版可裁。
- 病灶三：sih init 缺命令行薄壳——域自举机械内核在 src/mcpserver/bootstrap.rs（MCP bootstrap 工具位在役），但无 CLI 入口，非 MCP 客户端采纳者无法一行落 sih 树。
- 病灶四：MCP 载体缺省传输无决策档——stdio 面与 8765 HTTP 面双通道在役（DEC-023 与 DES-015），缺省面未裁，客户端配置无所依。

## 二、关键设计 {#design}

- 2.1 README 重写：以现役调用形（引擎 bin 位与 sihmcp stdio/HTTP 双面）为唯一动线源，围堰旧调用形一律不入；五分钟跑起来链路与 sih init 引导衔接。
- 2.2 接入指南单页（doc/guide/）：域自举（sih init 落位清单对表 bootstrap 常量面）→ 客户端配置（缺省 stdio 形）→ 会话启动五件（自检、回锚、判据扫、读数、心跳）→ 租约治理首件；AGENTS 模版给最小可配形，采纳者照模版裁剪。
- 2.3 sih init 薄壳：src/bin/sih.rs 承 init 子命令，直调 src/mcpserver/bootstrap.rs 域自举内核；src/mcpserver/ 只做最小 pub 可见性放宽，零行为变更，逐条申报；新增 .rs 文件头注前二十行必须含正典指针词形（SPEC 或 DES 或 DEC 至少一处，SDDG-2 机械门要求）。
- 2.4 DEC-026：缺省 stdio 决策成文（doc/decision/026-mcp-default-stdio-v1.md，000 文档格式），HTTP 面转显式选用位；决策只裁缺省面，不裁通道存废。

## 三、工作清单 {#work}

- [ ] af-01：README 重写（现役动线全量对表，五分钟跑起来段逐条活体可跑）
- [ ] af-02：接入指南 + AGENTS 模版（doc/guide/adopt-v1.md 与 doc/guide/AGENTS-template.md）
- [ ] af-03：sih init 薄壳（src/bin/sih.rs）+ mcpserver 最小 pub 放宽（零行为变更，逐条申报）
- [ ] af-04：DEC-026 缺省 stdio 决策成文
- [ ] af-05：验收——cargo test 全绿、Cargo.toml 0.9.0 与依赖清单零变动对表、sih init 活体落位对表 bootstrap 常量、T6 三步审计（化格、核阅、检词）过

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | README | README.md 重写后链路命令逐条活体可跑（五分钟跑起来段），围堰旧调用形零残留 |
| **F-2** | 指南与模版 | 接入指南步骤与 sih init 实际落位文件一一对应（对表 bootstrap.rs 常量面），AGENTS 模版照本仓 AGENTS.md 最小可配形可裁 |
| **F-3** | sih init | 目标根活体跑 sih init 后 sih 树落位与 MCP bootstrap 位产物一致；src/mcpserver/ diff 仅 pub 可见性放宽，cargo test 全绿证零行为变更 |
| **F-4** | DEC-026 | 决策档成文过 000 文档格式与 T6 三步审计，缺省 stdio、HTTP 显式选用语义无歧义 |
| **F-5** | 硬纪律 | Cargo.toml 0.9.0 钉死与依赖清单零增删、他批在途件零触碰、新增 .rs 头注前二十行含正典指针词形（SDDG-2 过） |

## 五、必读文件 {#read}

- sih-engine/src/mcpserver/bootstrap.rs（域自举内核与常量面）
- sih-engine/src/mcpserver/mod.rs
- sih-engine/README.md（现状）
- sih-engine/doc/decision/023-mcp-rust-carrier.md 与 025-project-memory-snapshot-initiation-v1.md（DEC-026 前位）
- sih-engine/doc/decision/000-*.md（000 文档格式正典）

## 六、约束 {#constraints}

1. Cargo.toml 版本 0.9.0 钉死，依赖清单零增删
2. src/mcpserver/ 仅允许为 sih init 薄壳做最小 pub 可见性放宽，零行为变更，逐条申报
3. 不碰他批在途件：主树 src/bin/{formatter,locator,nomenclator,parser,selector,tally}.rs 的 M 态与 sih/event 与 sih/state 旧档零触碰
4. 围堰只读：sih-tools/ 零写入（identity verify 只读运行唯一例外，PYTHONDONTWRITEBYTECODE=1）
5. 全部施工写入工地副本路径，主树零直写（任务包与批材料按惯例落主树 sih/sih/{state,event}/plan/ 未跟踪位）
6. 新增 .rs 文件头注前二十行必须含正典指针词形（SPEC 或 DES 或 DEC 至少一处，SDDG-2 机械门）
7. 禁 git commit/push 直呼，commit 由 lease commit 机械落笔
8. T6 管线序固定：化格、核阅、检词、认证

## 七、范式偏离声明 {#deviation}

- 甲表派生对表修正：任务包名 adoptface 无分隔符，stem 闸机械分段仅按 `-`/`_`/`.` 切分（lease.rs:258 对表 attachments.rs:258-263），成单段 adoptface；立约甲表 --claim-derivation 由 adopt:new,face:new 修正为 adoptface:new（对表闸指引：派生对表须与机械分段圆合，attachments.rs verify_new_stem_claim 缺段表外段即拒）；概念锚 zh=采纳面 与 无承 申报不变，整词 adoptface 词典实态 unknown 已查册在案（2026-09-17）。

## 八、关联文件 {#related}

- sih-engine/sih/event/plan/adoptface-results.md（随批产出）
- sih-engine/sih/event/plan/adoptface-materials/（意图件与正身件）

## 九、请求写入 {#requested-writes}

- sih-engine/README.md
- sih-engine/doc/guide/adopt-v1.md
- sih-engine/doc/guide/AGENTS-template.md
- sih-engine/src/bin/sih.rs
- sih-engine/src/mcpserver/
- sih-engine/doc/decision/026-mcp-default-stdio-v1.md
- sih-engine/sih/event/plan/adoptface-results.md
- sih-engine/sih/event/plan/adoptface-materials/
