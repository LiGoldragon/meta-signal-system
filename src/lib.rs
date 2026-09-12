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

use std::marker::PhantomData;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

/// A portable rkyv Signal frame whose target contract is carried in its type.
pub struct Signal<T> {
    bytes: Vec<u8>,
    target: PhantomData<fn() -> T>,
}

/// Data that can form a portable Signal frame.
pub trait Signalizable: Sized {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error>;
}

/// A frame exposes its peer-wire bytes for transport framing.
pub trait ByteViewable {
    fn bytes(&self) -> &[u8];
}

/// A typed portable Signal can restore the contract value it carries.
pub trait Restorable<T> {
    fn restore(&self) -> Result<T, rkyv::rancor::Error>;
}

impl Signalizable for Query {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl Signalizable for Response {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl<T> From<Vec<u8>> for Signal<T> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            target: PhantomData,
        }
    }
}

impl<T> ByteViewable for Signal<T> {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Restorable<Query> for Signal<Query> {
    fn restore(&self) -> Result<Query, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}

impl Restorable<Response> for Signal<Response> {
    fn restore(&self) -> Result<Response, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}
