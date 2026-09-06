# facepatch-solo 结果档：BATCH-FACE 勘误四条累积批（只增不改）

> 承接：任务包 facepatch-solo.md 与 2026-09-06 至 09-07 四批实测沉淀。单件：BATCH-FACE.md 勧误区追加一节四条，追加制零改动既有条款，零代码零外溢。
> 队形单线形 solo（委外执行），日期 2026-09-07，会话 sess-zcode-260907-facepatch（session_id d20b841ac39f1fa8，双仓租约 sih-tools＋sih-engine）。

## 意图锚定

- 意图事件：intent_refined（event_hash `d9ed1c64...`）
- record：sih-tools/scribe/reports/2026-09-07-ask3-facepatch-solo-record.json（三锚引文程序切片：08-on-settle.md L110 应而不藏、07-on-assay.md L55 鉴只列事实、01-ontology-of-names.md L18 承载不撤回）
- validation：sih-tools/scribe/reports/2026-09-07-ask3-facepatch-solo-validation.json（status ok，anchor_count 3）

## 四条勘误入档（本批核心产出）

新增节「坑位勘误 2026-09-07（facepatch-solo 批增，四批实测累积）」四条，逐条携令源批名：

1. **facet 测量必须从工地 facet 跑**（令源 confmath-solo）：主树执行致飞轮 trail 误落主树，cwd 置工地 facet 绝对路径传参。
2. **checkcite --cited 单值参数**（令源 confconst-solo，confrulegate 复踩）：多传仅末位生效即假阳性空转，对表一律拼接扫描形，§10.5 增补注记。
3. **F-1 红可材料修复即修复重核，不可修复才停批**（令源 confrulegate-solo）：形缺陷程序切片重嵌推进判定文本零改，实质冲突不可调和即停批。
4. **open 前按请求写入节逐仓核对 --repo 覆盖**（令源 confrevise-solo）：漏传仓即无工地通道被迫 bypass 兜底；承既有「lease repo 路径」条延伸注记（彼管路径解析形，此管仓覆盖面），合并不重立。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 四条入档 | 数据治理 | 勘误节新增四条零遗漏，逐条携令源批名 | 通过（新增节四条即任务包四条一一对应，令源批名逐条在档） |
| F-2 零改写在役条款 | 治理 | 既有命令与坑位零触碰（diff 仅新增） | 通过（git diff 7 行纯新增，删除行计数 0） |
| F-3 管线绿 | 治理 | 化格核阅检词三步读数在档 | 通过（化格 exit 0、核阅 exit 2 属域外如实记档、检词 exit 0 零违例） |

## 前置读数

- 三问双门：核阅 ask3 包 exit 0；引擎 ask3repeater exit 0 status ok anchor_count 3。
- 叩问：elicit check 五词全出轻信号（追加制/单值参数/逐仓核对/令源批名/勘误区）；digest passed covered 5。
- 正身：identity verify attest 零异常。
- 不重复立法核对：入档前 grep 四主题——facet 工地纪律、checkcite 单值、F-1 修复、逐仓核对均无既有条目；第四条与「lease repo 路径」条语义分面（路径解析 vs 仓覆盖），以延伸注记合并不重立。

## 管线与对表读数

- 化格：BATCH-FACE.md packs/general-v1 exit 0。
- 核阅：引擎件 des-001 exit 2 如实记档（sih-tools 属 des-001 域外）。
- 检词：nomenclator check packs/core exit 0 零违例。
- 书单对表：拼接扫描形，扫描域＝本批追加节（diff 新增行）。首扫整档 fail 属扫描域误选非本批产出引用——BATCH-FACE 既有正文引引擎规格号位（SPEC-014/015/019）被 ID_RE 的 SPEC 前缀误命中（数学仓书单合法地不含引擎规格文档）；改追加节扫描即 pass（追加节零数学载体 ID 引用）。ID_RE 的 SPEC 前缀在引擎文档语境的范围限制申报为勘误候选，候后继批不代立。

## 认证清单

| 件 | 类型 | 状态 |
|---|---|---|
| ask3 记录 | 意图记录 | 书简认证 `98f58abf`（event_hash 前8） |
| ask3 验证件 | 双门读数 | 书简认证 `b8cfa6bc` |
| 正身件 | 身份报告 | 书简认证 `6e75a07a` |

## 越线与误差申报

- checkcite 首扫整档 fail（扫描域误选）：守卫语义是批引用产出对表，本批产出为追加节非整档既有正文；改追加节扫描 pass，SPEC 前缀范围限制申报勘误候选，如实记档。
- 其余零越线零申报。

## 结算读数

- 待补（收约后经补笔通道回填）。
