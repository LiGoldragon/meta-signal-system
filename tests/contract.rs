use meta_signal_system::{
    ByteViewable, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    MetaSystemOperationKind, Query, RequestUnimplemented, Response, Restorable, Signal,
    Signalizable, SystemBackend, SystemDaemonConfiguration, UnimplementedReason,
};
use signal_persona::OwnerIdentity;

fn configuration() -> SystemDaemonConfiguration {
    SystemDaemonConfiguration {
        system_socket_path: "/run/persona/system.sock".into(),
        system_socket_mode: 0o600,
        supervision_socket_path: "/run/persona/system-supervision.sock".into(),
        supervision_socket_mode: 0o600,
        system_backend: SystemBackend::Niri,
        owner_identity: OwnerIdentity::UnixUser(1000),
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
            configuration_generation: 7,
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
            reader: ReaderBudget { remaining: 1024 },
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
        reader: ReaderBudget { remaining: 4096 },
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
