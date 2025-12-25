# ACP Daemon Service

**Document Type**: Reference + How-To  
**Status**: OUTLINE — Content to be added  
**Last Updated**: December 2025

---

## Overview

The ACP Daemon is a background service that provides:
- Real-time file watching and cache updates
- Multi-tool coordination
- Proxy-based AI tool integration
- Persistent state management
- Event streaming for IDE extensions

---

## Installation

> **TODO**: Expand with platform-specific instructions

### With CLI
```bash
acp daemon install
```

### Manual (systemd)
```bash
sudo cp acp-daemon.service /etc/systemd/system/
sudo systemctl enable acp-daemon
sudo systemctl start acp-daemon
```

### Manual (launchd)
```bash
cp com.acp-protocol.daemon.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/com.acp-protocol.daemon.plist
```

---

## Quick Start

> **TODO**: Add complete workflow example

### 1. Start the Daemon
```bash
acp daemon start
```

### 2. Verify Status
```bash
acp daemon status
```

### 3. Use AI Tools with ACP
```bash
acp start -- cursor .
# Cursor now has ACP context automatically
```

---

## Commands

### `acp daemon start`

Start the daemon service.

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--foreground` | Run in foreground | `false` |
| `--port <port>` | API port | `7431` |
| `--socket <path>` | Unix socket path | `/tmp/acp.sock` |

> **TODO**: Add examples

---

### `acp daemon stop`

Stop the daemon service.

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--force` | Force stop | `false` |

---

### `acp daemon status`

Check daemon status.

**Output**:
```
ACP Daemon Status
━━━━━━━━━━━━━━━━━

Status:     Running
PID:        12345
Uptime:     2h 34m
Projects:   3 registered
API:        http://localhost:7431
Socket:     /tmp/acp.sock

Watched Projects:
  /home/user/project-a  [fresh]
  /home/user/project-b  [indexing...]
  /home/user/project-c  [fresh]
```

---

### `acp daemon logs`

View daemon logs.

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--follow` | Follow log output | `false` |
| `--lines <n>` | Number of lines | `50` |
| `--level <level>` | Filter by level | `info` |

---

### `acp daemon restart`

Restart the daemon.

---

## Architecture

> **TODO**: Add architecture diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         ACP Daemon                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │ File Watcher│  │ Cache Mgr   │  │ Event Bus   │             │
│  │             │  │             │  │             │             │
│  │ inotify/    │─▶│ Incremental │─▶│ Notify      │             │
│  │ FSEvents    │  │ Updates     │  │ Subscribers │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
│         │                │                │                     │
│         ▼                ▼                ▼                     │
│  ┌─────────────────────────────────────────────────┐           │
│  │                  API Server                      │           │
│  │  REST: /api/...   WebSocket: /ws   Unix: /sock  │           │
│  └─────────────────────────────────────────────────┘           │
│                           │                                     │
└───────────────────────────┼─────────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
      ┌─────────┐    ┌─────────┐    ┌─────────┐
      │ VS Code │    │  Cursor │    │  Claude │
      │   Ext   │    │  Proxy  │    │   Code  │
      └─────────┘    └─────────┘    └─────────┘
```

---

## API Reference

### REST API

> **TODO**: Document all endpoints

#### `GET /api/status`

Daemon status.

#### `GET /api/projects`

List registered projects.

#### `POST /api/projects`

Register a new project.

#### `GET /api/projects/{id}/cache`

Get project cache.

#### `POST /api/projects/{id}/index`

Trigger re-indexing.

---

### WebSocket API

> **TODO**: Document event types

Connect to `/ws` for real-time events:

```javascript
const ws = new WebSocket('ws://localhost:7431/ws');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  switch (data.type) {
    case 'cache_updated':
      // Cache was updated
      break;
    case 'file_changed':
      // File was modified
      break;
    case 'constraint_violated':
      // AI attempted constraint violation
      break;
  }
};
```

---

### Unix Socket API

> **TODO**: Document socket protocol

Connect to `/tmp/acp.sock` for local IPC.

---

## Configuration

### Daemon Configuration

**File**: `~/.config/acp/daemon.json`

```json
{
  "port": 7431,
  "socket": "/tmp/acp.sock",
  "log_level": "info",
  "log_file": "~/.local/share/acp/daemon.log",
  "watch_debounce_ms": 100,
  "max_projects": 50,
  "auto_index": true
}
```

### Project Registration

Projects can be registered:

1. **Automatically**: When `acp init` is run
2. **Manually**: Via API or config file
3. **On-demand**: When `acp start` is used

---

## Proxy Mode

The daemon can proxy AI tool traffic to inject ACP context:

```bash
acp start -- cursor .
```

This:
1. Starts the daemon (if not running)
2. Registers the current project
3. Launches Cursor with proxy configuration
4. Injects ACP context into tool requests

> **TODO**: Document proxy architecture

---

## Multi-Tool Coordination

When multiple AI tools access the same project:

1. Daemon maintains single source of truth
2. Changes from any tool trigger cache update
3. All tools receive update notifications
4. Conflict resolution (if applicable)

---

## Sections to Add

- [ ] **Security**: Authentication, authorization
- [ ] **Performance**: Tuning for large codebases
- [ ] **Troubleshooting**: Common issues
- [ ] **Platform-Specific**: Windows, macOS, Linux details
- [ ] **Service Management**: systemd, launchd, Windows Service
- [ ] **Clustering**: Multi-machine setup (future)

---

*This document is an outline. [Contribute content →](https://github.com/acp-protocol/acp-daemon)*
