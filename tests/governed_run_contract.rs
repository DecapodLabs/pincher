use pincher::governed_run::*;
use pincher::{ProofVerification, StateCommitmentManager};
use std::sync::{Arc, Mutex};

fn id<T: Ref>(value: &str) -> T {
    T::make(value)
}

trait Ref: Sized {
    fn make(value: &str) -> Self;
}

macro_rules! refs {
    ($($ty:ty),+ $(,)?) => {
        $(impl Ref for $ty {
            fn make(value: &str) -> Self {
                <$ty>::new(value).expect("fixture reference")
            }
        })+
    };
}

refs!(
    RunId,
    IntentId,
    SessionRef,
    TaskRef,
    WorkUnitRef,
    RepositoryRef,
    WorkspaceRef,
    CustodyReceiptRef,
    ContextEvidenceRef,
    ApprovalInterlockRef,
    ApprovalEvidenceRef,
    ValidationEvidenceRef,
    ProofEvidenceRef,
    ProviderProposalRef,
    CorrelationId,
    IdempotencyKey,
    EventId,
);

fn request(custody: CustodyBinding) -> RunRequest {
    RunRequest::v1(
        id("run-1"),
        id("intent-1"),
        id("correlation-1"),
        id("idempotency-1"),
        custody,
    )
}

fn custody() -> CustodyBinding {
    CustodyBinding::complete(
        id("session-1"),
        id("task-1"),
        id("work-unit-1"),
        id("repository-1"),
        id("workspace-1"),
    )
}

#[derive(Clone)]
struct RecordingSink {
    events: Arc<Mutex<Vec<RunEvent>>>,
}

impl RecordingSink {
    fn new() -> (Self, Arc<Mutex<Vec<RunEvent>>>) {
        let events = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                events: Arc::clone(&events),
            },
            events,
        )
    }
}

impl EventSink for RecordingSink {
    fn publish(&mut self, event: RunEvent) -> Result<(), EventSinkError> {
        self.events.lock().unwrap().push(event);
        Ok(())
    }
}

#[derive(Clone)]
struct FakeControl {
    calls: Arc<Mutex<Vec<&'static str>>>,
    interlocks: InterlockDecision,
    approval: ApprovalStatus,
    context_resolved: bool,
    workspace_allowed: bool,
    validation: ValidationEvidence,
    proof: ProofEvidence,
}

impl FakeControl {
    fn new() -> (Self, Arc<Mutex<Vec<&'static str>>>) {
        let calls = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                calls: Arc::clone(&calls),
                interlocks: InterlockDecision::Allow { advisory: None },
                approval: ApprovalStatus::NotRequired,
                context_resolved: true,
                workspace_allowed: true,
                validation: ValidationEvidence {
                    reference: id("validation-1"),
                    passed: true,
                },
                proof: ProofEvidence {
                    reference: id("proof-1"),
                    backed: true,
                },
            },
            calls,
        )
    }

    fn record(&self, call: &'static str) {
        self.calls.lock().unwrap().push(call);
    }
}

impl DecapodControlPlane for FakeControl {
    fn validate_custody(
        &self,
        binding: &CustodyBinding,
    ) -> Result<CustodyEvidence, DecapodPortError> {
        self.record("custody");
        Ok(CustodyEvidence {
            session: binding.session.clone().expect("complete fixture"),
            task: binding.task.clone().expect("complete fixture"),
            work_unit: binding.work_unit.clone().expect("complete fixture"),
            repository: binding.repository.clone().expect("complete fixture"),
            workspace: binding.workspace.clone().expect("complete fixture"),
            receipt: id("custody-1"),
            workspace_allowed: self.workspace_allowed,
        })
    }

    fn resolve_context(
        &self,
        _custody: &CustodyEvidence,
        _intent: &IntentId,
    ) -> Result<ContextEvidence, DecapodPortError> {
        self.record("context");
        Ok(ContextEvidence {
            reference: id("context-1"),
            resolved: self.context_resolved,
        })
    }

    fn evaluate_interlocks(
        &self,
        _custody: &CustodyEvidence,
        _context: &ContextEvidence,
    ) -> Result<InterlockDecision, DecapodPortError> {
        self.record("interlocks");
        Ok(self.interlocks.clone())
    }

    fn approval_status(
        &self,
        _custody: &CustodyEvidence,
        _context: &ContextEvidence,
    ) -> Result<ApprovalStatus, DecapodPortError> {
        self.record("approval");
        Ok(self.approval.clone())
    }

    fn validate(
        &self,
        _custody: &CustodyEvidence,
        _context: &ContextEvidence,
        _proposal: &ProviderProposal,
    ) -> Result<ValidationEvidence, DecapodPortError> {
        self.record("validation");
        Ok(self.validation.clone())
    }

    fn obtain_proof(
        &self,
        _custody: &CustodyEvidence,
        _validation: &ValidationEvidence,
    ) -> Result<ProofEvidence, DecapodPortError> {
        self.record("proof");
        Ok(self.proof.clone())
    }
}

#[derive(Clone)]
struct FakeProvider {
    calls: Arc<Mutex<Vec<GovernedInferenceRequest>>>,
}

impl FakeProvider {
    fn new() -> (Self, Arc<Mutex<Vec<GovernedInferenceRequest>>>) {
        let calls = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                calls: Arc::clone(&calls),
            },
            calls,
        )
    }
}

impl ProviderTurn for FakeProvider {
    fn infer(&self, request: GovernedInferenceRequest) -> Result<ProviderProposal, ProviderError> {
        self.calls.lock().unwrap().push(request);
        Ok(ProviderProposal {
            reference: id("proposal-1"),
            output_digest: "provider-output-digest".to_string(),
        })
    }
}

fn engine(
    control: FakeControl,
    provider: FakeProvider,
    sink: RecordingSink,
) -> GovernedRunEngine<FakeControl, FakeProvider, RecordingSink> {
    GovernedRunEngine::new(control, provider, sink)
}

#[test]
fn provider_is_called_only_after_custody_and_context() {
    let (control, control_calls) = FakeControl::new();
    let (provider, provider_calls) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);

    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(outcome, RunOutcome::Ready(_)));
    assert_eq!(
        *control_calls.lock().unwrap(),
        vec![
            "custody",
            "context",
            "interlocks",
            "approval",
            "validation",
            "proof"
        ]
    );
    assert_eq!(provider_calls.lock().unwrap().len(), 1);
}

#[test]
fn every_missing_custody_reference_stops_before_inference() {
    let mut bindings = Vec::new();
    let complete = custody();
    for field in [
        CustodyField::Session,
        CustodyField::Task,
        CustodyField::WorkUnit,
        CustodyField::Repository,
        CustodyField::Workspace,
    ] {
        let mut binding = complete.clone();
        match field {
            CustodyField::Session => binding.session = None,
            CustodyField::Task => binding.task = None,
            CustodyField::WorkUnit => binding.work_unit = None,
            CustodyField::Repository => binding.repository = None,
            CustodyField::Workspace => binding.workspace = None,
        }
        bindings.push(binding);
    }

    for binding in bindings {
        let (control, control_calls) = FakeControl::new();
        let (provider, provider_calls) = FakeProvider::new();
        let (sink, _) = RecordingSink::new();
        let mut engine = engine(control, provider, sink);
        let outcome = engine.run(request(binding)).unwrap();
        assert!(matches!(outcome, RunOutcome::Failed(_)));
        assert!(control_calls.lock().unwrap().is_empty());
        assert!(provider_calls.lock().unwrap().is_empty());
    }
}

#[test]
fn blocking_interlock_prevents_provider_invocation() {
    let (mut control, control_calls) = FakeControl::new();
    control.interlocks = InterlockDecision::Block {
        reference: id("approval-interlock-1"),
        remediation: Remediation::new("obtain human approval"),
    };
    let (provider, provider_calls) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);

    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(outcome, RunOutcome::Blocked(_)));
    assert!(provider_calls.lock().unwrap().is_empty());
    assert_eq!(
        &control_calls.lock().unwrap()[..],
        &["custody", "context", "interlocks"]
    );
    assert!(matches!(
        outcome.snapshot().blocked,
        Some(BlockedReason::Interlock { .. })
    ));
}

#[test]
fn advisory_does_not_become_a_blocker() {
    let (mut control, _) = FakeControl::new();
    control.interlocks = InterlockDecision::Allow {
        advisory: Some(AdvisoryEvidence {
            reference: Some(id("advisory-1")),
        }),
    };
    let (provider, _) = FakeProvider::new();
    let (sink, events) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    assert!(matches!(
        engine.run(request(custody())).unwrap(),
        RunOutcome::Ready(_)
    ));
    assert!(
        events
            .lock()
            .unwrap()
            .iter()
            .any(|event| event.advisory_ref.is_some())
    );
}

#[test]
fn provider_output_cannot_create_authority_or_readiness() {
    let (mut control, _) = FakeControl::new();
    control.proof.backed = false;
    let (provider, provider_calls) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert_eq!(provider_calls.lock().unwrap().len(), 1);
    assert!(matches!(outcome, RunOutcome::Failed(_)));
    assert!(outcome.snapshot().approval.is_none());
    assert!(
        outcome
            .snapshot()
            .proof
            .as_ref()
            .is_some_and(|proof| !proof.backed)
    );
}

#[test]
fn readiness_requires_successful_validation_evidence() {
    let (mut control, _) = FakeControl::new();
    control.validation.passed = false;
    let (provider, _) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(outcome, RunOutcome::Failed(_)));
    assert!(matches!(
        outcome.snapshot().failure,
        Some(RunFailure::Validation {
            reason: ValidationFailure::DecapodRejected,
            evidence: Some(_),
            ..
        })
    ));
}

#[test]
fn readiness_requires_decopod_proof_evidence() {
    let (mut control, _) = FakeControl::new();
    control.proof.backed = false;
    let (provider, _) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(outcome, RunOutcome::Failed(_)));
    assert!(matches!(
        outcome.snapshot().failure,
        Some(RunFailure::Proof {
            reason: ProofFailure::DecapodRejected,
            evidence: Some(_),
            ..
        })
    ));
}

#[test]
fn context_failure_is_typed_and_provider_free() {
    let (mut control, control_calls) = FakeControl::new();
    control.context_resolved = false;
    let (provider, provider_calls) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(outcome, RunOutcome::Failed(_)));
    assert!(matches!(
        outcome.snapshot().failure,
        Some(RunFailure::Context { .. })
    ));
    assert_eq!(&control_calls.lock().unwrap()[..], &["custody", "context"]);
    assert!(provider_calls.lock().unwrap().is_empty());
}

#[test]
fn workspace_custody_is_authoritative() {
    let (mut control, _) = FakeControl::new();
    control.workspace_allowed = false;
    let (provider, provider_calls) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(
        outcome.snapshot().failure,
        Some(RunFailure::Custody {
            reason: CustodyFailure::WorkspaceNotAllowed { .. },
            ..
        })
    ));
    assert!(provider_calls.lock().unwrap().is_empty());
}

#[test]
fn illegal_terminal_transition_is_rejected() {
    let (control, _) = FakeControl::new();
    let (provider, _) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let ready = engine.run(request(custody())).unwrap();
    let handed_off = engine.handoff(ready).unwrap();
    let error = engine.handoff(handed_off).unwrap_err();
    assert!(matches!(error, RunError::IllegalTransition { .. }));
}

#[test]
fn event_sequences_and_state_order_are_stable() {
    let (control, _) = FakeControl::new();
    let (provider, _) = FakeProvider::new();
    let (sink, events) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    engine.run(request(custody())).unwrap();
    let events = events.lock().unwrap();
    let sequences: Vec<u64> = events.iter().map(|event| event.sequence).collect();
    let states: Vec<RunState> = events.iter().filter_map(|event| event.state).collect();
    assert_eq!(sequences, (1..=events.len() as u64).collect::<Vec<_>>());
    assert_eq!(
        states,
        vec![
            RunState::Prepared,
            RunState::ContextResolved,
            RunState::Executing,
            RunState::Executing,
            RunState::Verifying,
            RunState::Ready,
        ]
    );
}

#[test]
fn serialization_round_trip_preserves_custody_and_evidence() {
    let (control, _) = FakeControl::new();
    let (provider, _) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    let encoded = serde_json::to_string(&outcome).unwrap();
    let decoded: RunOutcome = serde_json::from_str(&encoded).unwrap();
    assert_eq!(
        decoded.snapshot().request.custody,
        outcome.snapshot().request.custody
    );
    assert_eq!(decoded.snapshot().proof, outcome.snapshot().proof);
}

#[test]
fn unknown_event_kinds_are_preserved_without_success_semantics() {
    let event = RunEvent {
        contract: ContractIdentity::v1(),
        event_id: id("event-1"),
        run_id: id("run-1"),
        correlation_id: id("correlation-1"),
        sequence: 99,
        occurred_at: chrono::Utc::now(),
        source: "future-host".to_string(),
        kind: EventKind::new("run.future.unknown"),
        state: None,
        custody: None,
        advisory_ref: None,
        approval_ref: None,
        approval_evidence_ref: None,
        validation_ref: None,
        proof_ref: None,
        failure: None,
        payload: serde_json::json!({ "raw": { "future": true } }),
    };
    let decoded: RunEvent = serde_json::from_str(&serde_json::to_string(&event).unwrap()).unwrap();
    assert_eq!(decoded.kind.as_str(), "run.future.unknown");
    assert!(decoded.state.is_none());
    assert!(decoded.proof_ref.is_none());
}

#[test]
fn host_values_never_serialize_secret_fields() {
    let request_json = serde_json::to_string(&request(custody())).unwrap();
    assert!(!request_json.contains("password"));
    assert!(!request_json.contains("api_key"));
    assert!(!request_json.contains("bearer"));

    let event = RunEvent {
        contract: ContractIdentity::v1(),
        event_id: id("event-1"),
        run_id: id("run-1"),
        correlation_id: id("correlation-1"),
        sequence: 1,
        occurred_at: chrono::Utc::now(),
        source: "pincher".to_string(),
        kind: EventKind::new("run.state.prepared"),
        state: Some(RunState::Prepared),
        custody: None,
        advisory_ref: None,
        approval_ref: None,
        approval_evidence_ref: None,
        validation_ref: None,
        proof_ref: None,
        failure: None,
        payload: serde_json::Value::Null,
    };
    let event_json = serde_json::to_string(&event).unwrap();
    assert!(!event_json.contains("password"));
    assert!(!event_json.contains("api_key"));
}

#[test]
fn local_commitment_can_only_mark_local_integrity() {
    let manager = StateCommitmentManager::new("agent-1");
    let proof = manager.create_proof_surface(
        "commitment-1",
        pincher::decapod::commitment::ProofType::Test,
        "cargo test",
        Default::default(),
    );
    let mut proof = proof;
    assert_eq!(proof.verification, ProofVerification::Proposed);
    manager.mark_locally_integrity_checked(&mut proof);
    assert_eq!(
        proof.verification,
        ProofVerification::LocallyIntegrityChecked
    );
    let encoded = serde_json::to_string(&proof).unwrap();
    assert!(!encoded.contains("verified"));
}

#[test]
fn blocked_outcome_is_host_renderable_and_retains_interlock() {
    let (mut control, _) = FakeControl::new();
    control.interlocks = InterlockDecision::Block {
        reference: id("interlock-1"),
        remediation: Remediation::new("request approval"),
    };
    let (provider, _) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert_eq!(outcome.snapshot().state, RunState::Blocked);
    assert!(outcome.snapshot().blocked.is_some());
    assert!(outcome.snapshot().transitions.iter().any(|transition| {
        transition.from == RunState::AwaitingApproval && transition.to == RunState::Blocked
    }));
}

#[test]
fn failed_outcome_retains_typed_provider_cause() {
    struct FailingProvider;
    impl ProviderTurn for FailingProvider {
        fn infer(
            &self,
            _request: GovernedInferenceRequest,
        ) -> Result<ProviderProposal, ProviderError> {
            Err(ProviderError::Unavailable {
                reason: "fixture timeout".to_string(),
            })
        }
    }

    let (control, _) = FakeControl::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = GovernedRunEngine::new(control, FailingProvider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(
        outcome.snapshot().failure,
        Some(RunFailure::Provider {
            reason: ProviderError::Unavailable { .. },
            ..
        })
    ));
}

#[test]
fn unsupported_real_adapter_fails_closed() {
    let (provider, provider_calls) = FakeProvider::new();
    let (sink, _) = RecordingSink::new();
    let mut engine = GovernedRunEngine::new(UnsupportedDecapodControlPlane, provider, sink);
    let outcome = engine.run(request(custody())).unwrap();
    assert!(matches!(outcome, RunOutcome::Failed(_)));
    assert!(provider_calls.lock().unwrap().is_empty());
}

#[test]
fn ready_outcome_can_be_handed_off_without_erasing_evidence() {
    let (control, _) = FakeControl::new();
    let (provider, _) = FakeProvider::new();
    let (sink, events) = RecordingSink::new();
    let mut engine = engine(control, provider, sink);
    let ready = engine.run(request(custody())).unwrap();
    let handed_off = engine.handoff(ready).unwrap();
    assert!(matches!(
        handed_off,
        RunOutcome::HandedOff {
            terminal_state: RunState::Ready,
            ..
        }
    ));
    assert!(handed_off.snapshot().proof.is_some());
    assert_eq!(
        events.lock().unwrap().last().unwrap().state,
        Some(RunState::HandedOff)
    );
}
