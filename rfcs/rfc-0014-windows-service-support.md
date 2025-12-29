# RFC-0014: Windows Service Support for ACP Daemon

- **RFC ID**: 0014
- **Title**: Windows Service Support for ACP Daemon
- **Author**: ACP Contributors
- **Status**: Draft
- **Created**: 2025-12-29
- **Updated**: 2025-12-29
- **Discussion**: [Pending GitHub Discussion]
- **Related**: acp-daemon v0.1.0

---

## Summary

This RFC proposes adding native Windows Service support to `acp-daemon`, enabling the HTTP REST API server to run as a proper background service on Windows. The implementation uses the `windows-service` crate for Windows Service Control Manager (SCM) integration while maintaining the existing Unix daemonization approach. A platform abstraction layer ensures consistent behavior and CLI experience across all supported platforms.

---

## Motivation

### Problem Statement

The ACP Daemon (`acpd`) provides an HTTP REST API for codebase intelligence, exposing endpoints for querying symbols, files, call graphs, and other indexed data. Currently, the daemon cannot be built or run on Windows due to its dependency on Unix-specific APIs:

1. **`daemonize` crate**: Uses Unix system calls (`fork()`, `setsid()`, `chroot()`) that don't exist on Windows
2. **`nix` crate**: Used for signal handling (`SIGTERM`) and process management
3. **PID file semantics**: Assumes Unix process model

This limitation excludes Windows developers from using the daemon's REST API capabilities, forcing them to rely solely on the CLI (`acp`) or MCP server (`acp-mcp`) for integration.

### Current Architecture

The daemon's lifecycle management in `src/lifecycle.rs` currently:

```rust
// Unix-only: Spawn background process
pub fn start_daemon(project_root: &Path, port: u16) -> Result<()> {
    let child = Command::new(std::env::current_exe()?)
        .args(["run", "--port", &port.to_string()])
        .current_dir(project_root)
        .spawn()?;

    // Write PID to file
    let pid_file = project_root.join(".acp/daemon.pid");
    std::fs::write(&pid_file, child.id().to_string())?;
    Ok(())
}

// Unix-only: Stop via SIGTERM
pub fn stop_daemon(project_root: &Path) -> Result<()> {
    let pid = read_pid(project_root)?;
    #[cfg(unix)]
    {
        use nix::sys::signal::{self, Signal};
        use nix::unistd::Pid;
        signal::kill(Pid::from_raw(pid as i32), Signal::SIGTERM)?;
    }
    Ok(())
}
```

### Goals

1. **Native Windows Service integration**: Use Windows Service Control Manager (SCM) for proper service lifecycle management
2. **Platform abstraction**: Create a clean abstraction layer that encapsulates platform-specific daemon behavior
3. **Unified CLI experience**: Maintain consistent `acpd start/stop/status` commands across platforms
4. **Minimal Unix disruption**: Preserve existing Unix behavior without regression
5. **Production-ready**: Support service installation, configuration, and proper Windows event logging

### Non-Goals

1. **macOS launchd integration**: Native macOS service management is out of scope (future RFC)
2. **Linux systemd units**: While useful, generating systemd unit files is out of scope (future RFC)
3. **GUI service manager**: No graphical interface for service management
4. **Remote service management**: Only local service control is supported

---

## Detailed Design

### Overview

The implementation introduces a platform abstraction layer with trait-based polymorphism, allowing platform-specific implementations while maintaining a unified interface:

```
┌─────────────────────────────────────────────────────────┐
│                      CLI Layer                          │
│              acpd start / stop / status                 │
└─────────────────────────┬───────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────┐
│                 Platform Abstraction                     │
│                  trait DaemonManager                     │
└──────────┬──────────────────────────────┬───────────────┘
           │                              │
┌──────────▼──────────┐      ┌────────────▼────────────┐
│   Unix Manager      │      │   Windows Manager       │
│  - fork/spawn       │      │  - SCM integration      │
│  - PID files        │      │  - Service registration │
│  - SIGTERM          │      │  - Event log            │
└─────────────────────┘      └─────────────────────────┘
```

### Platform Abstraction Trait

Define a trait in `src/platform/mod.rs`:

```rust
//! Platform-specific daemon management abstraction.

use std::path::Path;
use anyhow::Result;

/// Daemon lifecycle management trait.
///
/// Implementations handle platform-specific details of running
/// the daemon as a background service.
pub trait DaemonManager: Send + Sync {
    /// Start the daemon as a background service.
    ///
    /// On Unix: Spawns a child process and writes PID file.
    /// On Windows: Registers and starts a Windows Service.
    fn start(&self, project_root: &Path, port: u16) -> Result<()>;

    /// Stop the running daemon.
    ///
    /// On Unix: Sends SIGTERM to the process.
    /// On Windows: Sends stop signal to the service.
    fn stop(&self, project_root: &Path) -> Result<()>;

    /// Check if the daemon is currently running.
    ///
    /// On Unix: Checks if process exists via signal 0.
    /// On Windows: Queries service state from SCM.
    fn status(&self, project_root: &Path) -> Result<DaemonStatus>;

    /// Get the daemon's port if running.
    fn port(&self, project_root: &Path) -> Result<Option<u16>>;
}

/// Daemon status information.
#[derive(Debug, Clone, PartialEq)]
pub enum DaemonStatus {
    /// Daemon is running.
    Running { pid: Option<u32>, port: u16 },
    /// Daemon is stopped.
    Stopped,
    /// Daemon state is unknown or transitioning.
    Unknown,
}

/// Get the platform-appropriate daemon manager.
pub fn daemon_manager() -> Box<dyn DaemonManager> {
    #[cfg(unix)]
    { Box::new(unix::UnixDaemonManager::new()) }

    #[cfg(windows)]
    { Box::new(windows::WindowsDaemonManager::new()) }
}

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;
```

### Unix Implementation

Preserve existing behavior in `src/platform/unix.rs`:

```rust
//! Unix daemon management using process spawning and signals.

use super::{DaemonManager, DaemonStatus};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

pub struct UnixDaemonManager;

impl UnixDaemonManager {
    pub fn new() -> Self {
        Self
    }

    fn pid_file(project_root: &Path) -> std::path::PathBuf {
        project_root.join(".acp/daemon.pid")
    }

    fn read_pid(project_root: &Path) -> Result<Option<u32>> {
        let pid_file = Self::pid_file(project_root);
        if !pid_file.exists() {
            return Ok(None);
        }
        let contents = std::fs::read_to_string(&pid_file)?;
        let pid: u32 = contents.trim().parse()?;
        Ok(Some(pid))
    }

    fn is_process_running(pid: u32) -> bool {
        signal::kill(Pid::from_raw(pid as i32), None).is_ok()
    }
}

impl DaemonManager for UnixDaemonManager {
    fn start(&self, project_root: &Path, port: u16) -> Result<()> {
        // Check if already running
        if let Ok(DaemonStatus::Running { .. }) = self.status(project_root) {
            anyhow::bail!("Daemon is already running");
        }

        // Spawn background process
        let child = Command::new(std::env::current_exe()?)
            .args(["run", "--port", &port.to_string()])
            .current_dir(project_root)
            .stdout(std::fs::File::create(project_root.join(".acp/daemon.log"))?)
            .stderr(std::fs::File::create(project_root.join(".acp/daemon.log"))?)
            .spawn()
            .context("Failed to spawn daemon process")?;

        // Write PID file
        let pid_file = Self::pid_file(project_root);
        std::fs::create_dir_all(pid_file.parent().unwrap())?;
        std::fs::write(&pid_file, child.id().to_string())?;

        Ok(())
    }

    fn stop(&self, project_root: &Path) -> Result<()> {
        let pid = Self::read_pid(project_root)?
            .context("No PID file found - daemon may not be running")?;

        signal::kill(Pid::from_raw(pid as i32), Signal::SIGTERM)
            .context("Failed to send SIGTERM to daemon")?;

        // Clean up PID file
        let _ = std::fs::remove_file(Self::pid_file(project_root));

        Ok(())
    }

    fn status(&self, project_root: &Path) -> Result<DaemonStatus> {
        match Self::read_pid(project_root)? {
            Some(pid) if Self::is_process_running(pid) => {
                let port = self.port(project_root)?.unwrap_or(9222);
                Ok(DaemonStatus::Running { pid: Some(pid), port })
            }
            Some(_) => {
                // PID file exists but process is gone - clean up
                let _ = std::fs::remove_file(Self::pid_file(project_root));
                Ok(DaemonStatus::Stopped)
            }
            None => Ok(DaemonStatus::Stopped),
        }
    }

    fn port(&self, project_root: &Path) -> Result<Option<u16>> {
        // Read port from config or default
        let config_file = project_root.join(".acp/daemon.port");
        if config_file.exists() {
            let port: u16 = std::fs::read_to_string(&config_file)?.trim().parse()?;
            return Ok(Some(port));
        }
        Ok(Some(9222)) // Default port
    }
}
```

### Windows Implementation

New implementation in `src/platform/windows.rs`:

```rust
//! Windows Service management using the Windows Service Control Manager.

use super::{DaemonManager, DaemonStatus};
use anyhow::{Context, Result};
use std::ffi::OsString;
use std::path::Path;
use std::time::Duration;
use windows_service::{
    define_windows_service,
    service::{
        ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl,
        ServiceExitCode, ServiceInfo, ServiceStartType, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher, service_manager::{ServiceManager, ServiceManagerAccess},
};

const SERVICE_NAME: &str = "AcpDaemon";
const SERVICE_DISPLAY_NAME: &str = "ACP Daemon";
const SERVICE_DESCRIPTION: &str = "ACP Daemon - HTTP REST API for codebase intelligence";

pub struct WindowsDaemonManager;

impl WindowsDaemonManager {
    pub fn new() -> Self {
        Self
    }

    /// Install the Windows Service (requires admin privileges).
    pub fn install_service(project_root: &Path, port: u16) -> Result<()> {
        let manager = ServiceManager::local_computer(
            None::<&str>,
            ServiceManagerAccess::CREATE_SERVICE,
        )?;

        let service_binary_path = std::env::current_exe()?;
        let service_args = format!(
            "service-run --project \"{}\" --port {}",
            project_root.display(),
            port
        );

        let service_info = ServiceInfo {
            name: OsString::from(SERVICE_NAME),
            display_name: OsString::from(SERVICE_DISPLAY_NAME),
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::OnDemand,
            error_control: ServiceErrorControl::Normal,
            executable_path: service_binary_path,
            launch_arguments: vec![OsString::from(service_args)],
            dependencies: vec![],
            account_name: None, // LocalSystem
            account_password: None,
        };

        let service = manager.create_service(&service_info, ServiceAccess::ALL_ACCESS)?;
        service.set_description(SERVICE_DESCRIPTION)?;

        Ok(())
    }

    /// Uninstall the Windows Service.
    pub fn uninstall_service() -> Result<()> {
        let manager = ServiceManager::local_computer(
            None::<&str>,
            ServiceManagerAccess::CONNECT,
        )?;

        let service = manager.open_service(SERVICE_NAME, ServiceAccess::DELETE)?;
        service.delete()?;

        Ok(())
    }

    fn get_service_status() -> Result<Option<ServiceState>> {
        let manager = ServiceManager::local_computer(
            None::<&str>,
            ServiceManagerAccess::CONNECT,
        )?;

        match manager.open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS) {
            Ok(service) => {
                let status = service.query_status()?;
                Ok(Some(status.current_state))
            }
            Err(_) => Ok(None), // Service not installed
        }
    }
}

impl DaemonManager for WindowsDaemonManager {
    fn start(&self, project_root: &Path, port: u16) -> Result<()> {
        // Check if service is installed
        let status = Self::get_service_status()?;

        if status.is_none() {
            // Auto-install service if not present
            Self::install_service(project_root, port)
                .context("Failed to install Windows Service")?;
        }

        // Start the service
        let manager = ServiceManager::local_computer(
            None::<&str>,
            ServiceManagerAccess::CONNECT,
        )?;

        let service = manager.open_service(SERVICE_NAME, ServiceAccess::START)?;
        service.start::<OsString>(&[])?;

        Ok(())
    }

    fn stop(&self, project_root: &Path) -> Result<()> {
        let manager = ServiceManager::local_computer(
            None::<&str>,
            ServiceManagerAccess::CONNECT,
        )?;

        let service = manager.open_service(SERVICE_NAME, ServiceAccess::STOP)?;
        service.stop()?;

        Ok(())
    }

    fn status(&self, project_root: &Path) -> Result<DaemonStatus> {
        match Self::get_service_status()? {
            Some(ServiceState::Running) => {
                let port = self.port(project_root)?.unwrap_or(9222);
                Ok(DaemonStatus::Running { pid: None, port })
            }
            Some(ServiceState::Stopped) => Ok(DaemonStatus::Stopped),
            Some(_) => Ok(DaemonStatus::Unknown),
            None => Ok(DaemonStatus::Stopped), // Service not installed
        }
    }

    fn port(&self, project_root: &Path) -> Result<Option<u16>> {
        // Read port from registry or config file
        let config_file = project_root.join(".acp/daemon.port");
        if config_file.exists() {
            let port: u16 = std::fs::read_to_string(&config_file)?.trim().parse()?;
            return Ok(Some(port));
        }
        Ok(Some(9222))
    }
}

// Windows service entry point
define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<OsString>) {
    if let Err(e) = run_service() {
        // Log error to Windows Event Log
        eprintln!("Service error: {}", e);
    }
}

fn run_service() -> Result<()> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop | ServiceControl::Shutdown => {
                // Signal the HTTP server to shut down
                // This would use a shared atomic or channel
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    // Report running status
    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // Start the HTTP server (blocking)
    // This would call the same run_foreground() used on Unix

    Ok(())
}
```

### CLI Changes

Add service management commands for Windows:

```rust
#[derive(Subcommand)]
pub enum Commands {
    /// Start the daemon
    Start {
        #[arg(short, long, default_value = "9222")]
        port: u16,
    },

    /// Stop the daemon
    Stop,

    /// Check daemon status
    Status,

    /// Run in foreground (used internally)
    Run {
        #[arg(short, long, default_value = "9222")]
        port: u16,
    },

    /// Windows Service management (Windows only)
    #[cfg(windows)]
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },
}

#[cfg(windows)]
#[derive(Subcommand)]
pub enum ServiceAction {
    /// Install as Windows Service
    Install {
        #[arg(short, long, default_value = "9222")]
        port: u16,
    },
    /// Uninstall Windows Service
    Uninstall,
    /// Internal: Run as Windows Service
    #[clap(hide = true)]
    Run {
        #[arg(long)]
        project: String,
        #[arg(long, default_value = "9222")]
        port: u16,
    },
}
```

### Cargo.toml Changes

```toml
[package]
name = "acp-daemon"
version = "0.2.0"
edition = "2021"

[[bin]]
name = "acpd"
path = "src/main.rs"

[dependencies]
# Core dependencies (cross-platform)
acp = { version = "0.3.0", package = "acp-protocol" }
axum = "0.8"
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace"] }
tokio = { version = "1.48", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
clap = { version = "4.5", features = ["derive"] }
console = "0.15"
notify = "8.2"
chrono = { version = "0.4", features = ["serde"] }

# Unix-specific dependencies
[target.'cfg(unix)'.dependencies]
nix = { version = "0.29", features = ["signal", "process"] }

# Windows-specific dependencies
[target.'cfg(windows)'.dependencies]
windows-service = "0.7"

[dev-dependencies]
tempfile = "3.15"
reqwest = { version = "0.12", features = ["json"] }
```

### Error Handling

| Error Condition | Unix Behavior | Windows Behavior |
|-----------------|---------------|------------------|
| Daemon already running | Return error with PID | Return error with service state |
| Permission denied | EPERM error | Access denied error |
| Service not installed | N/A | Auto-install or prompt |
| Stop failed | ESRCH if process gone | Service state error |
| Port in use | Bind error at startup | Bind error at startup |

---

## Examples

### Basic Usage (Cross-Platform)

```bash
# Start daemon on default port
acpd start

# Start daemon on custom port
acpd start --port 3000

# Check status
acpd status
# Output: Daemon running on port 9222 (PID: 12345)

# Stop daemon
acpd stop
```

### Windows Service Management

```powershell
# Install as Windows Service (requires admin)
acpd service install --port 9222

# The service can now be managed via:
# - Services MMC snap-in (services.msc)
# - PowerShell: Start-Service AcpDaemon / Stop-Service AcpDaemon
# - sc.exe: sc start AcpDaemon / sc stop AcpDaemon

# Uninstall service
acpd service uninstall
```

### Programmatic Access (All Platforms)

```bash
# Once running, access the REST API
curl http://localhost:9222/health
# {"status": "ok"}

curl http://localhost:9222/symbols
# [{"name": "UserService", "kind": "class", ...}]
```

---

## Drawbacks

1. **Increased complexity**: Platform abstraction adds code and cognitive overhead
   - *Mitigation*: Clean trait boundaries isolate platform code

2. **Windows testing requirements**: CI must include Windows builds and tests
   - *Mitigation*: GitHub Actions supports Windows runners

3. **Admin privileges for service install**: Windows Service installation requires elevation
   - *Mitigation*: Clear error messages and documentation; `acpd start` can work without install via foreground mode

4. **Potential feature drift**: Platform-specific features may diverge over time
   - *Mitigation*: Unified test suite covering all platforms

5. **Maintenance burden**: Two codepaths to maintain
   - *Mitigation*: HTTP server and API are shared; only lifecycle differs

---

## Alternatives

### Alternative A: Foreground-Only on Windows

Simply remove the `daemonize` dependency and run in foreground mode on Windows.

**Pros:**
- Minimal code changes
- Works immediately

**Cons:**
- Requires terminal window to remain open
- No integration with Windows service infrastructure
- Poor user experience compared to Unix

**Rejected**: Does not meet the goal of proper Windows integration.

### Alternative B: NSSM Wrapper

Use NSSM (Non-Sucking Service Manager) as an external wrapper.

**Pros:**
- No Rust code changes
- Well-tested external tool

**Cons:**
- External dependency
- User must install NSSM separately
- Less integrated experience

**Rejected**: External dependency is undesirable for a standalone tool.

### Alternative C: Cross-Platform Service Library

Use a crate like `service-manager` that abstracts all platforms.

**Pros:**
- Single API for all platforms
- Could support macOS launchd too

**Cons:**
- Less mature than `windows-service`
- May not expose all Windows-specific features
- Additional abstraction layer

**Considered for future**: May revisit when adding launchd support.

### Do Nothing

Keep the daemon Unix-only.

**Impact**: Windows users continue to be excluded from daemon functionality. They must use CLI or MCP server for all integrations.

---

## Compatibility

### Backward Compatibility

- **Unix users**: No change in behavior; existing workflows continue to work
- **Windows users**: Gain new functionality; no breaking changes (feature was unavailable)
- **API endpoints**: No changes to REST API
- **Configuration**: `.acp/` directory structure unchanged

### Forward Compatibility

- **macOS launchd**: The abstraction layer enables future native macOS service support
- **systemd**: Could add `acpd service install --systemd` to generate unit files
- **Container support**: Abstraction doesn't preclude container orchestration

### Migration Path

No migration required. Windows support is additive:

1. Existing Unix deployments continue unchanged
2. Windows users gain `acpd start/stop/status` commands
3. Optional `acpd service install` for Windows Service integration

---

## Implementation

### Phase 1: Platform Abstraction

**Effort**: 1 week

1. Create `src/platform/mod.rs` with `DaemonManager` trait
2. Create `src/platform/unix.rs` with existing logic refactored
3. Update `src/lifecycle.rs` to use the abstraction
4. Ensure all tests pass on Unix

**Files:**
- New: `src/platform/mod.rs`
- New: `src/platform/unix.rs`
- Modified: `src/lifecycle.rs`
- Modified: `src/main.rs`

### Phase 2: Windows Implementation

**Effort**: 2 weeks

1. Create `src/platform/windows.rs` with Windows Service integration
2. Add Windows-specific CLI commands
3. Implement service installation/uninstallation
4. Add Windows event logging

**Files:**
- New: `src/platform/windows.rs`
- Modified: `src/main.rs` (CLI commands)
- Modified: `Cargo.toml` (dependencies)

### Phase 3: CI and Testing

**Effort**: 1 week

1. Add Windows to CI matrix
2. Create Windows-specific integration tests
3. Test service installation on Windows runners
4. Document Windows-specific requirements

**Files:**
- Modified: `.github/workflows/ci.yml`
- Modified: `.github/workflows/release.yml`
- New: `tests/windows_service.rs` (cfg-gated)

### Phase 4: Documentation and Release

**Effort**: 0.5 weeks

1. Update README with Windows instructions
2. Add troubleshooting guide for Windows Service issues
3. Update release workflow for Windows builds
4. Release v0.2.0

**Files:**
- Modified: `README.md`
- New: `docs/windows-service.md`
- Modified: `.github/workflows/release.yml`

**Total Effort**: ~4.5 weeks

---

## Rollout Plan

1. **v0.1.x**: Unix-only releases (current)
2. **v0.2.0-alpha**: Windows builds enabled, service support behind feature flag
3. **v0.2.0-beta**: Full Windows Service support, community testing
4. **v0.2.0**: Stable release with Windows support

---

## Open Questions

1. **Service name configurability**: Should the Windows Service name be configurable, or always "AcpDaemon"?
   - *Recommendation*: Use fixed name for simplicity; reconsider if multi-instance needed

2. **Automatic service installation**: Should `acpd start` auto-install the service on Windows?
   - *Recommendation*: Yes, with clear messaging about admin requirements

3. **Port persistence**: How should the configured port be persisted across service restarts?
   - *Recommendation*: Store in `.acp/daemon.port` file, same as Unix

4. **Event logging verbosity**: How much should be logged to Windows Event Log?
   - *Recommendation*: Start/stop events and errors; debug logs to file only

---

## Resolved Questions

1. **Q**: Should we support Windows arm64?
   **A**: Not in initial release. Focus on x64 which covers vast majority of Windows users.

2. **Q**: Should we use `daemonize` on Unix or keep current spawn approach?
   **A**: Keep current spawn approach. It works well and `daemonize` adds unnecessary complexity.

---

## References

- [windows-service crate](https://crates.io/crates/windows-service)
- [Windows Service Architecture](https://docs.microsoft.com/en-us/windows/win32/services/service-programs)
- [nix crate](https://crates.io/crates/nix)
- [acp-daemon repository](https://github.com/acp-protocol/acp-daemon)

---

## Appendix

### A. Windows Service State Diagram

```
                    ┌─────────────┐
                    │  Not        │
         install    │  Installed  │◄────── uninstall
            │       └──────┬──────┘            ▲
            ▼              │                   │
     ┌──────────────┐      │            ┌──────┴──────┐
     │   Stopped    │◄─────┘            │   Running   │
     │              │───────start──────►│             │
     │              │◄──────stop────────│             │
     └──────────────┘                   └─────────────┘
```

### B. Security Considerations

1. **Service account**: By default, runs as LocalSystem. Consider supporting configurable service accounts for production deployments.

2. **Port binding**: Binding to `127.0.0.1` ensures local-only access. If network access is needed, firewall rules must be configured separately.

3. **Admin privileges**: Service installation requires admin rights, but normal start/stop does not after installation.

### C. Comparison with Other Daemons

| Feature | acpd (proposed) | postgres | nginx (Windows) |
|---------|-----------------|----------|-----------------|
| Service install | `acpd service install` | pg_ctl register | Installer |
| Service name | AcpDaemon | postgresql-x64-15 | nginx |
| Config location | `.acp/` | `data/` | `conf/` |
| Default port | 9222 | 5432 | 80 |

---

## Changelog

| Date       | Change                        |
|------------|-------------------------------|
| 2025-12-29 | Initial draft                 |
