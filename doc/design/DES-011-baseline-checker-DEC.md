# DES-011 桥接件立名 DEC + cross-link 协议

## 概览 {#overview}

- 文档类型即 DEC 决策，2026-08-17 立文，承接 DES-011-adjudication-baseline-check.md 桥接件定位节，关联 T6D-02 任务包 fix-failures-t6d 的 A5::[决策内容](#decision)
- 决策一即暂名基线核对器正式立名 baseline_checker，英文小写下划线制，不保留桥接件作独立名称::[立名承接](#naming)
- 决策二即 cross-link 协议，定义 facet 与 sih-engine 两侧事件互链的四步链与四条不变量，facet 侧 trail 增 sih_engine_event_id 与 cross_link_verified 两字段::[cross-link 协议](#cross-link)
- 生效分两档即立文即生效与配套实现后生效::[决策生效条件](#effective)
- 边界五条不越 DES-011 既有划定，可证伪条件三条::[边界](#boundary)

## 一、决策内容 {#decision}

DES-011 桥接件定位节所述「暂名基线核对器」正式立名为 baseline_checker。

英文小写下划线命名，与 maturation_gate、program_signoff、knife_edge_risk_metric 等 facet 组件命名风格一致。

承接路径：

- 路径一：DES-011 核对规则集，即 R1 至 R7 形式合规与来源合规
- 路径二：DES-011 闸三态映射，即 stable_clear、near_threshold、boundary 处置

落地点即 DES-011 桥接件定位节列举：

- 落点一：OQ-09 doclint 迁移获得组件协议承接方
- 落点二：OQ-10 旧仓 MCP 的 record_trail 职能获得新仓等价实现路径
- 落点三：OQ-11 OQ 机制的消化获得确定性程序承载形态

## 二、cross-link 协议 {#cross-link}

承接 T6D-02 A5 schema 改动，即 facet record_program_signoff 加两字段。

### 协议定义 {#protocol-def}

facet 侧即材料生成器 sih-tools/facet/probes/flywheel_trail.py，签名扩展：

    def record_program_signoff(
        ...,
        *,
        sih_engine_event_id: str | None = None,
        cross_link_verified: bool = False,
        trail_root=None,
    ):
        record = {
            ...
            "sih_engine_event_id": sih_engine_event_id,
            "cross_link_verified": cross_link_verified,
        }

sih-engine 侧即 baseline_checker，消费流程：

    facet_event = facet_trail.load_latest_program_signoff(guidance_id, facet_trail_root)
    形式核对 R1 至 R7，语义核对 v3 verdict 与 criteria_version 与 family_temperature_version
    通过即写 crosscheck_completed 事件并回写 facet 侧两字段
    不通过即写含偏差记录的 crosscheck_completed 事件

### 跨仓事件流 {#event-flow}

四步单向链：

1. facet 写 program_signoff 事件，sih_engine_event_id 为空，cross_link_verified 为假
2. baseline_checker 消费，执行规则 R1 至 R7 加语义核对
3. sih-engine event_stream 写 crosscheck_completed 事件即 evt_X
4. 回写 facet trail 事件，sih_engine_event_id 记 evt_X 即单向引用，cross_link_verified 置真即双向确认

### 协议不变量 {#invariants}

- 单向引用：facet 指向 sih-engine，facet 事件的 sih_engine_event_id 是唯一外键
- 双向确认：sih-engine 写完 crosscheck_completed 后必须回写 facet 事件的 cross_link_verified 为真
- 失败态：baseline_checker 拒绝材料时不回写，cross_link_verified 保持假即待核对标记
- 事件类型扩展承接 SPEC-004：crosscheck_completed 事件新增字段 facet_event_id，即反向引用，从 sih-engine 侧指回 facet 事件

## 三、立名承接 {#naming}

承接 DEC-004 书简组件命名约定的轻量化命名，即 PRO-001 立名本体论：

- 基线核对器即 DES-011 暂名，新名 baseline_checker，英文下划线制
- 桥接件即行为描述旧称，归并入 baseline_checker，单一组件名

不再保留桥接件作为独立名称：桥接是行为描述不是组件名，baseline_checker 既是组件名也是行为描述。

## 四、决策生效条件 {#effective}

### 立即生效 {#effective-now}

- DES-011 桥接件定位段的暂名基线核对器更新为 baseline_checker
- A5 schema 改动即 sih_engine_event_id 加 cross_link_verified，随本 DEC 立文获得含义锚定
- audit_pipeline A5 脚本的 cross-link 缺口判定预期从三个 gap 降至一个 gap，剩 DEC 文档本身的存在性判定

### 后续生效 {#effective-later}

- baseline_checker 实际代码实现，DES-011 边界注明实现归后续 SPEC
- SPEC-004 事件流规约扩展，即新增 facet_event_id 字段
- facet 端消费 cross_link_verified 为真信号后的行为，即 supersession 是否自动触发，待 SPEC 决定

## 五、边界 {#boundary}

- 不实现 baseline_checker 代码，DES-011 边界已划定
- 不改 DES-011 核对规则集，即 R1 至 R7 形式不变
- 不改 DES-011 闸三态映射，即 stable_clear、near_threshold、boundary 不变
- 不改 facet 仓判据，即 v3 交付定格不动摇
- 不动 SPEC-004 现有事件类型，只扩展 crosscheck_completed 的 facet_event_id 字段

## 六、可证伪条件 {#falsifiable}

- F-DEC-1：若 baseline_checker 立名与 program_signoff、maturation_gate 等组件命名风格不一致，则本 DEC 须重审命名
- F-DEC-2：若 cross-link 协议单向引用导致 facet 侧 trail 与 sih-engine event_stream 失同步，即一方写入另一方丢失，则协议须扩展为双向事务
- F-DEC-3：若 cross_link_verified 字段被滥用为已升格衍生原则标记，即实质裁决动作，则须修订该字段语义为仅核对不裁决

任一 F 触发即 DEC 须重审。

## 七、关联 {#relation}

- 元依据：DES-011-adjudication-baseline-check.md 桥接件定位
- 承接：PRO-001 立名本体论、DEC-004 书简组件命名、SPEC-004 事件流规约
- 平行：T6D-02 fix-failures-t6d 任务包 A5、flywheel_trail.py 的 record_program_signoff A5 schema
- 下游：baseline_checker 代码实现 SPEC、SPEC-004 扩展、facet supersession 路径

## 八、自检 {#self-check}

### 形式合规 {#formal}

- 一级标题无锚点，仅一个
- 二级及以上标题均带锚点
- 首个二级标题命名为概览
- 无破折号、无装饰符号、无 Unicode Emoji
- 全角括号仅用于引用论证

### 内容合规 {#content}

- 决策内容即命名加协议显式声明
- 边界明确划分，即不实现、不改判据、不动 SPEC-004 现有事件类型
- 可证伪条件三条指向命名一致性、协议同步、字段语义滥用三种失败模式
- 关联文档链回 DES-011、T6D-02 任务包、现有 schema
