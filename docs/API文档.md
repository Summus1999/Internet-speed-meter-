# API 文档

## 概述

本文档提供了网络速度测试工具库及其公共接口的详细 API 文档。

---

## 核心模块

### 1. 速度测试模块（`speed_test.rs`）

#### `SpeedTester` 结构体

```rust
pub struct SpeedTester {
    _server_url: String,
}
```

负责执行网络速度测试的主结构体。

##### 方法

###### `new(server_url: Option<String>) -> Self`

创建新的 `SpeedTester` 实例。

**参数:**
- `server_url: Option<String>` - 可选的自定义测试服务器 URL。如果未提供，使用默认测试服务器。

**返回:** 新的 `SpeedTester` 实例

**示例:**
```rust
let tester = SpeedTester::new(None);
let tester_custom = SpeedTester::new(Some(\"http://example.com/testfile\".to_string()));
```

###### `run_test(&self) -> Result<SpeedTestResult>`

执行完整的速度测试，包括 ping、下载和上传测量。

**返回:** `Result<SpeedTestResult>` 包含测试结果或错误

**行为:**
1. 初始化测试
2. 测量 ping 延迟
3. 测量下载速度
4. 测量上传速度
5. 计算总耗时
6. 返回结果

**示例:**
```rust
let tester = SpeedTester::new(None);
match tester.run_test().await {
    Ok(result) => result.print_summary(),
    Err(e) => println!(\"错误: {}\", e),
}
```

#### `SpeedTestResult` 结构体

```rust
pub struct SpeedTestResult {
    pub download_speed: f64,  // Mbps
    pub upload_speed: f64,    // Mbps
    pub ping: f64,            // ms
    pub test_duration: f64,   // 秒
}
```

包含速度测试结果。

##### 方法

###### `print_summary(&self)`

将速度测试结果的格式化总结打印到标准输出。

**输出格式:**
```
📊 速度测试结果：
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📥 下载速度: X.XX Mbps
📤 上传速度: X.XX Mbps
🔗 Ping: X.XX ms
⏱️  执行时间: X.XX s
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**示例:**
```rust
result.print_summary();
```

---

### 2. 网络诊断模块（`diagnostics.rs`）

#### `NetworkDiagnostics` 结构体

```rust
pub struct NetworkDiagnostics;
```

提供全面网络诊断功能。

##### 方法

###### `new() -> Self`

创建新的 `NetworkDiagnostics` 实例。

**返回:** 新的 `NetworkDiagnostics` 实例

**示例:**
```rust
let diagnostics = NetworkDiagnostics::new();
```

###### `run_full_diagnostic(&self, target_host: &str) -> Result<DiagnosticResult>`

执行完整的网络诊断分析。

**参数:**
- `target_host: &str` - 要诊断的主机（例如 \"8.8.8.8\"）

**返回:** `Result<DiagnosticResult>` 包含诊断结果或错误

**诊断检查:**
1. 检查网络连通性
2. 测试 DNS 解析
3. 检索 DNS 服务器
4. 测量延迟
5. 检查数据包丢失
6. 检测问题
7. 生成建议

**示例:**
```rust
let diagnostics = NetworkDiagnostics::new();
match diagnostics.run_full_diagnostic(\"8.8.8.8\").await {
    Ok(result) => result.print_report(),
    Err(e) => println!(\"错误: {}\", e),
}
```

#### `DiagnosticResult` 结构体

```rust
pub struct DiagnosticResult {
    pub target_host: String,
    pub connectivity: bool,
    pub dns_resolution: bool,
    pub dns_servers: Vec<String>,
    pub average_latency: f64,
    pub packet_loss: f64,
    pub issues_detected: Vec<String>,
    pub recommendations: Vec<String>,
}
```

包含全面的诊断结果。

##### 方法

###### `print_report(&self)`

将详细的诊断报告打印到标准输出。

**输出格式:**
```
📋 网络诊断报告
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🎯 目标: <主机>

✅ 状态:
   • 连通性: ✓/✗ <状态>
   • DNS 解析: ✓/✗ <状态>

🔍 DNS 服务器:
   1. <服务器1>
   2. <服务器2>
   ...

📊 网络指标:
   • 平均延迟: X.XX ms
   • 数据包丢失: X.XX%

[⚠️  检测到问题 | ✅ 未检测到问题]

💡 建议:
   1. <建议1>
   ...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**示例:**
```rust
result.print_report();
```

---

### 3. 工具模块（`utils.rs`）

#### 函数

###### `get_timestamp() -> String`

获取当前时间戳的格式化字符串。

**返回:** 格式为 \"YYYY-MM-DD HH:MM:SS\" 的字符串

**示例:**
```rust
let timestamp = get_timestamp();
println!(\"测试开始于: {}\", timestamp);
```

###### `format_bytes(bytes: u64) -> String`

将字节数转换为人类可读的格式。

**参数:**
- `bytes: u64` - 字节数

**返回:** 格式化的字符串（例如 \"1.50 MB\"）

**单位:** B, KB, MB, GB

**示例:**
```rust
println!(\"文件大小: {}\", format_bytes(1_048_576));  // \"1.00 MB\"
```

###### `calculate_speed(bytes: u64, duration_secs: f64) -> f64`

根据数据大小和持续时间计算网络速度。

**参数:**
- `bytes: u64` - 传输的字节数
- `duration_secs: f64` - 持续时间（秒）

**返回:** 速度（Mbps）

**公式:** (bytes * 8) / (duration_secs * 1,000,000)

**示例:**
```rust
let speed = calculate_speed(1_000_000, 1.0);  // 8.0 Mbps
```

---

## 错误处理

所有公共函数都返回 `Result<T>`，其中：
- `Ok(T)` - 成功操作结果
- `Err(anyhow::Error)` - 带有上下文消息的错误

**错误传播:**
```rust
match operation.await {
    Ok(result) => {
        // 处理成功
    }
    Err(e) => {
        eprintln!(\"操作失败: {}\", e);
    }
}
```

---

## 异步/等待用法

所有 I/O 操作都实现为异步函数，必须使用 await：

```rust
// 使用 Tokio 运行时
#[tokio::main]
async fn main() -> Result<()> {
    let tester = SpeedTester::new(None);
    let result = tester.run_test().await?;
    result.print_summary();
    Ok(())
}
```

---

## 数据结构

### SpeedTestResult 字段

| 字段 | 类型 | 单位 | 描述 |
|------|------|------|------|
| `download_speed` | `f64` | Mbps | 下载带宽 |
| `upload_speed` | `f64` | Mbps | 上传带宽 |
| `ping` | `f64` | ms | 网络延迟 |
| `test_duration` | `f64` | 秒 | 总测试耗时 |

### DiagnosticResult 字段

| 字段 | 类型 | 描述 |
|------|------|------|
| `target_host` | `String` | 诊断的目标主机 |
| `connectivity` | `bool` | 网络连通性状态 |
| `dns_resolution` | `bool` | DNS 解析状态 |
| `dns_servers` | `Vec<String>` | DNS 服务器列表 |
| `average_latency` | `f64` | 平均延迟（毫秒） |
| `packet_loss` | `f64` | 数据包丢失百分比 |
| `issues_detected` | `Vec<String>` | 检测到的问题列表 |
| `recommendations` | `Vec<String>` | 建议列表 |

---

## 使用示例

### 完整速度测试示例

```rust
use internet_speed_meter::speed_test::SpeedTester;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 使用默认服务器创建测试器
    let tester = SpeedTester::new(None);
    
    // 运行测试
    let result = tester.run_test().await?;
    
    // 显示结果
    result.print_summary();
    
    // 访问各个指标
    println!(\"下载: {} Mbps\", result.download_speed);
    println!(\"上传: {} Mbps\", result.upload_speed);
    println!(\"Ping: {} ms\", result.ping);
    
    Ok(())
}
```

### 完整诊断示例

```rust
use internet_speed_meter::diagnostics::NetworkDiagnostics;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建诊断工具
    let diag = NetworkDiagnostics::new();
    
    // 运行诊断
    let result = diag.run_full_diagnostic(\"8.8.8.8\").await?;
    
    // 显示报告
    result.print_report();
    
    // 检查特定字段
    println!(\"已连接: {}\", result.connectivity);
    println!(\"DNS 工作: {}\", result.dns_resolution);
    println!(\"问题: {:?}\", result.issues_detected);
    
    Ok(())
}
```

---

## 版本信息

- **当前版本**: 0.1.0
- **编辑**: 2021
- **MSRV**: Rust 1.70+

---

## 许可证

查看 LICENSE 文件了解详情。
