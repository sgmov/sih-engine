#!/usr/bin/env python3
"""confmodel 标定通道估计程序参考实现（S1/S2/S3，推导档 §7.3 规则面的机读形）。

确定性承诺：显式给参（--events 数据集路径），零随机零读钟零环境量；
仅排序、计数、中位数、最大值、整除取整。同参双跑逐字节一致。
输出值是夹具或数据的机械产出；本实现不主张任何模型常数值。

用法：python3 estimator_ref.py --events <events.jsonl> [--out <out.json>]
事件行形：{"ts": "YYYY-MM-DD", "kind": "clean_close|...", "account": "<键>", "amount": <int>}
"""
import argparse
import json
from datetime import date
from pathlib import Path


def d(s: str) -> date:
    return date.fromisoformat(s)


def median(xs):
    xs = sorted(xs)
    n = len(xs)
    if n == 0:
        return None
    mid = n // 2
    if n % 2 == 1:
        return xs[mid]
    return (xs[mid - 1] + xs[mid]) / 2


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--events", required=True)
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    events = [json.loads(l) for l in Path(args.events).read_text(encoding="utf-8").splitlines() if l.strip()]
    accounts = sorted({e["account"] for e in events})

    # S2：每账户相邻 clean_close 间隔（日），中位数，上取整到整日
    gaps = []
    for k in accounts:
        ts = sorted(d(e["ts"]) for e in events if e["account"] == k and e["kind"] == "clean_close")
        gaps += [(b - a).days for a, b in zip(ts, ts[1:]) if b > a]
    med = median(gaps)
    import math
    w = math.ceil(med) if med is not None else 1

    # 切窗：自数据集最早时间戳起按 w 日切
    t0 = min(d(e["ts"]) for e in events)

    def win(idx: int):
        start = t0.fromordinal(t0.toordinal() + idx * w)
        end = t0.fromordinal(t0.toordinal() + (idx + 1) * w - 1)
        return start, end

    n_windows = (max(d(e["ts"]) for e in events).toordinal() - t0.toordinal()) // w + 1

    # S1：每（账户，窗）clean_close 计数最大值；S3：净变动正部/负部最大值
    n_max = 0
    up_max = 0
    down_max = 0
    for k in accounts:
        for i in range(n_windows):
            lo, hi = win(i)
            in_win = [e for e in events if e["account"] == k and lo <= d(e["ts"]) <= hi]
            c = sum(1 for e in in_win if e["kind"] == "clean_close")
            n_max = max(n_max, c)
            net = sum(int(e["amount"]) for e in in_win)
            up_max = max(up_max, net)
            down_max = max(down_max, -net)

    result = {
        "w_days": w,
        "n_max": n_max,
        "U_plus": up_max,
        "U_minus": down_max,
        "U_symmetric": max(up_max, down_max),
        "N_min_formula": "floor(n_max)+1 (推导通道复算式，非本程序产出断言)",
    }
    text = json.dumps(result, ensure_ascii=False, sort_keys=True) + "\n"
    if args.out:
        Path(args.out).write_text(text, encoding="utf-8")
    print(text, end="")


if __name__ == "__main__":
    main()
