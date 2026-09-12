#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type WirePath = String;
#[rustfmt::skip]
pub type SocketMode = i64;
#[rustfmt::skip]
pub type SystemSocketPath = WirePath;
#[rustfmt::skip]
pub type SystemSocketMode = SocketMode;
#[rustfmt::skip]
pub type SupervisionSocketPath = WirePath;
#[rustfmt::skip]
pub type SupervisionSocketMode = SocketMode;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SystemBackend {
    Niri,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SystemDaemonConfiguration {
    pub system_socket_path: SystemSocketPath,
    pub system_socket_mode: SystemSocketMode,
    pub supervision_socket_path: SupervisionSocketPath,
    pub supervision_socket_mode: SupervisionSocketMode,
    pub system_backend: SystemBackend,
    pub owner_identity: signal_persona::OwnerIdentity,
}
#[rustfmt::skip]
pub type ConfigurationGeneration = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Configured {
    pub configuration_generation: ConfigurationGeneration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    MalformedConfiguration,
    UnsupportedBackend,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigurationRejected {
    pub configuration_rejection_reason: ConfigurationRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MetaSystemOperationKind {
    Configure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
    ComponentPaused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RequestUnimplemented {
    pub meta_system_operation_kind: MetaSystemOperationKind,
    pub unimplemented_reason: UnimplementedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Configure(SystemDaemonConfiguration),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Configured(Configured),
    ConfigurationRejected(ConfigurationRejected),
    RequestUnimplemented(RequestUnimplemented),
}
