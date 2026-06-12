# skills — meta-signal-system

Work here when the change concerns the privileged `system` policy contract.

Before editing, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `INTENT.md`
- this repo's `ARCHITECTURE.md`
- `../signal-system/ARCHITECTURE.md`
- `../system/ARCHITECTURE.md`

Rules:

- Keep the crate contract-only. Do not add runtime code.
- Use `SystemDaemonConfiguration` from `signal-system`.
- Keep operation names contract-local and authority-shaped.
- Add round-trip witnesses when adding operations or payload variants.
