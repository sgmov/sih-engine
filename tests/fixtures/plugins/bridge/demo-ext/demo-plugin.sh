#!/bin/sh
# fixture 假插件：行分隔 JSON-RPC 2.0——每读一帧回一帧 result。
# 用外部 printf（非内建）：内建输出到管道全缓冲不冲刷，桥 read_line 永等。
while IFS= read -r line; do
  /usr/bin/printf '{"jsonrpc":"2.0","id":1,"result":{"echo":true,"got":%s}}\n' "$line"
done
