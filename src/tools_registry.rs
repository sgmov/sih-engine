//! tools_registry：腿五插件槽位架构骨架（SPEC-025 § 插件槽位协议 · 批
//! lease-mergeall-parallel · 簇C）。
//!
//! 三件套：ToolProvider trait（name 加 description 加 input_schema 加 call
//! 四面，async）加 ToolRegistry（register/list/call 动态注册）加
//! PluginManifest 解析（sih/plugins/<name>/plugin.json 五键）。外部插件桥
//! 接口签名以 ExternalPluginBridge 申报。
//!
//! 选形申报（模块落位形）：工地 src/lib.rs 已在且为唯一 lib 入口（mcpserver
//! 等 lib 模块同此），registry 以顶层 lib 模块落位加 lib.rs 一行导出为最小
//! 侵入——sihmcp.rs 是薄 bin（经 sih_engine::mcpserver 消费），在役面接线
//! 批同径消费 sih_engine::tools_registry，bin 侧零路径改造。src/bin/lease/
//! 子模块形会把 registry 锁进单一 bin 私域，tests 与其他 bin 均不可达，
//! 不满足 A3 统一注册形取向，故不取。
//!
//! 在役面改造让位后批申报：sihmcp 分派面（src/mcpserver/server.rs）本批零
//! 触碰；演示迁移取 HeartbeatProvider（包装在役只读工具
//! mcpserver::alpha::heartbeat，元数据与 MCP 面 defs 冻结同形，call 零变换
//! 直委托，同参同出参），运行演示 bin 即 src/bin/registrydemo.rs；MCP 面
//! 接线改造（defs 加 dispatch 改走 registry）候腿五收口批。
//!
//! A4 申报缺席形：ExternalPluginBridge 四面（spawn 加 send_request 加 recv
//! 加 shutdown）只立签名，缺省体全部返回 BridgeUnsupported，进程外桥实装
//! 候后批（实装批如须新增 crate 依赖须单独申报裁决，SPEC-025 A6）。
//!
//! 零 LLM 零网络红线承袭（SPEC-025 A9）：本模块零新增 Cargo 依赖
//! （async-trait 加 tokio 加 serde 加 serde_json 加 thiserror 全在 Cargo）。

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;

// ------------------------------------------------------------------- errors

/// registry 面错误三态承载：未找到（调度）、重名（注册）、参坏（调用面），
/// 加 manifest 两态与桥缺席态（A4）。
#[derive(Debug, Error)]
pub enum ToolError {
    #[error("tool not registered: {0}")]
    NotFound(String),
    #[error("duplicate tool name: {0}")]
    DuplicateTool(String),
    #[error("tool `{tool}` invalid arguments: {reason}")]
    InvalidArguments { tool: String, reason: String },
    #[error("tool `{tool}` execution failed: {reason}")]
    Execution { tool: String, reason: String },
    #[error("plugin manifest invalid ({site}): {reason}")]
    ManifestInvalid { site: String, reason: String },
    #[error("plugin manifest io error ({site}): {reason}")]
    ManifestIo { site: String, reason: String },
    /// A4 申报缺席形：外部插件桥四面缺省体统一返回此态，进程外桥实装候后批。
    #[error("external plugin bridge not implemented (A4 declared absence; off-process bridge deferred to a later batch): {0}")]
    BridgeUnsupported(String),
    #[error("registry lock poisoned: {0}")]
    LockPoisoned(String),
}

// ------------------------------------------------------------ provider trait

/// 内置统一形 trait（SPEC-025 § 插件槽位协议）：全部工具实现四面，经
/// ToolRegistry 统一注册承载工具面，废弃散点注册。
#[async_trait]
pub trait ToolProvider: Send + Sync {
    /// 工具注册名（registry 调度键，MCP tools 面同名承载）。
    fn name(&self) -> &str;
    /// 工具描述（冻结契约文本：静态、只载操作语义不含可执行指令面）。
    fn description(&self) -> &str;
    /// 入参 schema（JSON Schema 形，MCP input_schema 同源）。
    fn input_schema(&self) -> Value;
    /// 调用：入参出参全 JSON Value，错误以 ToolError 三态显形。
    async fn call(&self, args: Value) -> Result<Value, ToolError>;
}

// ----------------------------------------------------------------- registry

/// 注册表列单读数：一具工具一元组（name 加 description 加 input_schema）。
#[derive(Debug, Clone, PartialEq)]
pub struct ToolDescriptor {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// ToolRegistry：动态注册形（SPEC-025 A3 统一注册）。
///
/// 存储取 `Vec<Arc<dyn ToolProvider>>` 加同步 RwLock：call 时短锁内 Arc
/// clone 即出，await 恒在锁外——std 锁 guard 不跨 await，future 保 Send。
#[derive(Default)]
pub struct ToolRegistry {
    providers: RwLock<Vec<Arc<dyn ToolProvider>>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(Vec::new()),
        }
    }

    /// 注册一具 provider；同名拒（重名即注册面歧义，fail closed）。
    pub fn register(&self, provider: Box<dyn ToolProvider>) -> Result<(), ToolError> {
        let provider: Arc<dyn ToolProvider> = Arc::from(provider);
        let mut guard = self.write_guard()?;
        if guard.iter().any(|p| p.name() == provider.name()) {
            return Err(ToolError::DuplicateTool(provider.name().to_string()));
        }
        guard.push(provider);
        Ok(())
    }

    /// 全量列单（注册序，不排序）；registry 零工具即空列绿态。
    pub fn list(&self) -> Result<Vec<ToolDescriptor>, ToolError> {
        let guard = self.read_guard()?;
        Ok(guard
            .iter()
            .map(|p| ToolDescriptor {
                name: p.name().to_string(),
                description: p.description().to_string(),
                input_schema: p.input_schema(),
            })
            .collect())
    }

    /// 按名调度调用：参出参即 provider 面原样，registry 层零变换。
    pub async fn call(&self, name: &str, args: Value) -> Result<Value, ToolError> {
        let provider = {
            let guard = self.read_guard()?;
            guard.iter().find(|p| p.name() == name).cloned()
        };
        match provider {
            Some(p) => p.call(args).await,
            None => Err(ToolError::NotFound(name.to_string())),
        }
    }

    fn read_guard(&self) -> Result<RwLockReadGuard<'_, Vec<Arc<dyn ToolProvider>>>, ToolError> {
        self.providers
            .read()
            .map_err(|p| ToolError::LockPoisoned(p.to_string()))
    }

    fn write_guard(&self) -> Result<RwLockWriteGuard<'_, Vec<Arc<dyn ToolProvider>>>, ToolError> {
        self.providers
            .write()
            .map_err(|p| ToolError::LockPoisoned(p.to_string()))
    }
}

// --------------------------------------------------- demo migration (leg 5)

/// 在役 heartbeat 工具描述（与 src/mcpserver/server.rs desc_heart 冻结同形：
/// 同名同描述同 schema，MCP 面零变）。
pub const HEARTBEAT_DESCRIPTION: &str = "心跳：秤星三维最新读数与距上快照间隔日（只读不落链）。Heartbeat: latest gauge tri-dimension readings and days since last snapshot (read-only). 承接 CLI：cd sih-tools/gauge 且 PYTHONPATH=src python3 -m gauge.cli read。正典：sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md；sih-engine/sih/state/plan/mcpline-line-v1.md";

/// 演示迁移（SPEC-025 腿五 · A3 预览形）：在役只读工具 heartbeat
/// （mcpserver::alpha::heartbeat）包装为 ToolProvider。
///
/// call 零变换直委托：heartbeat 为零参工具，入参忽略，出参（成功读数与
/// error_payload 读数）原样 Value 透传，同参同出参。MCP 面接线改造候后批
/// （见模块头注）。
pub struct HeartbeatProvider;

#[async_trait]
impl ToolProvider for HeartbeatProvider {
    fn name(&self) -> &str {
        "heartbeat"
    }

    fn description(&self) -> &str {
        HEARTBEAT_DESCRIPTION
    }

    fn input_schema(&self) -> Value {
        json!({ "properties": {}, "required": [], "type": "object" })
    }

    async fn call(&self, _args: Value) -> Result<Value, ToolError> {
        Ok(crate::mcpserver::alpha::heartbeat().await)
    }
}

// ----------------------------------------------------------------- manifest

/// 插件清单文件名（扫描形 sih/plugins/<name>/plugin.json）。
pub const PLUGIN_MANIFEST_FILE: &str = "plugin.json";
/// 协议唯一 transport（v1 只立 stdio 进程外形）。
pub const PLUGIN_TRANSPORT_STDIO: &str = "stdio";

/// 插件清单 tools 数组条目：name 必填，description 可选（缺省空串）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginToolEntry {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// 插件清单：sih/plugins/<name>/plugin.json 五键（name、version、
/// transport=stdio、command、tools 数组）。
///
/// 严形：未知字段拒（typo 面）；前向扩展走新 version 值另立，不在 v1 形上
/// 静默放宽。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub transport: String,
    pub command: String,
    pub tools: Vec<PluginToolEntry>,
}

impl PluginManifest {
    /// 从清单 JSON 文本反序列化加语义校验。
    pub fn parse(json_str: &str) -> Result<Self, ToolError> {
        let manifest: PluginManifest =
            serde_json::from_str(json_str).map_err(|e| ToolError::ManifestInvalid {
                site: "inline".to_string(),
                reason: e.to_string(),
            })?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// 五键语义校验：transport 固定 stdio；name 非空且不含路径分隔符
    /// （name 即目录键）；version 与 command 非空；tools 条目名非空不重。
    pub fn validate(&self) -> Result<(), ToolError> {
        let bad = |reason: String| ToolError::ManifestInvalid {
            site: format!("plugin `{}`", self.name),
            reason,
        };
        if self.name.trim().is_empty() {
            return Err(bad("name 必填非空".to_string()));
        }
        if self.name.contains('/') || self.name.contains('\\') || self.name == "." || self.name == ".." {
            return Err(bad(format!(
                "name `{}` 不得含路径分隔符或为相对位",
                self.name
            )));
        }
        if self.version.trim().is_empty() {
            return Err(bad("version 必填非空".to_string()));
        }
        if self.transport != PLUGIN_TRANSPORT_STDIO {
            return Err(bad(format!(
                "transport 须为 `{PLUGIN_TRANSPORT_STDIO}`，得 `{}`",
                self.transport
            )));
        }
        if self.command.trim().is_empty() {
            return Err(bad("command 必填非空".to_string()));
        }
        let mut seen = BTreeSet::new();
        for t in &self.tools {
            if t.name.trim().is_empty() {
                return Err(bad("tools[] 含空名条目".to_string()));
            }
            if !seen.insert(t.name.clone()) {
                return Err(bad(format!("tools[] 工具名重复：`{}`", t.name)));
            }
        }
        Ok(())
    }
}

/// 扫描读出：一插件一清单加其清单文件路径。
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub path: PathBuf,
}

/// 扫描 `<plugins_root>/*/plugin.json`，一级目录逐件展开。
///
/// fail closed：根目录缺席或子目录缺清单即 Io 错，清单解析或校验不过即
/// Invalid 错，零跳过零静默降级——插件面是协议面，坏件跳过会吞缺席语义。
/// 根目录在而零插件子目录即空列绿态（对表泊界空目录绿态先例）。
pub fn load_manifests(plugins_root: &Path) -> Result<Vec<LoadedPlugin>, ToolError> {
    let root_site = plugins_root.display().to_string();
    if !plugins_root.is_dir() {
        return Err(ToolError::ManifestIo {
            site: root_site,
            reason: "plugins 根目录缺席".to_string(),
        });
    }
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(plugins_root)
        .map_err(|e| ToolError::ManifestIo {
            site: root_site,
            reason: e.to_string(),
        })?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    let mut loaded = Vec::new();
    for dir in dirs {
        let path = dir.join(PLUGIN_MANIFEST_FILE);
        let site = path.display().to_string();
        if !path.is_file() {
            return Err(ToolError::ManifestIo {
                site,
                reason: format!("{PLUGIN_MANIFEST_FILE} 缺席"),
            });
        }
        let raw = std::fs::read_to_string(&path).map_err(|e| ToolError::ManifestIo {
            site: site.clone(),
            reason: e.to_string(),
        })?;
        let manifest: PluginManifest =
            serde_json::from_str(&raw).map_err(|e| ToolError::ManifestInvalid {
                site: site.clone(),
                reason: e.to_string(),
            })?;
        manifest.validate()?;
        loaded.push(LoadedPlugin { manifest, path });
    }
    Ok(loaded)
}

// ----------------------------------------------------- external bridge (A4)

/// 外部插件桥接口声明（SPEC-025 § 插件槽位协议 · A4 申报缺席形）。
///
/// 本批只立接口签名：spawn 进程加 JSON-RPC 收发四面。缺省体全部返回
/// `ToolError::BridgeUnsupported`（显式缺席错误，非 panic），进程外桥实装
/// 候后批；实装批如须新增 crate 依赖须单独申报裁决（A6）。
#[async_trait]
pub trait ExternalPluginBridge: Send + Sync {
    /// 按 manifest command 启动插件子进程（stdio JSON-RPC），成功返回会话
    /// 句柄读出。
    async fn spawn(&self, manifest: &PluginManifest) -> Result<Value, ToolError> {
        Err(ToolError::BridgeUnsupported(format!(
            "spawn `{}`",
            manifest.name
        )))
    }

    /// 发一帧 JSON-RPC request 并候其 response。
    async fn send_request(&self, plugin: &str, request: Value) -> Result<Value, ToolError> {
        let _ = request;
        Err(ToolError::BridgeUnsupported(format!(
            "send_request `{plugin}`"
        )))
    }

    /// 收一帧 JSON-RPC（response 或 notification）。
    async fn recv(&self, plugin: &str) -> Result<Value, ToolError> {
        Err(ToolError::BridgeUnsupported(format!("recv `{plugin}`")))
    }

    /// 停插件子进程（stdin 收口加 wait）。
    async fn shutdown(&self, plugin: &str) -> Result<Value, ToolError> {
        Err(ToolError::BridgeUnsupported(format!(
            "shutdown `{plugin}`"
        )))
    }
}

// ------------------------------------------------- external bridge (A4 转正)

/// 进程外插件桥实装形（SPEC-025 § 插件槽位协议，A4 缺席申报转正）。
///
/// 协议定形：插件子进程按 manifest.command 起（stdio 管道），帧为行分隔
/// JSON-RPC 2.0——请求 `{"jsonrpc":"2.0","id":N,"method":"call","params":…}`
/// 响应 `{"jsonrpc":"2.0","id":N,"result":…}`（error 形映射 Execution）。
/// 零网络纯 stdio 承 A9；零新增依赖（tokio process 与 io 全在既有 feature
/// 面）承 A6。
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout};
use tokio::sync::Mutex as TMutex;

struct PluginSession {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

/// 共享会话表桥本体（Clone 共享同表，Arc 内嵌）。
#[derive(Clone, Default)]
pub struct StdioPluginBridge {
    sessions: Arc<TMutex<HashMap<String, PluginSession>>>,
}

impl StdioPluginBridge {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ExternalPluginBridge for StdioPluginBridge {
    async fn spawn(&self, manifest: &PluginManifest) -> Result<Value, ToolError> {
        let mut child = tokio::process::Command::new(&manifest.command)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| ToolError::Execution {
                tool: manifest.name.clone(),
                reason: format!("插件进程启动失败: {e}"),
            })?;
        let stdin = child.stdin.take().ok_or_else(|| ToolError::Execution {
            tool: manifest.name.clone(),
            reason: "stdin 管道不可得".into(),
        })?;
        let stdout = child.stdout.take().ok_or_else(|| ToolError::Execution {
            tool: manifest.name.clone(),
            reason: "stdout 管道不可得".into(),
        })?;
        self.sessions.lock().await.insert(
            manifest.name.clone(),
            PluginSession {
                child,
                stdin,
                stdout: BufReader::new(stdout),
                next_id: 1,
            },
        );
        Ok(json!({
            "plugin": manifest.name,
            "transport": "stdio",
            "protocol": "line-delimited jsonrpc-2.0",
            "tools": manifest.tools,
        }))
    }

    async fn send_request(&self, plugin: &str, request: Value) -> Result<Value, ToolError> {
        let mut guard = self.sessions.lock().await;
        let session = guard
            .get_mut(plugin)
            .ok_or_else(|| ToolError::NotFound(format!("plugin `{plugin}` 未启动")))?;
        let id = session.next_id;
        session.next_id += 1;
        let mut frame = request;
        frame["jsonrpc"] = json!("2.0");
        frame["id"] = json!(id);
        let mut line = serde_json::to_string(&frame).map_err(|e| ToolError::Execution {
            tool: plugin.to_string(),
            reason: format!("请求序列化失败: {e}"),
        })?;
        line.push('\n');
        session
            .stdin
            .write_all(line.as_bytes())
            .await
            .map_err(|e| ToolError::Execution { tool: plugin.to_string(), reason: format!("请求写入失败: {e}") })?;
        session
            .stdin
            .flush()
            .await
            .map_err(|e| ToolError::Execution { tool: plugin.to_string(), reason: format!("请求冲刷失败: {e}") })?;
        let mut resp_line = String::new();
        session
            .stdout
            .read_line(&mut resp_line)
            .await
            .map_err(|e| ToolError::Execution { tool: plugin.to_string(), reason: format!("响应读取失败: {e}") })?;
        if resp_line.trim().is_empty() {
            return Err(ToolError::Execution {
                tool: plugin.to_string(),
                reason: "插件进程响应空帧（可能已退出）".into(),
            });
        }
        let resp: Value = serde_json::from_str(resp_line.trim()).map_err(|e| ToolError::Execution {
            tool: plugin.to_string(),
            reason: format!("响应非 JSON: {e}"),
        })?;
        if let Some(err) = resp.get("error") {
            return Err(ToolError::Execution {
                tool: plugin.to_string(),
                reason: err.to_string(),
            });
        }
        Ok(resp.get("result").cloned().unwrap_or(Value::Null))
    }

    async fn recv(&self, plugin: &str) -> Result<Value, ToolError> {
        // 请求-响应一一对应形：响应已在 send_request 同步候回，recv 为在场哨。
        let guard = self.sessions.lock().await;
        if guard.contains_key(plugin) {
            Ok(json!({"plugin": plugin, "state": "session-open"}))
        } else {
            Err(ToolError::NotFound(format!("plugin `{plugin}` 未启动")))
        }
    }

    async fn shutdown(&self, plugin: &str) -> Result<Value, ToolError> {
        let mut guard = self.sessions.lock().await;
        let mut session = guard
            .remove(plugin)
            .ok_or_else(|| ToolError::NotFound(format!("plugin `{plugin}` 未启动")))?;
        let _ = session.stdin.shutdown().await;
        // 健壮协议形：优雅 EOF 候 3 秒，超时 kill 兜底（外部插件可能不优雅退）。
        let status = match tokio::time::timeout(
            std::time::Duration::from_secs(3),
            session.child.wait(),
        )
        .await
        {
            Ok(s) => s.map_err(|e| ToolError::Execution {
                tool: plugin.to_string(),
                reason: format!("wait 失败: {e}"),
            })?,
            Err(_) => {
                let _ = session.child.kill().await;
                session.child.wait().await.map_err(|e| ToolError::Execution {
                    tool: plugin.to_string(),
                    reason: format!("kill 后 wait 失败: {e}"),
                })?
            }
        };
        Ok(json!({"plugin": plugin, "shutdown": true, "status": status.to_string()}))
    }
}

/// 外部插件工具经桥包装为 ToolProvider（tools/list 显形可调用）。
pub struct PluginToolProvider {
    pub tool_name: String,
    pub plugin_name: String,
    pub description: String,
    bridge: StdioPluginBridge,
}

#[async_trait]
impl ToolProvider for PluginToolProvider {
    fn name(&self) -> &str {
        &self.tool_name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn input_schema(&self) -> Value {
        json!({"properties": {}, "required": [], "type": "object"})
    }
    async fn call(&self, args: Value) -> Result<Value, ToolError> {
        self.bridge
            .send_request(
                &self.plugin_name,
                json!({"method": "call", "params": {"tool": self.tool_name, "args": args}}),
            )
            .await
    }
}

impl StdioPluginBridge {
    /// manifest 全具注册：spawn 起进程加每工具一 provider 入 registry。
    pub async fn register_plugin(
        &self,
        registry: &ToolRegistry,
        manifest: &PluginManifest,
    ) -> Result<Value, ToolError> {
        self.spawn(manifest).await?;
        for tool in &manifest.tools {
            registry.register(Box::new(PluginToolProvider {
                tool_name: tool.name.clone(),
                plugin_name: manifest.name.clone(),
                description: if tool.description.is_empty() {
                    format!(
                        "外部插件 {} 工具 {}（进程外 stdio JSON-RPC 桥）。正典指针：SPEC-025 插件槽位协议节。",
                        manifest.name, tool.name
                    )
                } else {
                    format!(
                        "{}（外部插件 {} 进程外 stdio JSON-RPC 桥。正典指针：SPEC-025 插件槽位协议节。）",
                        tool.description, manifest.name
                    )
                },
                bridge: self.clone(),
            }))?;
        }
        Ok(json!({"plugin": manifest.name, "registered": manifest.tools}))
    }
}
