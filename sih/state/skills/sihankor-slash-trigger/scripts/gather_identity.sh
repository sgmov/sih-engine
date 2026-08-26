#!/bin/bash
# gather_identity.sh
# 采集 client 端可查 pid/hostname/whoami/sandbox 组合,加盐 hash 成 identity_string
# 用于通过 MCP 提交给 server,留痕不签用
#
# 用法:
#   bash gather_identity.sh
#   echo $IDENTITY_STRING
#
# 输出单行 string 格式:
#   v1|pid|ppid|hostname|whoami|sandbox_id|mcp_session|timestamp|hash|salt

set -e

PID=$$
PPID_VAL=$PPID
HOSTNAME_VAL=$(hostname)
WHOAMI_VAL=$(whoami)
SANDBOX_ID_VAL="${TRAE_SANDBOX_SBOX_ID:-none}"
MCP_SESSION_VAL="${MCP_SESSION_ID:-none}"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

SALT=$(openssl rand -hex 32)

PAYLOAD="${PID}|${PPID_VAL}|${HOSTNAME_VAL}|${WHOAMI_VAL}|${SANDBOX_ID_VAL}|${MCP_SESSION_VAL}|${TIMESTAMP}|${SALT}"
HASH=$(echo -n "$PAYLOAD" | shasum -a 256 | awk '{print $1}')

IDENTITY_STRING="v1|${PID}|${PPID_VAL}|${HOSTNAME_VAL}|${WHOAMI_VAL}|${SANDBOX_ID_VAL}|${MCP_SESSION_VAL}|${TIMESTAMP}|${HASH}|${SALT}"

echo "$IDENTITY_STRING"
