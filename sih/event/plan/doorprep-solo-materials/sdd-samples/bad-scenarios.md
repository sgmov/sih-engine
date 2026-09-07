# 场景清单：两道门备料批 doorprep-solo（坏样例——重号加缺判据行，红证专用隔离件）

### Requirement: R-001 流程包数据形
流程包 flow-v1 SHALL 以纯数据形声明任务类门规映射。

#### Scenario: S-001 流程包数据形落位读数
- **WHEN** 读 flow-v1.json
- **THEN** 包名与版本与治理批类条目在位
- 判据：json 解析 期望退出码 0

#### Scenario: S-001 流程包重号场景（故意重号触发 SL-003）
- **WHEN** 读 flow-v1.json
- **THEN** 此场景与上一场景编号重号，检查器应报重号违规

### Requirement: R-002 缺判据行场景
缺判据行要求 SHALL 触发 SL 缺判据行三态定位。

#### Scenario: S-002 缺判据行场景（故意无判据行）
- **WHEN** 读本场景
- **THEN** 本场景 THEN 行下无判据行，检查器应报缺判据行
