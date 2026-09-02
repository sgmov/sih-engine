# 路由金向量冻结说明（autoflow2-solo）

本目录是 SPEC-015 A1 金向量：期望输出全部由围堰 Python 原件
（sih-tools/selector）跑出冻结，围堰输出是唯一基准，禁自造期望。
冻结于 2026-09-02，冻结后零漂移由 T2 哈希对表测试钉死。

## 目录构成

| 目录 | 类别 | 内容 |
|---|---|---|
| net-core/ | 净目标一 | 六件真实任务包评价材料经 core 包路由（sih-tools/selector/baseline/process-files 真实件） |
| net-parking/ | 净目标二 | sih-tools/parking/materials 真实在泊材料十六件经 parking 包路由，参照时间 2026-09-02 |
| dirty-schema/ | 脏形一 | 真实件缺 state 判 R001 败走 scrap_track，饥饿告警退出码一 |
| dirty-anchor-domain/ | 脏形二 | anchors 出白名单判 R003 败走 siding |
| dirty-expired/ | 脏形三 | 停泊材料参照时间越 ttl 判 time_deadline 败走 siding，超期告警 |
| badpack-kind/ | 包非法形 | 未知谓词 kind 拒包错误信封，@PACKDIR@ token 归一 |
| attribution-core/ | 归因基线 | 同混合材料集（七件真实任务包评价件加三件真实泊件）经 core 包路由 |
| attribution-parking/ | 归因基线 | 同材料集经 parking 包路由，与 core 互为镜像归因 |

## 归一形条款

badpack-kind/expected.json 含 `@PACKDIR@` token（包目录绝对路径），
测试运行时以自身运行时路径反填后逐字节 cmp。归一仅涉路径位，不涉任何
语义字段。报告类期望件不含路径位即原样逐字节冻结。

## 同参形条款

包数据一致（src/attractor/packs/ 随迁件与围堰原包逐字节一致）、材料
一致（输入 json 同源）、参照时间一致（同值同形）、绝对路径形态运行。
活体双跑对表证据：sih/event/plan/autoflow2-solo-materials/route-double-run-cmp.log
（三场景 cmp 全 IDENTICAL）。
