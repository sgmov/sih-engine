# callloghyg-solo 归属披露：14 件 CALL-LOG 增量行逐行归属对表

> 批：callloghyg-solo（会话 84ac7d6e813ca51f）
> 日期：2026-09-07
> 核验判据：归属零错配——每行批名与权威腿 calls.ndjson（session＋occasion）对得上才入账

## 一、总量

14 件 CALL-LOG 投影腿共增量 **29 行**，归属三个源批：

| 源批 | 会话号 | 行数 | 说明 |
|---|---|---|---|
| chaingreen-solo | f3770e02d8bcafe0 | 15 | 12 件修改册中 13 行（lease 双行）＋acceptor/checker 各 1 行 |
| doorprep-solo | b5e510eb18a64e57 | 12 | 11 件修改册各 1 行＋checker 1 行 |
| calllog-solo T-8 | s1（dogfooding） | 2 | lease 册 dogfooding 自动留痕两行（clog-b834533f、clog-b9488131） |

12 件修改册增量 26 行（elicit/facet/formatter/gauge/identity/meter/nomenclator/scrutinator/selector/tally/watchcheck 各 2 行，lease 5 行含 2 dogfooding）；2 件未跟踪册（acceptor 1 行、checker 2 行）共 3 行。

## 二、逐行归属对表（权威腿 event_id 为核验证据）

### elicit/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 叩问四词四信号 digest covered 4 | chaingreen | f3770e02d8bcafe0 | clog-b2cfe30c |
| 叩问四词 digest covered 4 | doorprep | b5e510eb18a64e57 | clog-1d029f81 |

### facet/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| m-chaingreen-1 出题 9 发回填计分 9/9 | chaingreen | f3770e02d8bcafe0 | clog-3ea8a24b |
| m-doorprep-1 出题 9 发回填计分 9/9 | doorprep | b5e510eb18a64e57 | clog-5144dc8e |

### formatter/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| terms.json json-canonical；三 MD general-v1 | chaingreen | f3770e02d8bcafe0 | clog-72bb4b09 |
| flow-v1 json-canonical 归一退 1；六件 MD | doorprep | b5e510eb18a64e57 | clog-fa96dfa1 |

### gauge/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 例行读数三维落链 | chaingreen | f3770e02d8bcafe0 | clog-6a23a822 |
| 例行读数三维落链 | doorprep | b5e510eb18a64e57 | clog-1dbd5aec |

### identity/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 正身核验 anomalies 空 | chaingreen | f3770e02d8bcafe0 | clog-d81baf6b |
| 正身核验 anomalies 空 | doorprep | b5e510eb18a64e57 | clog-368a7cdc |

### lease/CALL-LOG.md（5 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| `{"at"...clog-b834533f..."commands":"status"...}` | calllog T-8 | s1 | clog-b834533f |
| `{"ok": true, "session_id": "s1"}...dogfooding` | calllog T-8 | s1 | clog-b9488131 |
| 租约开工双仓 13 锁 CALL-LOG 三腿 14 册 | chaingreen | f3770e02d8bcafe0 | clog-de40465a |
| DES 命名空间拆会话重开 --namespace | chaingreen | 27492fc552700ace | clog-66883c10 |
| 租约开工 11 锁、收约无主闸 16 件 bypass-orphan | doorprep | b5e510eb18a64e57 | clog-70bdddd9 |

### meter/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 意图 1 笔加认证 12 笔包裹 | chaingreen | f3770e02d8bcafe0 | clog-a6d701dd |
| 意图 1 笔加认证 23 笔；meter 不透传缺陷申报 | doorprep | b5e510eb18a64e57 | clog-751825f0 |

### nomenclator/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 批产三件检词 core 零违例；terms canonical | chaingreen | f3770e02d8bcafe0 | clog-45d40ce6 |
| 七件检词 core 零违例 | doorprep | b5e510eb18a64e57 | clog-fa5c5a12 |

### scrutinator/CALL-LOG.md（1 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 批产三件 des-001 域外 exit-2 如实记 | chaingreen | f3770e02d8bcafe0 | clog-afb20125 |

### selector/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 泊界心跳两线（引擎线 siding_surplus 既知告警 1） | chaingreen | f3770e02d8bcafe0 | clog-d310facb |
| 收约后心跳两线（引擎线既知告警 1 同态） | doorprep | b5e510eb18a64e57 | clog-c8e593a3 |

### tally/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| assemble --baseline stable_clear | chaingreen | f3770e02d8bcafe0 | clog-eec1c203 |
| assemble 基线换 doorprep 正身配对 | doorprep | b5e510eb18a64e57 | clog-eb778eb1 |

### watchcheck/CALL-LOG.md（2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 对表呈 30 件无主（8 件本批面＋22 在盘遗留）零代行 | chaingreen | f3770e02d8bcafe0 | clog-28cbcd51 |
| 开工对表 16 件无主、收约无主闸同 16 件 bypass-orphan | doorprep | b5e510eb18a64e57 | clog-7cd73626 |

### acceptor/CALL-LOG.md（未跟踪，1 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| 判定包端到端主树红证退出码 1（TC-003） | chaingreen | f3770e02d8bcafe0 | clog-722fa891 |

### checker/CALL-LOG.md（未跟踪，2 行）
| 行 | 源批 | 会话 | 权威腿证据 |
|---|---|---|---|
| test_golden_replay 改读基线向量新家 7 测全绿 | chaingreen | f3770e02d8bcafe0 | clog-d39e1478 |
| 样例五件五跑全绿、坏样例退 1 红证红绿双证 | doorprep | b5e510eb18a64e57 | clog-1007756d |

## 三、对表结论

- 29 行逐一与权威腿（calls.ndjson）session＋occasion 对表：**29/29 全过**，零悬置零硬归（F-2 归属零错配通过）。
- 三个疑误项（formatter/nomenclator/scrutinator 的 chaingreen 行）系投影行 command 格内嵌 `<topic|results|prompt>` 竖线致文本切分假阴性，真实会话 f3770e02d8bcafe0 在权威腿有对应记录（clog-72bb4b09/45d40ce6/afb20125），实为全过。

## 四、归账记录

- 14 件 CALL-LOG 投影腿主树 `--no-verify` 提交 f6b2def8（14 files changed, 29 insertions），经 `lease bypass` 登记通道入版控（bypass 台账 2026-09-07T06:01:58+00:00，会话 84ac7d6e813ca51f）。
- 留存量：`attnanchor/anchor.py`（attnanchor 批在飞）、`calllog/calls.ndjson`（calllog 批在飞，authority 腿）两件他批生产物零触碰，归属方自收（任务包约束 1）。
- 19 件 watchcheck 遗留（session_orphan）维持候裁零代清（F-4）。
