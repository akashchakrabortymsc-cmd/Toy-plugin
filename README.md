# sprocket-plugin-toy

Minimal proof-of-concept for a Crankshaft/Sprocket execution-backend
plugin system. The engine (`plugin-host`) spawns an external process
(`plugin-example`) and drives it over TCP using a line-delimited
JSON-RPC protocol.

Built as a proof-of-concept while independently developing a plugin
architecture for [Crankshaft](https://github.com/stjude-rust-labs/crankshaft),
in conversation with the maintainer.

## Why this shape

Three options for plugin isolation were considered:

| Approach | Problem |
|---|---|
| Dynamic libraries (`.so`/`.dll`) | Rust has no stable ABI — version drift causes silent crashes |
| WebAssembly | WASI networking can't comfortably reach async cloud APIs today |
| **Subprocess + JSON-RPC over TCP** | No ABI problem. Full native networking. Plugin crash ≠ engine crash. |

See [`DESIGN.md`](./DESIGN.md) for the full reasoning.

## Structure

```
sprocket-plugin-toy/
├── .gitignore
├── Cargo.lock
├── Cargo.toml              # workspace root
├── DESIGN.md               # why subprocess+TCP, not WASM or dylib
├── README.md
├── plugin-core/
│   ├── Cargo.toml
│   ├── DESIGN.md           # why this crate is smaller than the real one
│   └── src/
│       └── lib.rs          # JobId, JobStatus, Job, RpcRequest, RpcResponse
├── plugin-host/
│   ├── Cargo.toml
│   ├── DESIGN.md           # why std::net, why host-listens not plugin-listens
│   └── src/
│       └── main.rs         # binds port, spawns plugin, submit + status
└── plugin-example/
    ├── Cargo.toml
    ├── DESIGN.md           # why sdk+example are fused, why a real shell command
    └── src/
        └── main.rs         # connects back over TCP, runs commands, reports status
```

- `plugin-core` → Part 1 of the roadmap (shared types and contracts)
- `plugin-host` → stripped-down Part 2 (no tokio yet, no retry loop)
- `plugin-example` → Parts 3+4 fused (no SDK abstraction until a second
  plugin exists to reveal what the API actually needs)

## Build and run

```bash
cargo build

# Linux/macOS
./target/debug/plugin-host ./target/debug/plugin-example

# Windows
.\target\debug\plugin-host.exe .\target\debug\plugin-example.exe
```

## Proof of life

Terminal output showing Hello from plugin!

<img width="1212" height="92" alt="Screenshot 2026-07-02 174046" src="https://github.com/user-attachments/assets/db0e78da-739a-4908-a042-5266b62337a8" />


Expected output:

```
[host] listening on 127.0.0.1:54213
[host] plugin connected
[host] submit response: RpcResponse { result: Some(String("job-1")), error: None }
[host] status response: RpcResponse { result: Some(Object {"status": String("Completed"),
  "stdout": String("Hello from plugin!\n")}), error: None }
```

The `stdout` field carrying `Hello from plugin!` is the proof-of-life —
a separate OS process ran a real shell command and its output came back
over the wire to the host.

## What's deliberately absent

| Missing | 
|---|
| tokio / async | 
| Retry-on-connect, polling loop, `kill_on_drop` | 
| `PluginHandler` trait + `PluginServer` SDK | 
| `PluginError`, `PluginBackend` trait | 
| Crash detection, state persistence, health checks | 
| `BackendConfig::Plugin` in Crankshaft config |

This toy proves the protocol and process model work end-to-end, in
isolation, before touching anything in `crankshaft-config` or
`crankshaft-engine`.

## Current status

The toy is working. The real `crankshaft-plugin-core` crate already
exists separately with proper types (`Job`, `JobStatus`, `JobId`,
`Resources`), a `PluginHandler` trait using `#[async_trait]`, and a
`PluginError` enum — built on top of what this toy proved out. The next
phase migrates `plugin-host` onto tokio and formalises the host-side
`PluginBackend` trait for composition with Crankshaft's existing
Docker/Slurm backends.
