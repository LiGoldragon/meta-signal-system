# INTENT — meta-signal-system

*The meta-only wire contract for privileged `system` daemon configuration.
Companion to `Cargo.toml` and the ordinary `signal-system` contract.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is for the `meta-signal-system`
contract. Workspace-shape intent stays in `primary/INTENT.md`; the component
daemon intent stays in `system/INTENT.md`; ordinary system focus observation
traffic stays in `signal-system/INTENT.md`.

## Why this repo exists

Every Persona component has exactly two contracts: `signal-<component>`
(ordinary) and `meta-signal-<component>` (meta). `meta-signal-system` is the
second leg for `system` — the authority surface that configures the
`system-daemon`, including backend selection and the privileged-action surface
that `system/INTENT.md` keeps separate from read-only observation. Before this
repo, `system` had only its ordinary contract; this completes the pair.

## The channel shape

The meta plane's baseline content is daemon configuration. The channel carries
a single `Configure` operation whose payload is the typed
`SystemDaemonConfiguration` imported from `signal-system` — the same record
that is the daemon's binary startup message. Reconfiguration arrives over this
meta plane as the same typed record, never as flags.

- **Request:** `Configure(SystemDaemonConfiguration)`.
- **Replies:** `Configured`, `ConfigurationRejected` (typed reason),
  `RequestUnimplemented` (includes a `ComponentPaused` reason — `system` is
  paused until a real focus consumer lands).

Privileged OS actions (force-focus, focus-drift suppression) that
`system/INTENT.md` names as authority-gated are additional operations that
extend this channel when the focus path activates; daemon configuration is the
foundation they build on.
