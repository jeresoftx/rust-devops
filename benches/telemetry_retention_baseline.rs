use std::time::Instant;

use rust_devops::telemetry_retention::{
    DataSensitivity, RetentionPurpose, RetentionTier, TelemetryRetentionPolicy,
    TelemetrySignalKind, evaluate_retention,
};

fn metric_policy() -> TelemetryRetentionPolicy {
    TelemetryRetentionPolicy::new("checkout_metrics", TelemetrySignalKind::Metric)
        .for_purpose(RetentionPurpose::TrendAnalysis)
        .owned_by("platform-observability")
        .retained_as(RetentionTier::new(14, 46, 305))
        .ingesting_gib_per_day(1.5)
        .reviewed_every(90)
}

fn sensitive_log_policy() -> TelemetryRetentionPolicy {
    TelemetryRetentionPolicy::new("checkout_logs", TelemetrySignalKind::Log)
        .with_sensitivity(DataSensitivity::Sensitive)
        .for_purpose(RetentionPurpose::IncidentInvestigation)
        .owned_by("payments-oncall")
        .retained_as(RetentionTier::new(45, 0, 0))
        .ingesting_gib_per_day(8.0)
        .reviewed_every(30)
}

fn regulated_policy_without_archive() -> TelemetryRetentionPolicy {
    TelemetryRetentionPolicy::new("audit_logs", TelemetrySignalKind::Log)
        .with_sensitivity(DataSensitivity::Regulated)
        .for_purpose(RetentionPurpose::Compliance)
        .owned_by("security")
        .retained_as(RetentionTier::new(14, 76, 0))
        .ingesting_gib_per_day(4.0)
        .reviewed_every(180)
}

fn main() {
    let metric = metric_policy();
    let sensitive = sensitive_log_policy();
    let regulated = regulated_policy_without_archive();
    let iterations = 100_000;
    let started = Instant::now();
    let mut ready_policies = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for policy in [&metric, &sensitive, &regulated] {
            let evaluation = evaluate_retention(policy);
            if evaluation.ready {
                ready_policies += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(ready_policies, iterations);
    assert_eq!(total_findings, iterations * 4);

    println!(
        "evaluated {iterations} telemetry retention policy triples in {:?}",
        started.elapsed()
    );
}
