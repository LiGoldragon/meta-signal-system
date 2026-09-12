//! Meta signal contract — privileged `system` daemon configuration.
//!
//! Ordinary system focus observation traffic lives in `signal-system`. This
//! crate carries the meta plane: the authenticated `Configure` operation that
//! applies `system`'s typed daemon configuration (backend selection and the
//! privileged-action authority surface the daemon binds).
//!
//! The basic meta operation of every component is daemon configuration — the
//! `SystemDaemonConfiguration` the Persona manager encodes is itself the binary
//! startup message, and later reconfiguration arrives over this meta plane as
//! the same typed record, never as flags.

pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

/// The portable rkyv Signal frame is one shared type across the estate.
/// A second copy here would be a different Rust type, forking the wire.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
