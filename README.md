# meta-signal-system

Meta signal contract for privileged system daemon configuration.

The meta-only wire contract for `system` — the second leg of the two-contract
pair (`signal-system` ordinary + `meta-signal-system` meta). The meta plane's
baseline content is daemon configuration: a typed `Configure` operation
carrying `system`'s `*DaemonConfiguration` (the same record that is the daemon's
binary startup message), with `Configured` / `ConfigurationRejected` /
`RequestUnimplemented` replies.

Default builds carry `nota-text` for CLI/debug projection; the wire is
binary/rkyv. See `ARCHITECTURE.md`.
