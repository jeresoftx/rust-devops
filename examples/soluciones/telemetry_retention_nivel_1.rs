use rust_devops::telemetry_retention::{
    RetentionPurpose, RetentionTier, TelemetryRetentionPolicy, TelemetrySignalKind,
    evaluate_retention,
};

fn main() {
    let policy = TelemetryRetentionPolicy::new("checkout_metrics", TelemetrySignalKind::Metric)
        .for_purpose(RetentionPurpose::TrendAnalysis)
        .owned_by("platform-observability")
        .retained_as(RetentionTier::new(14, 46, 305))
        .ingesting_gib_per_day(1.5)
        .reviewed_every(90);

    let evaluation = evaluate_retention(&policy);

    assert!(evaluation.ready);
    assert_eq!(evaluation.total_retention_days, 365);
    assert_eq!(evaluation.estimated_total_gib, Some(547.5));
}
