# meta-signal-system

Meta signal contract for privileged system daemon configuration.

The meta-only wire contract for `system` — the second leg of the two-contract
pair (`signal-system` ordinary + `meta-signal-system` meta). The meta plane's
baseline content is daemon configuration: a typed `Configure` operation
carrying `SystemDaemonConfiguration` (the same record that is the daemon's
binary startup message), with `Configured` / `ConfigurationRejected` /
`RequestUnimplemented` replies.

The contract is authored in `ethos/signal.ethos`; `ethos-zero` generates
`src/generated/signal.rs`, which is committed, and `build.rs` asserts the
committed generation matches a fresh one. The wire is binary rkyv; the
optional `datom` feature adds Datom text projection. See `ARCHITECTURE.md`.
