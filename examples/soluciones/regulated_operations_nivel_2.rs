use rust_devops::regulated_operations::{
    AuthorizationKind, DataClassification, Environment, RegulatedOperationEvent,
    RegulatedOperationFinding, evaluate_regulated_operation,
};

fn main() {
    let unsafe_change =
        RegulatedOperationEvent::new("evt-2", "release-bot", "deploy", "payments-api")
            .in_environment(Environment::Production)
            .with_data_classification(DataClassification::Internal)
            .correlated_with("change-124")
            .evidenced_by("audit://change-124")
            .retaining_evidence_for(180);

    let unsafe_evaluation = evaluate_regulated_operation(&unsafe_change);
    assert!(!unsafe_evaluation.auditable);
    assert!(
        unsafe_evaluation
            .findings
            .contains(&RegulatedOperationFinding::ProductionWithoutAuthorization)
    );

    let safe_change = unsafe_change.authorized_by(AuthorizationKind::ApprovedAutomation);
    let safe_evaluation = evaluate_regulated_operation(&safe_change);
    assert!(safe_evaluation.auditable);
}
