# DES-011 桥接件立名 DEC + cross-link 协议

> 文档类型：DEC（决策）
> 承接：DES-011-adjudication-baseline-check.md §桥接件定位
> 日期：2026-08-17
> 关联：T6D-02 任务包 fix-failures-t6d §A5

## 一、决策内容 {#decision}

DES-011 §桥接件定位所述"暂名基线核对器"正式立名为：

**`baseline_checker`**

英文小写下划线命名（与 `maturation_gate` / `program_signoff` / `knife_edge_risk_metric` 等 facet 组件命名风格一致）。

承接路径：
- 路径 1：DES-011 §核对规则集（R1-R7 形式合规与来源合规）
- 路径 2：DES-011 §闸三态映射（stable_clear / near_threshold / boundary 处置）

落地点（DES-011 §桥接件定位 §62 列举）：
- 落点 1：OQ-09 doclint 迁移获得组件协议承接方
- 落点 2：OQ-10 旧仓 MCP 的 `record_trail` 职能获得新仓等价实现路径
- 落点 3：OQ-11 OQ 机制的消化获得确定性程序承载形态

## 二、cross-link 协议 {#cross-link}

承接 T6D-02 A5 schema 改动（facet `record_program_signoff` 加 2 字段）：

### 协议定义

**facet 侧**（材料生成器，sih-tools/facet/probes/flywheel_trail.py）：

```python
def record_program_signoff(
    ...,
    *,
    sih_engine_event_id: str | None = None,  # 新参数
    cross_link_verified: bool = False,       # 新参数
    trail_root=None,
):
    # 写 trail 事件，2 字段附在事件 dict
    record = {
        ...
        "sih_engine_event_id": sih_engine_event_id,  # cross-link 到 sih-engine event_stream
        "cross_link_verified": cross_link_verified,  # baseline_checker 核对后置 True
    }
```

**sih-engine 侧**（基线核对器，DES-011 落点 1）：

```python
# baseline_checker 消费 facet program_signoff 事件
facet_event = facet_trail.load_latest_program_signoff(guidance_id, facet_trail_root)
# 形式核对：R1-R7（DES-011 §核对规则集）
# 语义核对：v3 verdict / criteria_version / family_temperature_version
# 通过 → 写 sih-engine event_stream 事件 crosscheck_completed
#   + 回写 facet_trail 事件的 sih_engine_event_id + cross_link_verified=True
# 不通过 → 写 crosscheck_completed 事件含偏差记录（DES-011 §输出契约）
```

### 跨仓事件流

```
facet program_signoff 事件 (sih_engine_event_id=None, cross_link_verified=False)
  ↓
baseline_checker 消费（规则 R1-R7 + 语义核对）
  ↓
sih-engine event_stream 写 crosscheck_completed 事件（event_id=evt_X）
  ↓
回写 facet_trail 事件:
  sih_engine_event_id="evt_X"  (单向引用)
  cross_link_verified=True      (双向确认)
```

### 协议不变量

- **单向引用**：facet → sih-engine（facet_event.sih_engine_event_id 是唯一外键）
- **双向确认**：sih-engine 写完 crosscheck_completed 后，**必须**回写 facet_event 的 cross_link_verified=True
- **失败态**：baseline_checker 拒绝材料时**不**回写（保持 cross_link_verified=False 作为"待核对"标记）
- **事件类型扩展**（承接 SPEC-004）：crosscheck_completed 事件新增字段 `facet_event_id`（反向引用，从 sih-engine 侧指回 facet_event）

## 三、立名承接 {#naming}

承接 `DEC-004` 书简组件命名约定的轻量化命名（PRO-001 立名本体论）：

| 组件 | 旧名（DES-011 暂名） | 新名（DEC 决定） | 命名风格 |
|---|---|---|---|
| 基线核对器 | 基线核对器 | `baseline_checker` | 英文下划线 |
| 桥接件 | 桥接件 | `baseline_checker` | 单一组件名 |

**不再保留"桥接件"作为独立名称**——桥接是行为描述，不是组件名。`baseline_checker` 既是组件名也是行为描述。

## 四、决策生效条件 {#effective}

### 4.1 立即生效（本 DEC 立文后）

- DES-011 §桥接件定位段中"暂名基线核对器"更新为"`baseline_checker`"
- A5 schema 改动（`sih_engine_event_id` + `cross_link_verified`）随本 DEC 立文获得含义锚定
- audit_pipeline A5 脚本的 cross-link 缺口判定**预期**从 3 个 gap 降至 1 个 gap（剩 DEC 文档本身的存在性判定）

### 4.2 后续生效（需配套实现）

- `baseline_checker` 实际代码实现（DES-011 §边界 §66 注明"实现归后续 SPEC"）
- SPEC-004 事件流规约扩展（新增 `facet_event_id` 字段）
- facet 端消费 `cross_link_verified=True` 信号后的行为（`supersession` 是否自动触发？—— 待 SPEC 决定）

## 五、边界 {#boundary}

本 DEC 不：
- 不实现 `baseline_checker` 代码（DES-011 §边界 §66 已划定）
- 不改 DES-011 §核对规则集（R1-R7 形式不变）
- 不改 DES-011 §闸三态映射（stable_clear / near_threshold / boundary 不变）
- 不改 facet 仓判据（v3 交付定格不动摇，DES-011 §边界 §66）
- 不动 SPEC-004 现有事件类型（只扩展 crosscheck_completed 的 `facet_event_id` 字段）

## 六、可证伪条件 {#falsifiable}

- **F-DEC-1**：若 `baseline_checker` 立名导致与现有 `program_signoff` / `maturation_gate` 等组件命名风格不一致，则本 DEC 须重审命名
- **F-DEC-2**：若 cross-link 协议单向引用导致 facet 侧 trail 与 sih-engine event_stream 失同步（一方写入另一方丢失），则协议须扩展为双向事务
- **F-DEC-3**：若 `cross_link_verified` 字段被滥用为"已升格衍生原则"标记（实质裁决动作），则须修订该字段语义为"仅核对，不裁决"

任一 F 触发 = DEC 须重审。

## 七、关联 {#relation}

- 元依据：DES-011-adjudication-baseline-check.md §桥接件定位
- 承接：PRO-001 立名本体论、DEC-004 书简组件命名、SPEC-004 事件流规约
- 平行：T6D-02 fix-failures-t6d 任务包 §A5、flywheel_trail.py:record_program_signoff A5 schema
- 下游：`baseline_checker` 代码实现 SPEC、SPEC-004 扩展、facet supersession 路径

## 八、自检 {#self-check}

### 形式合规

- 一级标题无锚点，仅一个
- 二级标题均带锚点
- 首个二级标题命名为"决策内容"
- 无破折号、无装饰符号、无 Unicode Emoji
- 引用使用半角双冒号加锚点链接格式

### 内容合规

- 决策内容（命名 + 协议）显式声明
- 边界明确划分（不实现 / 不改判据 / 不动 SPEC-004 现有事件类型）
- 可证伪条件三条指向命名一致性 / 协议同步 / 字段语义滥用三种失败模式
- 关联文档链回 DES-011 + T6D-02 任务包 + 现有 schema
