# 沙盒清场注记(如实记)

- 沙盒仓(h3-repo 与 h4-a/b/c 及其 -wt 工地副本)的 .git 目录已剥除:防 gitlink 嵌入主仓,文件树留作惰性证据。
- h1-fakeroot 工地副本裁剪至 lease/src/lease/*.py(演示所需最小面),其余(含台账副本与 hooks)不入档。
- 各仓 sessions.ndjson.lock 空锁件与 __pycache__ 已清。
- 复现脚本(h4-repro.py 与 h4-b-child.py 与 h1-h3-repro.py)与日志(h4-repro.log 与 h1-h3-repro.log)与判定件(red-evidence.json)全数在档,可重放:python3 h4-repro.py 与 python3 h1-h3-repro.py(生产 lease 源只读导入)。
