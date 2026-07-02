# Design: why subprocess + JSON-RPC over TCP

This toy exists to answer one question before anything bigger gets
designed: how should a Crankshaft plugin actually run? Three options
were on the table.

## Option 1: dynamic libraries (.so / .dll)

This is what Nextflow does via PF4J — plugins are .jar files loaded
directly into the running JVM. The JVM can do this safely because Java
bytecode has a stable, portable binary representation.

Rust has no equivalent guarantee. A .so compiled by one rustc version
has no promised ABI compatibility with code compiled by a different
version, or even the same version with different flags. Loading a plugin
this way would mean either pinning every plugin author to an exact
toolchain version forever, or accepting silent hard-to-diagnose crashes
when versions drift. Ruled out on ABI stability grounds alone.

## Option 2: WebAssembly modules

WASM sandboxing is attractive for isolation. The blocker is async I/O:
plugins that need to talk to cloud APIs (S3, GCS, Kubernetes) over async
HTTP clients hit real gaps in WASI's networking story today. A plugin
model that can't reach a cloud API is not a viable general-purpose
execution backend. Ruled out.

## Option 3 (chosen): separate OS process, JSON over TCP

No ABI problem — the two sides never share compiled code, only bytes
over a socket. No WASI networking gap — the plugin is a normal native
binary with full access to whatever async HTTP client it wants.

This also gives a real reliability property for free: if a plugin
crashes, it is a separate process crashing, not a shared-memory fault.
The engine keeps running. That property gets built out properly in later
phases, but it is a direct consequence of this one decision made here.

## What this toy proves

That the mechanics actually work end to end: a host process can spawn a
child, the child connects back over TCP, JSON messages flow in both
directions, and real work (running a shell command) happens inside the
child with its result observed by the parent. Everything past this point
— real traits, error types, retries, reliability — is refinement of a
proven mechanism, not a new bet.