# Meta Signal System — Agent Instructions

## Purpose

`meta-signal-system` is the meta policy contract for the `system` component.
It carries privileged configuration and future authority-gated system actions.
Ordinary OS observation traffic stays in `signal-system`; runtime behavior
stays in `system`.

## Local Rules

- Keep this crate contract-only: no actors, sockets, redb, daemon loops, or
  backend adapter code.
- Import the daemon startup configuration from `signal-system`; do not define a
  second local mirror of `SystemDaemonConfiguration`.
- Keep meta operations authority-shaped and closed. Add new operations only
  when the `system` component has a concrete policy boundary for them.
- Keep NOTA text behind the crate's `nota-text` feature for CLI/tooling edges.
