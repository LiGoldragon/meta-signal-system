# skills — meta-signal-system

Work here when the change concerns the privileged `system` policy contract.

Before editing, read:

- the `ethos` and `datom` skills
- this repo's `ARCHITECTURE.md`
- `../signal-system/ARCHITECTURE.md`
- `../system/ARCHITECTURE.md`

Rules:

- Keep the crate contract-only. Do not add runtime code.
- The contract is `ethos/signal.ethos`. Generated Rust is committed and
  freshness-asserted by `build.rs`.
- Keep operation names contract-local and authority-shaped.
- Add round-trip witnesses when adding operations or payload variants.
