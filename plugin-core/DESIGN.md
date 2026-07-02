# Design decisions: plugin-core

## Minimal on purpose

This toy's plugin-core has five types: JobId, JobStatus, Job,
RpcRequest, RpcResponse. No PluginError, no trait, no builder pattern.

The goal was isolating one question — does the wire protocol work at
all — from every other question. Adding error handling or a trait at
this stage would make it harder to tell which part of a failure was the
protocol and which was something else. The real crankshaft-plugin-core
already has all of those things; this toy deliberately does not.

## RpcRequest/RpcResponse instead of formal JSON-RPC 2.0

The real JSON-RPC 2.0 spec has fields like jsonrpc, id, and structured
error objects. This toy uses a stripped-down version because it only
ever has one request in flight at a time, so request IDs and full spec
compliance add complexity without adding proof. Moving to a real
JSON-RPC crate (or adding request IDs for concurrent requests) is the
natural next step if this graduates past the toy stage.

## JobStatus::Failed(String) carries a message; the others do not

Even a toy needs to distinguish "ran fine" from "ran but failed," and
the failure case needs some detail to be useful at all — even before a
real error taxonomy (ConnectionFailed, Timeout, etc.) exists.

## One wire format decision worth stating explicitly

Both sides agree that messages are newline-delimited JSON — one JSON
object per line, no framing beyond the newline. This is the simplest
possible wire protocol. The real implementation should document this
contract explicitly in a PROTOCOL.md, because changing it after a
second plugin author builds against it is a breaking change.