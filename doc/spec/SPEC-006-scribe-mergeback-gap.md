# SPEC-006 书简融回落差规格

本规格承接 SPEC-004 书简操作组件规格与 DEC-013 融回门机制，钉死书简融回的引擎侧待建面。本规格是 SDD 产物即先于实现，实现批按 TDD 执行即逐判据先红后绿。工具侧现状权威即 sih-tools/scribe/CONTRACT.md。

## 概览 {#overview}

- 引擎现面即 append 四验与 hash 单源与 verify 与 query 二十七单测，落差为三入口加对拍壳加退出码面::[现面盘点](#baseline)
- 三入口即 intent 双件消费、park 双动作与配对不变量、报告消费八项负载::[入口落差](#entries)
- golden 对拍即向量集五类边界加生产 trail 四文件全量复验，不过即无写权限::[对拍与复验](#golden)
- 组件边界即库扩展加命令行二元，MCP 暴露位承引擎主线布线::[组件边界](#boundary)
- TDD 测试计划六组即 T1 至 T6 逐判据先红后绿::[测试计划](#tdd)

## 现面盘点 {#baseline}

引擎已有即 src/event_stream 四模块。append 承四验即事件 ID 重复拒、时间戳非单调拒、前事件哈希不匹配拒、操作者非法拒。hash 即 compute_event_hash 单源权威，SHA-256 加字段名字典序序列化加创世前哈希六十四零。verify 承全量与区间。query 承条件检索。合计二十七单测。

scribe 生产面已有即五子命令 append、verify、query、intent、park，golden 前置即出厂首写前对拍，退出码三值即零成功、一异常发现、二工具自身异常。

落差即引擎缺三入口、缺对拍壳、缺命令行面。

## 入口落差 {#entries}

### intent 入口 {#intent-entry}

输入为双件即意图记录 JSON 加核验报告 JSON。核验报告须为 ask3 类零发现，有发现即拒不入流。负载必载即记录路径与哈希、报告路径与哈希、规约包版本清单、会话标识、轮次、锚点数、调用计量、意图目标摘要、工具版本、golden 基线。doc_id 取会话标识即同会话跨事件串联。事件类型 intent_refined，event_class 沿 record_only。写入前校验承 SPEC-004 四验不松。

### park 入口 {#park-entry}

双动作即 enter 与 exit。enter 必载 entry_id、title、exit_condition、ttl_days 即正数，context 可选。exit 必载 entry_id、disposition 即 promoted 或 discarded、ruling。doc_id 取 entry_id。事件类型 parking_entered 与 parking_exited，event_class 沿 record_only。

配对不变量两道机械门。重入拒即同 entry_id 已在泊再 enter 拒。无主出拒即无在泊 enter 的 exit 拒。在泊判定按链序重放停泊事件。拒绝零留痕即不入流。

### 报告消费入口 {#append-entry}

输入为工具报告 JSON 加显式退出码参数。负载八项必载即报告路径、报告自身哈希、机械提取的规约包版本清单、机械提取的目标内容哈希清单、机械提取的发现或结论计数、显式退出码参数、工具版本、golden 基线版本。事件类型 certification_completed。重复消费同一报告即两次独立事件各载同一报告哈希，可审计重复，不是拒绝事由。写入前校验承 SPEC-004 四验不松。

## 对拍与复验 {#golden}

向量集五类边界。创世首事件、details 空值、可选字段缺省、非 ASCII 转义、时间戳编码边界。引擎侧生成向量集，实现复算逐字节一致。

生产复验。sih-tools/scribe/trail 即 2026-08-21 至 2026-08-27 四文件全量 verify，判据为四文件全 valid 且末哈希与工具侧 verify 输出一致。

对拍不过即组件不存在写权限，承 scribe 契约 golden 前置条款。

## 组件边界 {#boundary}

库面。三入口落 src/event_stream 新模块，复用既有 Event 与 append 四验与 compute_event_hash，不另建事件结构。

命令行面。二元承 ask3gate 先例即 src/bin 下新入口，子命令五件对齐工具例即 append、verify、query、intent、park，退出码三值对齐。

MCP 暴露位。引擎主线即 Rust MCP server 承 DES-020 布线，本组件只保证库面可被 MCP 层直接调用，不在本落差范围建 server。

## 测试计划 {#tdd}

逐判据先红后绿，红态即测试先行而入口未建，绿态即实现批完成。六组如下。

T1 golden 向量集
: 引擎生成五类边界向量，实现复算全量一致。红即向量入口不存在。

T2 生产复验
: 四文件全量 verify 全 valid，末哈希与工具侧一致。

T3 intent 双件消费
: 零发现放行即事件入流且负载十项齐。有发现拒即不入流且零留痕。

T4 park 配对不变量
: 重入拒与无主出拒两道门，在泊判定链序重放出泊后可再入。

T5 报告消费
: 八项负载机械提取，重复消费两事件同报告哈希可审计。

T6 退出码
: 五子命令退出码三值即零成功、一异常发现、二工具异常。

红转绿记录入实现批结果档，全绿为切换批入口条件。

## 验收判据 {#acceptance}

- A1 三入口签名与负载与 scribe 生产面对表无漏项
- A2 配对不变量两道机械门可判定
- A3 向量集五类边界齐备且复算一致
- A4 生产四文件复验全 valid
- A5 退出码三值与工具例对齐
- A6 库面可被 MCP 层直接调用即无 CLI 耦合入库层
