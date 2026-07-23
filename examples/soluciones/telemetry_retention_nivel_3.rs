use rust_devops::telemetry_retention::{
    DataSensitivity, RetentionPurpose, RetentionTier, TelemetryRetentionFinding,
    TelemetryRetentionPolicy, TelemetrySignalKind, evaluate_retention,
};

fn main() {
    let incomplete_policy = TelemetryRetentionPolicy::new("audit_logs", TelemetrySignalKind::Log)
        .with_sensitivity(DataSensitivity::Regulated)
        .for_purpose(RetentionPurpose::Compliance)
        .owned_by("security")
        .retained_as(RetentionTier::new(14, 76, 0))
        .ingesting_gib_per_day(4.0)
        .redacting_sensitive_fields()
        .reviewed_every(180);

    let incomplete_evaluation = evaluate_retention(&incomplete_policy);
    assert!(!incomplete_evaluation.ready);
    assert!(
        incomplete_evaluation
            .findings
            .contains(&TelemetryRetentionFinding::RegulatedDataWithoutColdArchive)
    );

    let archived_policy = TelemetryRetentionPolicy::new("audit_logs", TelemetrySignalKind::Log)
        .with_sensitivity(DataSensitivity::Regulated)
        .for_purpose(RetentionPurpose::Compliance)
        .owned_by("security")
        .retained_as(RetentionTier::new(14, 76, 275))
        .ingesting_gib_per_day(4.0)
        .redacting_sensitive_fields()
        .reviewed_every(180);

    let archived_evaluation = evaluate_retention(&archived_policy);
    assert!(archived_evaluation.ready);
    assert_eq!(archived_evaluation.total_retention_days, 365);
}
