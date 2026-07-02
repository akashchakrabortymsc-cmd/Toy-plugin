# Design decisions: plugin-example

## This fuses plugin-sdk and plugin-example into one binary

In the full design, plugin-sdk gives a plugin author a PluginHandler
trait and a PluginServer that handles RPC dispatch, and plugin-example
is a thin implementation on top of that SDK. Here, both are one binary
that hand-rolls its own dispatch in a match statement.

That is intentional. Building the SDK abstraction before proving the
underlying mechanism works means designing an API around a guess. The
right time to design plugin-sdk is after a second, genuinely different
plugin exists and reveals what the abstraction actually needs to cover
— not before there is even a first one working.

## Running a real shell command, not a stubbed result

The job is echo Hello from plugin!, executed for real. A stubbed
response would prove the JSON round-trips correctly but nothing about
whether the plugin can do real work and report a real result. Running
something real, however trivial, is what makes the proof-of-life
actually mean something.

## Job state is a plain HashMap, not thread-safe

The real plugin will need Arc<Mutex<HashMap<...>>> once it handles
concurrent requests over an async runtime. This toy processes one
request at a time on a single thread, so a plain HashMap is the correct
amount of structure for what it is actually doing. Adding
synchronization primitives here would defend against a failure mode
this toy cannot produce.

## cmd /C on Windows, sh -c on Unix

This cross-platform decision is noted here because it was discovered
by actually running the toy, not by designing for it in advance. The
original code used sh -c unconditionally, which silently fails on
Windows with "program not found." The fix — cfg!(target_os = "windows")
branching — is small, but the lesson is real: the toy caught a portability
assumption that would have been invisible in the design documents.