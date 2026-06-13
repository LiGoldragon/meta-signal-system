use meta_signal_system::{
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    MetaSystemFrame, MetaSystemFrameBody, MetaSystemReply, Operation, OperationKind,
    RequestUnimplemented, SystemDaemonConfiguration, UnimplementedReason,
};
#[cfg(feature = "nota-text")]
use nota_next::{NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_persona::origin::{OwnerIdentity, UnixUserIdentifier};
use signal_persona::{SocketMode, WirePath};
use signal_system::SystemBackend;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MetaSystemFixture {
    exchange: ExchangeIdentifier,
}

impl MetaSystemFixture {
    fn new() -> Self {
        Self {
            exchange: ExchangeIdentifier::new(
                SessionEpoch::new(1),
                ExchangeLane::Connector,
                LaneSequence::first(),
            ),
        }
    }

    fn configuration(&self) -> SystemDaemonConfiguration {
        SystemDaemonConfiguration {
            system_socket_path: WirePath::new("/run/persona/system.sock"),
            system_socket_mode: SocketMode::new(0o600),
            supervision_socket_path: WirePath::new("/run/persona/system-supervision.sock"),
            supervision_socket_mode: SocketMode::new(0o600),
            backend: SystemBackend::Niri,
            owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
        }
    }

    fn round_trip_request(&self, request: Operation) -> Operation {
        let frame = MetaSystemFrame::new(MetaSystemFrameBody::Request {
            exchange: self.exchange.clone(),
            request: request.clone().into_request(),
        });
        let bytes = frame.encode_length_prefixed().expect("encode request");
        let decoded = MetaSystemFrame::decode_length_prefixed(&bytes).expect("decode request");
        match decoded.into_body() {
            MetaSystemFrameBody::Request { request, .. } => request.payloads().head().clone(),
            other => panic!("expected request frame, got {other:?}"),
        }
    }

    fn round_trip_reply(&self, reply: MetaSystemReply) -> MetaSystemReply {
        let frame = MetaSystemFrame::new(MetaSystemFrameBody::Reply {
            exchange: self.exchange.clone(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
        });
        let bytes = frame.encode_length_prefixed().expect("encode reply");
        let decoded = MetaSystemFrame::decode_length_prefixed(&bytes).expect("decode reply");
        match decoded.into_body() {
            MetaSystemFrameBody::Reply { reply, .. } => match reply {
                Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                    SubReply::Ok(payload) => payload,
                    other => panic!("expected accepted reply payload, got {other:?}"),
                },
                Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
            },
            other => panic!("expected reply frame, got {other:?}"),
        }
    }
}

#[test]
fn configure_request_carries_system_daemon_configuration() {
    let fixture = MetaSystemFixture::new();
    let request = Operation::Configure(fixture.configuration());

    assert_eq!(request.kind(), OperationKind::Configure);
    assert_eq!(fixture.round_trip_request(request.clone()), request);
}

#[test]
fn meta_system_request_heads_are_contract_local_operations() {
    assert_eq!(<Operation as SignalOperationHeads>::HEADS, &["Configure"]);
}

#[test]
fn reply_variants_round_trip() {
    let fixture = MetaSystemFixture::new();
    let replies = [
        MetaSystemReply::Configured(Configured {
            generation: ConfigurationGeneration::new(7),
        }),
        MetaSystemReply::ConfigurationRejected(ConfigurationRejected {
            reason: ConfigurationRejectionReason::ManagerAuthorityRequired,
        }),
        MetaSystemReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::Configure,
            reason: UnimplementedReason::ComponentPaused,
        }),
    ];

    for reply in replies {
        assert_eq!(fixture.round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn configuration_generation_projects_to_integer() {
    let generation = ConfigurationGeneration::new(11);
    assert_eq!(generation.value(), 11);
}

#[cfg(feature = "nota-text")]
#[test]
fn meta_system_operations_encode_as_contract_local_nota_heads() {
    let fixture = MetaSystemFixture::new();
    let request = Operation::Configure(fixture.configuration());
    let text = request.to_nota();

    assert!(text.starts_with("(Configure"));

    let decoded = NotaSource::new(&text)
        .parse::<Operation>()
        .expect("decode request nota");
    assert_eq!(decoded, request);
}
