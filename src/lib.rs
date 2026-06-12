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

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_system::SystemDaemonConfiguration;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct ConfigurationGeneration(u64);

impl ConfigurationGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl NotaDecode for ConfigurationGeneration {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        Ok(Self(NotaBlock::new(block).parse_integer()?))
    }
}

impl NotaEncode for ConfigurationGeneration {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Configured {
    pub generation: ConfigurationGeneration,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    MalformedConfiguration,
    UnsupportedBackend,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ConfigurationRejected {
    pub reason: ConfigurationRejectionReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
    ComponentPaused,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel MetaSystem {
        operation Configure(SystemDaemonConfiguration),
    }
    reply MetaSystemReply {
        Configured(Configured),
        ConfigurationRejected(ConfigurationRejected),
        RequestUnimplemented(RequestUnimplemented),
    }
}

pub type MetaSystemRequest = Operation;
pub type MetaSystemFrame = Frame;
pub type MetaSystemFrameBody = FrameBody;
pub type MetaSystemRequestBuilder = RequestBuilder;
pub type ChannelRequest = Operation;
pub type ChannelReply = MetaSystemReply;

impl From<SystemDaemonConfiguration> for MetaSystemRequest {
    fn from(payload: SystemDaemonConfiguration) -> Self {
        Self::Configure(payload)
    }
}
