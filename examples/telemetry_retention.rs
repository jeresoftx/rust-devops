use rust_devops::telemetry_retention::{
    DataSensitivity, RetentionPurpose, RetentionTier, TelemetryRetentionFinding,
    TelemetryRetentionPolicy, TelemetrySignalKind, evaluate_retention,
};

fn main() {
    let metric_policy =
        TelemetryRetentionPolicy::new("checkout_metrics", TelemetrySignalKind::Metric)
            .for_purpose(RetentionPurpose::TrendAnalysis)
            .owned_by("platform-observability")
            .retained_as(RetentionTier::new(14, 46, 305))
            .ingesting_gib_per_day(1.5)
            .reviewed_every(90);

    let metric_evaluation = evaluate_retention(&metric_policy);
    assert!(metric_evaluation.ready);
    assert_eq!(metric_evaluation.total_retention_days, 365);
    assert_eq!(metric_evaluation.estimated_hot_gib, Some(21.0));

    let risky_log_policy = TelemetryRetentionPolicy::new("checkout_logs", TelemetrySignalKind::Log)
        .with_sensitivity(DataSensitivity::Sensitive)
        .for_purpose(RetentionPurpose::IncidentInvestigation)
        .owned_by("payments-oncall")
        .retained_as(RetentionTier::new(45, 0, 0))
        .ingesting_gib_per_day(8.0)
        .reviewed_every(30);

    let risky_evaluation = evaluate_retention(&risky_log_policy);
    assert!(!risky_evaluation.ready);
    assert!(
        risky_evaluation
            .findings
            .contains(&TelemetryRetentionFinding::SensitiveDataWithoutRedaction)
    );
    assert!(
        risky_evaluation
            .findings
            .contains(&TelemetryRetentionFinding::ExcessiveHotRetention { hot_days: 45 })
    );
}
