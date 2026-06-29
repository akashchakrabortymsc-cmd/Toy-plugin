# sprocket-plugin-toy

Minimal proof-of-concept for a Crankshaft/Sprocket execution-backend plugin system.
The engine (`plugin-host`) spawns an external process (`plugin-example`) and drives it
over TCP using a line-delimited JSON-RPC protocol.

Built in response to conversation Clay McLeod 
---

## Why this shape

Three options for plugin isolation:

| Approach | Problem |
|---|---|
| Dynamic libraries (`.so`/`.dll`) | Rust has no stable ABI — version drift causes silent crashes |
| WebAssembly | WASI networking can't comfortably reach async cloud APIs today |
| **Subprocess + JSON-RPC over TCP** | No ABI problem. Full native networking. Plugin crash ≠ engine crash. |

See [`DESIGN.md`](DESIGN.md) for the full reasoning.

---

## Structure

```
sprocket-plugin-toy/
├── Cargo.toml              # workspace root
├── .gitignore
├── DESIGN.md               # why subprocess+TCP, not WASM or dylib
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

`plugin-core` → **Part 1** of the roadmap (types and contracts).  
`plugin-host` → stripped-down **Part 2** (no tokio yet, no retry loop).  
`plugin-example` → **Parts 3+4** fused (no SDK abstraction until a second plugin exists to reveal what the API needs).

---

## Build and run

```bash
cargo build
./target/debug/plugin-host ./target/debug/plugin-example
```

Expected output:

```
[host] listening on 127.0.0.1:54213
[host] plugin connected
[host] submit response: RpcResponse { result: Some(String("job-1")), error: None }
[host] status response: RpcResponse { result: Some(Object {"status": String("Completed"),
  "stdout": String("Hello from plugin!\n")}), error: None }
```

The `stdout` field is the proof-of-life from Part 2.5 of the roadmap.

---

## What's deliberately absent

| Missing | Deferred to |
|---|---|
| `tokio` / async | Part 2 (real `plugin-host`) |
| Retry-on-connect, polling loop, `kill_on_drop` | Part 2.6–2.8 |
| `PluginHandler` trait + `PluginServer` SDK | Part 3 |
| `PluginError`, `PluginBackend` trait | Part 1 (real version) |
| Crash detection, state persistence, health checks | Part 6 |
| `BackendConfig::Plugin` in Crankshaft config | Part 5 |

This toy proves the protocol and process model work end-to-end, in isolation, before
touching anything in `crankshaft-config` or `crankshaft-engine`.

---

## Next step

Layer in real `plugin-core` types (`PluginBackend` trait, `PluginError`), migrate
`plugin-host` onto tokio, and begin Parts 1–2 for real — now informed by a toy that
has already proven the approach works.
