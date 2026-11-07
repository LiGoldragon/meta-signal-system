use meta_signal_system::{
    ByteViewable, ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason,
    Configured, MetaSystemOperationKind, Query, RequestUnimplemented, Response, Restorable, Signal,
    Signalizable, SocketMode, SupervisionSocketMode, SupervisionSocketPath, SystemBackend,
    SystemDaemonConfiguration, SystemSocketMode, SystemSocketPath, UnimplementedReason, WirePath,
};
use signal_persona::{OwnerIdentity, UnixUserIdentifier};

fn configuration() -> SystemDaemonConfiguration {
    SystemDaemonConfiguration {
        system_socket_path: SystemSocketPath(WirePath("/run/persona/system.sock".into())),
        system_socket_mode: SystemSocketMode(SocketMode(0o600)),
        supervision_socket_path: SupervisionSocketPath(WirePath(
            "/run/persona/system-supervision.sock".into(),
        )),
        supervision_socket_mode: SupervisionSocketMode(SocketMode(0o600)),
        system_backend: SystemBackend::Niri,
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier(1000)),
    }
}

#[test]
fn query_and_response_round_trip_through_received_bytes() {
    let query = Query::Configure(configuration());
    let received =
        Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
    assert_eq!(received.restore().expect("query restores"), query);

    for response in [
        Response::Configured(Configured {
            configuration_generation: ConfigurationGeneration(7),
        }),
        Response::ConfigurationRejected(ConfigurationRejected {
            configuration_rejection_reason: ConfigurationRejectionReason::ManagerAuthorityRequired,
        }),
        Response::RequestUnimplemented(RequestUnimplemented {
            meta_system_operation_kind: MetaSystemOperationKind::Configure,
            unimplemented_reason: UnimplementedReason::ComponentPaused,
        }),
    ] {
        let received = Signal::<Response>::from(
            response
                .signalize()
                .expect("response archives")
                .bytes()
                .to_vec(),
        );
        assert_eq!(received.restore().expect("response restores"), response);
    }
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn query_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = Query::Configure(configuration());
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(text)
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget::try_from(1024).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("Datom restores");
    assert_eq!(restored, query);
}

#[cfg(feature = "datom")]
#[test]
fn every_canonical_datom_line_actualizes_into_a_contract_head() {
    use datom_codec::{Actualizing, Budget, Potential};
    use protos::ReaderBudget;

    let budget = || Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    };

    for line in include_str!("../examples/canonical.datom")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(';'))
    {
        let query = Potential::<Query>::from(line.to_string()).actualize(&mut budget());
        let response = Potential::<Response>::from(line.to_string()).actualize(&mut budget());
        assert!(
            query.is_ok() || response.is_ok(),
            "canonical line is neither a Query nor a Response: {line}\n  as Query: {query:?}\n  as Response: {response:?}"
        );
    }
}

#[test]
fn newtype_heads_archive_as_the_types_they_replace() {
    fn archived<T>(value: &T) -> Vec<u8>
    where
        T: for<'a> rkyv::Serialize<
                rkyv::api::high::HighSerializer<
                    rkyv::util::AlignedVec,
                    rkyv::ser::allocator::ArenaHandle<'a>,
                    rkyv::rancor::Error,
                >,
            >,
    {
        rkyv::to_bytes::<rkyv::rancor::Error>(value)
            .expect("archive")
            .to_vec()
    }
    let text = String::from("/run/persona/system.sock");
    assert_eq!(archived(&WirePath(text.clone())), archived(&text));
    assert_eq!(archived(&SocketMode(0o600)), archived(&0o600_i64));
    assert_eq!(
        archived(&SystemSocketPath(WirePath(text.clone()))),
        archived(&text)
    );
    assert_eq!(
        archived(&SupervisionSocketMode(SocketMode(0o600))),
        archived(&0o600_i64)
    );
    assert_eq!(archived(&ConfigurationGeneration(7)), archived(&7_i64));
}
