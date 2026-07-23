use std::time::Instant;

use rust_devops::regulated_operations::{
    AuthorizationKind, DataClassification, Environment, RegulatedOperationEvent,
    evaluate_regulated_operation,
};

fn auditable_deploy() -> RegulatedOperationEvent {
    RegulatedOperationEvent::new("evt-1", "release-bot", "deploy", "payments-api")
        .in_environment(Environment::Production)
        .with_data_classification(DataClassification::Sensitive)
        .authorized_by(AuthorizationKind::HumanApproval)
        .correlated_with("change-123")
        .evidenced_by("audit://change-123")
        .minimizing_sensitive_data()
        .retaining_evidence_for(365)
}

fn production_without_authorization() -> RegulatedOperationEvent {
    RegulatedOperationEvent::new("evt-2", "release-bot", "deploy", "payments-api")
        .in_environment(Environment::Production)
        .with_data_classification(DataClassification::Internal)
        .correlated_with("change-124")
        .evidenced_by("audit://change-124")
        .retaining_evidence_for(180)
}

fn emergency_without_review() -> RegulatedOperationEvent {
    RegulatedOperationEvent::new("evt-3", "oncall", "rotate_secret", "vault")
        .in_environment(Environment::Production)
        .with_data_classification(DataClassification::Sensitive)
        .authorized_by(AuthorizationKind::Emergency)
        .correlated_with("incident-77")
        .evidenced_by("audit://incident-77")
        .minimizing_sensitive_data()
        .retaining_evidence_for(365)
}

fn main() {
    let auditable = auditable_deploy();
    let unauthorized = production_without_authorization();
    let emergency = emergency_without_review();
    let iterations = 100_000;
    let started = Instant::now();
    let mut auditable_events = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for event in [&auditable, &unauthorized, &emergency] {
            let evaluation = evaluate_regulated_operation(event);
            if evaluation.auditable {
                auditable_events += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(auditable_events, iterations);
    assert_eq!(total_findings, iterations * 2);

    println!(
        "evaluated {iterations} regulated operation event triples in {:?}",
        started.elapsed()
    );
}
