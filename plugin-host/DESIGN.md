# Design decisions: plugin-host

## std::net, not tokio — deliberately

The real crankshaft-plugin-host will run on tokio, matching how
crankshaft-engine itself works. This toy uses plain std::net instead.
That is not an oversight — it is the same isolation principle as
plugin-core's minimalism: this toy answers "does spawn + TCP + JSON
work," not "does it work efficiently under concurrent load." Adding an
async runtime here would mean debugging two new things at once (the
protocol, and async Rust) instead of one. Tokio is the correct next
step once this mechanism is proven, not before.

## The host binds the port; the plugin connects back

This could have gone the other way — the plugin listens, the host
connects to it. The host-listens design was chosen because the host
controls when the plugin starts (it spawns the child process), so it
can bind a port first and pass that port to the child as an argument
before the child even exists. If the plugin listened instead, the host
would need to wait for the plugin to choose a port and report it back
through some side channel (a file, stdout parsing) before connecting —
a problem the host-listens design does not have.

## One submit, one status check — not a real polling loop

The real host polls repeatedly until a job reaches a terminal state.
This toy does exactly one submit and one status check, because the
thing being tested is "can the host ask the plugin two different things
and get two correct answers back." Proving that once is enough to prove
the mechanism; a loop around it adds nothing new at this stage.

## Cross-platform shell command

The plugin runs the job's command via cmd /C on Windows and sh -c on
Unix. This is a real design decision that the real implementation also
needs: the host sends a raw command string and the plugin decides how
to execute it on whatever OS it runs on. This means the plugin, not the
host, owns the execution environment — which is exactly the right
separation for a system where plugins target different platforms
(Kubernetes, Slurm, AWS Batch) that may each require different execution
conventions.

## kill_on_drop not implemented here

This toy uses std::process::Command, which has no kill_on_drop
equivalent the way tokio's Command does. The real host needs this to
avoid orphaned plugin processes if the engine exits unexpectedly. It is
deferred here because it is a tokio-specific concern, consistent with
deferring tokio itself.