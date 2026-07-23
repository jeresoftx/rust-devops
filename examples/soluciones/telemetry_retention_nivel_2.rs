use rust_devops::telemetry_retention::{
    DataSensitivity, RetentionPurpose, RetentionTier, TelemetryRetentionFinding,
    TelemetryRetentionPolicy, TelemetrySignalKind, evaluate_retention,
};

fn main() {
    let risky_policy = TelemetryRetentionPolicy::new("checkout_logs", TelemetrySignalKind::Log)
        .with_sensitivity(DataSensitivity::Sensitive)
        .for_purpose(RetentionPurpose::IncidentInvestigation)
        .owned_by("payments-oncall")
        .retained_as(RetentionTier::new(7, 23, 0))
        .ingesting_gib_per_day(8.0)
        .reviewed_every(30);

    let risky_evaluation = evaluate_retention(&risky_policy);
    assert!(!risky_evaluation.ready);
    assert!(
        risky_evaluation
            .findings
            .contains(&TelemetryRetentionFinding::SensitiveDataWithoutRedaction)
    );

    let corrected_policy = risky_policy.redacting_sensitive_fields();
    let corrected_evaluation = evaluate_retention(&corrected_policy);
    assert!(corrected_evaluation.ready);
}
