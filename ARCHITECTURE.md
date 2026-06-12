# meta-signal-system — architecture

*Meta policy contract for the `system` component.*

## Surface

`meta-signal-system` is the privileged companion contract to
`signal-system`. It carries the meta plane for `system`; ordinary focus
observations, status queries, and subscription events stay in
`signal-system`.

The current channel has one operation:

```text
MetaSystemRequest                         MetaSystemReply
└─ Configure(SystemDaemonConfiguration)   ├─ Configured
                                          ├─ ConfigurationRejected
                                          └─ RequestUnimplemented
```

`SystemDaemonConfiguration` is imported from `signal-system`. The startup
binary file and the meta reconfiguration operation use the same typed record;
configuration never arrives as flags.

## Boundaries

This crate owns:

- the meta request and reply vocabulary for `system`;
- typed configuration-generation and rejection records;
- NOTA and rkyv derives for the meta contract.

This crate does not own:

- the `system` daemon runtime;
- ordinary focus observation traffic;
- backend selection logic beyond the typed configuration payload;
- engine-management supervision protocol details.

## Invariants

- Every component has exactly two public contracts:
  `signal-<component>` and `meta-signal-<component>`.
- `Configure` carries `signal-system::SystemDaemonConfiguration`; no local
  mirror type is allowed.
- Runtime reconfiguration may be rejected by the daemon until `system` owns a
  hot-configuration reducer, but the rejection is typed.
- Future privileged OS actions extend this meta contract only after their
  authority boundary is concrete in `system`.

## Code Map

```text
src/lib.rs    payloads, signal_channel! declaration, and component aliases
```
