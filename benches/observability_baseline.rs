use std::time::Instant;

use rust_devops::observability::{
    ObservabilityPlan, OperationalAction, OperationalQuestion, SignalKind, TelemetrySignal,
    evaluate_observability,
};

fn complete_release_health() -> ObservabilityPlan {
    ObservabilityPlan::new(OperationalQuestion::ReleaseHealth)
        .observing(
            TelemetrySignal::new(SignalKind::Log, "checkout.error")
                .emitted_by("checkout-api", "production")
                .for_version("v1.2.0")
                .with_correlation_key()
                .structured()
                .retained_for(30)
                .enabling(OperationalAction::Investigate),
        )
        .observing(
            TelemetrySignal::new(SignalKind::Metric, "http_error_rate")
                .emitted_by("checkout-api", "production")
                .for_version("v1.2.0")
                .with_correlation_key()
                .structured()
                .retained_for(30)
                .enabling(OperationalAction::Pause),
        )
        .observing(
            TelemetrySignal::new(SignalKind::Trace, "checkout.request")
                .emitted_by("checkout-api", "production")
                .for_version("v1.2.0")
                .with_correlation_key()
                .structured()
                .retained_for(7)
                .enabling(OperationalAction::RollBack),
        )
}

fn blind_log_only() -> ObservabilityPlan {
    ObservabilityPlan::new(OperationalQuestion::IncidentDiagnosis)
        .observing(TelemetrySignal::new(SignalKind::Log, "checkout failed"))
}

fn high_cardinality_metric() -> ObservabilityPlan {
    ObservabilityPlan::new(OperationalQuestion::UserImpact)
        .observing(
            TelemetrySignal::new(SignalKind::Log, "checkout.error")
                .emitted_by("checkout-api", "production")
                .for_version("v1.2.0")
                .with_correlation_key()
                .structured()
                .retained_for(30)
                .enabling(OperationalAction::Investigate),
        )
        .observing(
            TelemetrySignal::new(SignalKind::Metric, "requests_by_user_id")
                .emitted_by("checkout-api", "production")
                .for_version("v1.2.0")
                .with_correlation_key()
                .structured()
                .retained_for(30)
                .with_high_cardinality()
                .enabling(OperationalAction::Escalate),
        )
        .observing(
            TelemetrySignal::new(SignalKind::Trace, "checkout.request")
                .emitted_by("checkout-api", "production")
                .for_version("v1.2.0")
                .with_correlation_key()
                .structured()
                .retained_for(7)
                .enabling(OperationalAction::RollBack),
        )
}

fn main() {
    let complete = complete_release_health();
    let blind = blind_log_only();
    let cardinality = high_cardinality_metric();
    let iterations = 100_000;
    let started = Instant::now();
    let mut observable = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for plan in [&complete, &blind, &cardinality] {
            let evaluation = evaluate_observability(plan);
            if evaluation.observable {
                observable += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(observable, iterations);
    assert_eq!(total_findings, iterations * 10);

    println!(
        "evaluated {iterations} observability plan triples in {:?}",
        started.elapsed()
    );
}
