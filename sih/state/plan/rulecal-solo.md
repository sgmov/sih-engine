# rulecal-solo：des-001-mathe C006 公式引用校准批

> task-packages 治理任务
> 承接：sess-zcode-260830-mathprobe 三问意图（round 11）即 2026-08-30 链事件 b375ba25；前置即 m-fmtc2 机签 854df20a 裁决通过后用户条件推进
> 队形：单线形 solo——主线亲写零子代理
> 日期：2026-08-30

## 一、问题陈述 {#problem}

数学条目的公式引用括号（见公式 $\ref{...}$）共 72 处被 C006 判违规，而其 LaTeX 层经 latex-helper validate 全量验证零错误，章程全角括号条款本身的语义即引用论证，规则把引用类误伤为违规属校准缺口。

## 二、关键设计 {#design}

C006 模式新增两条前瞻豁免即见公式前缀与含 $\ref 的括号，message 同步注明允许形态，manifest 升 0.3.0 加注释行载机签凭证。验收即核阅全量复跑放宽计数恰 72 且其余类逐一不变，复算不符即回滚模式。des-001 基础包不动。

## 三、工作清单 {#work}

- [ ] C006 模式与 message 修订加注释行
- [ ] manifest 0.3.0
- [ ] 核阅全量复跑对表验证
- [ ] 认证上链双仓收约

## 四、可证伪条件 {#falsifiable}

| F | 类别 | 判据 |
|---|---|---|
| **F-1** 放宽计数恰 72 | 工程 | 复跑后 C006 总数恰降 72 且全为公式引用类 |
| **F-2** 无其他变化 | 工程治理 | S005 与 M008 与 C002 计数逐一不变，非公式类全角括号照旧违规 |
| **F-3** 版本管理齐 | 治理 | manifest 0.3.0 加注释行载机签哈希，des-001 基础包零改动 |

## 五、必读文件 {#read}

- 必读 1：sih-tools/scrutinator/packs/des-001-mathe/rules.toml 即 C006 现行定义
- 必读 2：sih-engine/sih/event/trail/2026-08-30.ndjson 即机签 854df20a 所在链

## 六、约束 {#constraints}

1. 只动 des-001-mathe 包
2. 复算不符即回滚模式
3. 上链遇锁即等待；链快照复制在最后一笔追加之后
4. 词债不过夜

## 七、请求写入 {#requested-writes}

- sih-tools/scrutinator/packs/des-001-mathe/rules.toml
- sih-tools/scrutinator/packs/des-001-mathe/manifest.toml
- sih-engine/sih/state/plan/rulecal-solo.md
- sih-engine/sih/state/plan/rulecal-solo-results.md
- sih-engine/sih/event/plan/rulecal-solo-results.md
- sih-engine/sih/event/plan/rulecal-solo-materials/
- sih-engine/sih/event/trail/2026-08-30.ndjson
- sih-tools/scrutinator/CALL-LOG.md
- sih-tools/scribe/CALL-LOG.md
- sih-tools/lease/CALL-LOG.md
- sih-tools/meter/counts/

## 八、验收标准 {#acceptance}

- [ ] F-1 至 F-3 全过
- [ ] 认证入链，双仓段结算收约
