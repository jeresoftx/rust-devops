use rust_devops::regulated_operations::{
    AuthorizationKind, DataClassification, Environment, RegulatedOperationEvent,
    evaluate_regulated_operation,
};

fn main() {
    let event = RegulatedOperationEvent::new("evt-1", "release-bot", "deploy", "payments-api")
        .in_environment(Environment::Production)
        .with_data_classification(DataClassification::Sensitive)
        .authorized_by(AuthorizationKind::HumanApproval)
        .correlated_with("change-123")
        .evidenced_by("audit://change-123")
        .minimizing_sensitive_data()
        .retaining_evidence_for(365);

    let evaluation = evaluate_regulated_operation(&event);

    assert!(evaluation.auditable);
    assert!(evaluation.production_scope);
    assert!(evaluation.sensitive_scope);
}
