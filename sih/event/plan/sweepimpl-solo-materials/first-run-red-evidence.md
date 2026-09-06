# sweepimpl-solo 首跑红证归档（先红留痕纪律，2026-09-06 条款承载）

test_sweep.py 前三跑红证（夹具构造缺陷，非判定语义红），逐跑 tail 逐字转录自批会话输出：

## 第一跑（sweepcore+tests 同批首建后）

```
E       AttributeError: 'PosixPath' object has no attribute 'append_text'
tests/test_sweep.py:256: AttributeError
=========================== short test summary info ============================
FAILED tests/test_sweep.py::test_sweep_exit_code_semantics_unchanged - Attrib...
ERROR tests/test_sweep.py::test_sweep_scan_classifies_fixtures - FileNotFound...
ERROR tests/test_sweep.py::test_sweep_fix_clears_auto_and_zero_touches_live
ERROR tests/test_sweep.py::test_sweep_scan_readonly_status_unchanged - FileNo...
1 failed, 1 passed, 3 errors in 0.58s
```

病灶：git checkout 换支清空未跟踪目录致 event_plan 目录消失；`Path.append_text` API 不存在。

## 第二跑

```
FAILED tests/test_sweep.py::test_sweep_scan_classifies_fixtures - assert 2 == 3
FAILED tests/test_sweep.py::test_sweep_exit_code_semantics_unchanged - FileNo...
2 failed, 3 passed in 1.53s
```

病灶：幻影夹具缺 sweepfixt-results.md 致证据计数 2 非 3（夹具意图错位，判定行为本身正确——四证取二按实算）。

## 第三跑（工地自举形 F-3 sweep 首跑）

```
{
  "action": "传三参即可放行",
  "error": "self_boot_rejected",
```

病灶：工地 cwd 跑 sweep 缺 --bills 三参被 self_boot_check 拒 exit 2 零读写（rootanchor T-2 正确执法，调用方缺参）；补参后 F-3 实跑成立。全件见 sweep-scan-selfboot-rejected-red.txt。

## 第四跑后

5 passed（f1-f2-test-sweep-readings.txt 在档）。
