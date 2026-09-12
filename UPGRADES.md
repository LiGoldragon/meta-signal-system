# Upgrades

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
