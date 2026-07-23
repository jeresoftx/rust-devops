use rust_devops::regulated_operations::{
    AuthorizationKind, DataClassification, Environment, RegulatedOperationEvent,
    RegulatedOperationFinding, evaluate_regulated_operation,
};

fn main() {
    let deploy = RegulatedOperationEvent::new("evt-1", "release-bot", "deploy", "payments-api")
        .in_environment(Environment::Production)
        .with_data_classification(DataClassification::Sensitive)
        .authorized_by(AuthorizationKind::HumanApproval)
        .correlated_with("change-123")
        .evidenced_by("audit://change-123")
        .minimizing_sensitive_data()
        .retaining_evidence_for(365);

    let deploy_evaluation = evaluate_regulated_operation(&deploy);
    assert!(deploy_evaluation.auditable);

    let emergency = RegulatedOperationEvent::new("evt-2", "oncall", "rotate_secret", "vault")
        .in_environment(Environment::Production)
        .with_data_classification(DataClassification::Sensitive)
        .authorized_by(AuthorizationKind::Emergency)
        .correlated_with("incident-77")
        .evidenced_by("audit://incident-77")
        .minimizing_sensitive_data()
        .retaining_evidence_for(365);

    let emergency_evaluation = evaluate_regulated_operation(&emergency);
    assert!(!emergency_evaluation.auditable);
    assert!(
        emergency_evaluation
            .findings
            .contains(&RegulatedOperationFinding::ExceptionWithoutPostReview)
    );
}
