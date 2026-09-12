# Meta Signal System — Agent Instructions

## Purpose

`meta-signal-system` is the meta policy contract for the `system` component.
It carries privileged configuration and future authority-gated system actions.
Ordinary OS observation traffic stays in `signal-system`; runtime behavior
stays in `system`.

## Local Rules

- Keep this crate contract-only: no actors, sockets, redb, daemon loops, or
  backend adapter code.
- Change the contract in `ethos/signal.ethos`, then regenerate:
  `ethos-zero 'Generate.{ <abs>/ethos/signal.ethos <abs>/src/generated }'`.
  Commit the generated Rust. Never hand-edit `src/generated/`.
- `SystemDaemonConfiguration` and `SystemBackend` are declared here. Do not
  redeclare them elsewhere; import them.
- Pin every LiGoldragon dependency by immutable `rev`. Never by branch.
- Keep meta operations authority-shaped and closed. Add new operations only
  when the `system` component has a concrete policy boundary for them.
- Add a round-trip witness in `tests/contract.rs` and a line in
  `examples/canonical.datom` for every new head.

## Protos estate status

Stack: correct-new destination
Status: active component contract, on the Datom stack
