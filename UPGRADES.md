# Upgrades

## 4.1.0 → 4.1.1: generator ethos-zero 22.0.0

- Build-dependency ethos-zero ccacc21 (22.0.0), which brings its own datom-codec 5cd6219
  (0.33.1) into the lock as a build dependency only. ethos-zero 5a5daca (19.2.0) stays
  in the lock as the build dependency of signal and signal-persona.
- `ethos/signal.ethos` writes each variant named for the type it carries bare
  (`X.X` becomes `X`), which 22.0.0 requires. The generated Rust is byte-identical;
  the Rust API is unchanged.

## 4.1.0 — repin to the 0.33 cohort

- Pins: signal-persona 0f2b3e7 (5.1.0, candidate branch protos033-923d14-signal-persona,
  pending main), signal 7afb68b (10.3.0), datom-codec eabead4 (0.33.0), protos 3c7ade3
  (0.33.0), and build-dependency ethos-zero 5a5daca (19.2.0). One revision of each in the lock.
- Rust API only: `src/generated/signal.rs` is regenerated with ethos-zero 19.2.0, so the seven
  type aliases WirePath, SocketMode, SystemSocketPath, SystemSocketMode, SupervisionSocketPath,
  SupervisionSocketMode and ConfigurationGeneration are tuple structs. A consumer wraps a value
  with the constructor and reads it as `.0`. A test compares the archived bytes of a sample of
  them with those of the wrapped types; it is a sample, not a proof for every type.
- `ReaderBudget` fields are private: build one with `ReaderBudget::try_from(n).unwrap()`.
- The contract holds no `Decimal`.

## 4.0.0 — repin to the current producer front, adopt the exchange layer

Repinned to `protos` 0.31.0 (rev `1febca7836bf8d5f5973a302fdd50aa9c84c159b`),
`datom-codec` 0.31.0 (rev `09e2a9d52bf7f2f11e51d15cb6c3177f72c6c927`),
`ethos-zero` 10.0.0 (rev `4bf73cae8d4f5a2072c76a11cd2f00aa3fe9f8e3`,
build-dependency only), `signal` 7.0.0 (rev
`66e7b153706696c2cfbb2abcf931a5e83aec91af`), and `signal-persona` 5.0.0 (rev
`3462a0afc533f5dcfbee8d3802d17b594accc14f`). `src/generated/signal.rs` was
regenerated with the `ethos-zero` 10.0.0 CLI: every generated type gains
`Eq, Hash` in its derive list (`datom-codec` 0.31.0 gave the public data
types a `Hash`); no field, no kind and no trait name changed.

An initial pass repinned only `protos`/`datom-codec`/`ethos-zero` and left
`signal` on 5.0.0 (rev `7bcb0949`) and `signal-persona` on its own old
producer front, on the premise that the `signal` 7.x wave was a separate,
later pass. That premise was wrong: `signal` declares `links = "signal"`,
which admits exactly one package per `links` key in a graph, so a stale
`signal` pin is a hard resolution failure, not deferred debt. The first
attempt's gate confirmed this concretely — `nix flake check` failed at the
`meta-signal-system-deps` build trying to fetch `datom-codec` at the old rev
directly from the network, because the sandboxed build has none and crane
vendors git sources from `Cargo.lock` alone; the missing entry was for a
revision `signal` 5.0.0's own manifest still demanded. Moving `signal` (and,
transitively required, `signal-persona`, which `signal`'s `links` key
equally binds) to the new front removes that demand and un-splits the
graph. `signal-persona` was outside this crate's originally assigned
closure but blocks it exactly the way `signal` does, so it was repinned in
this same pass as well (see `signal-persona`'s own changelog).

`datom-codec` 0.31.0 removed every datom kind from `f64`; this contract holds
no `f64` and no `Decimal` anywhere in its ethos source, so nothing here
needed conversion.

`datom-codec`'s dependency list now includes `rkyv` (this crate's own
build-dependency on `ethos-zero` enables `datom-codec/rkyv`, unified into the
one resolved `datom-codec` package for this platform).

`signal` 7.0.0 adds the exchange layer (contract handshake by source
digest). This crate now carries the one-line
`impl signal::Contracted for Query { const CONTRACT_SOURCE: &'static str = ETHOS; }`
in `src/lib.rs` that the new handshake needs, per
`flows/f6db8d/reports/exchange-protocol.md` §4 and §9.

## 3.0.0 — un-fork the portable Signal frame

`Signal<T>`, `Signalizable`, `ByteViewable`, and `Restorable<T>` are no
longer defined in this crate. They are re-exported from the `signal`
crate (`5.0.0`), which is now the one shared portable-frame type across
the estate. A locally-defined `Signal<T>` would have been a distinct
Rust type from every other consumer's, forking the wire even where the
bytes agreed.

`ethos-zero` moved 8.0.1 → 9.0.0, which emits `datom_codec::Composing`
where it used to emit `datom_codec::Compositional`; the committed
`src/generated/signal.rs` was regenerated against the pinned library
accordingly. `datom-codec`'s `Compositional` derive/kind is now named
`Composing` (the kind was split: `Composing` is what
`Potential::<T>::actualize` needs).

Deploying this change:

- Every consumer of this crate must move to `3.0.0` in the same change
  that picks up this crate's new pin — there is no compatibility path.
- A consumer that imported `Signal`, `Signalizable`, `ByteViewable`, or
  `Restorable` needs no source change: the names are unchanged, only
  their defining crate is. A consumer that named the *type*
  `meta_signal_system::Signal<T>` in a signature alongside another
  crate's own forked `Signal<T>` will now see them unify — that is the
  point of the change, not a regression to route around.
- A consumer using `datom_codec::Compositional` as a trait bound or
  derive on types that flow through this contract must rename it to
  `datom_codec::Composing`.
- Depend on `signal` with the exact pin string
  `git = "https://github.com/LiGoldragon/signal"` (no `.git` suffix) —
  `signal` declares `links = "signal"`, so cargo treats a differently
  spelled source URL for the same commit as a different crate and
  resolution fails.
