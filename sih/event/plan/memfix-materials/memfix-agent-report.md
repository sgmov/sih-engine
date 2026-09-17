# memfix 批子代理施工归报

- 工地：w/memfix（手工形），移植至租约形 worktrees/sih-engine/memfix（分支 msh/memfix），移植后字节等同
- 施工形：委外子代理亲写零再委外，零 git 写动作零治理命令

## 改动点清单

1. `index_path()`（修后 :86-92，原 :86-88）：`retriever-index-<pid>.ndjson` → `retriever-index-<pid>-<ThreadId(N):?>.ndjson`
2. `build_index()` 内 canonical pack 临时件（修后 :97-102，原 :93-97）：`retriever-canonical-pack-<pid>.json` → `retriever-canonical-pack-<pid>-<ThreadId(N):?>.json`

两处均保留 pid 追加 `{:?}` 形线程 id（`ThreadId(3)`，合法文件名成分），跨进程隔离不回退，函数签名与语义零变。

## 新测试

`retriever::locator_bridge::tests::index_path_distinct_across_threads`（:220）：起 8 线程各调 `index_path()` 收集断言 HashSet 去重后 len==8。可见性零改动，测试子模块天然可见父模块私有 fn。

## 红转绿证据

- 修复前红证：`assertion failed: unique.len() == 8`，left: 1 right: 8，8 路径全同 `retriever-index-85276.ndjson`，0 passed 1 failed
- 修复后：cargo build 退出码 0；cargo test --lib retriever 14 passed 0 failed 6 ignored（ignored 系既有 wenguobs 用例）；mem_recall_f_suite 9 passed 0 failed

## 偏差申报

1. TDD 双向验证：先落测试跑出修复前必红红证再修，红证归档本材料。
2. 掺名方式保留 pid 追加 tid 而非替换，改动最小。
3. 范围外发现未修：src/retriever/mod.rs:371 `retr-env-<pid>` 同类掺名位，因域界禁改，由 retrline 批域内顺带处置，主会知悉。
4. 子代理首轮验证曾误取管道尾退出码，已改 pipestatus 真值捕获，后续退出码俱真。
