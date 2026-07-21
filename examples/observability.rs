use rust_devops::observability::{
    ObservabilityFinding, ObservabilityPlan, OperationalAction, OperationalQuestion, SignalKind,
    TelemetrySignal, evaluate_observability,
};

fn main() {
    let observable_release = ObservabilityPlan::new(OperationalQuestion::ReleaseHealth)
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
        );

    let observable_evaluation = evaluate_observability(&observable_release);
    assert!(observable_evaluation.observable);
    assert!(observable_evaluation.findings.is_empty());

    let blind_spot = ObservabilityPlan::new(OperationalQuestion::IncidentDiagnosis)
        .observing(TelemetrySignal::new(SignalKind::Log, "checkout failed"));
    let blind_spot_evaluation = evaluate_observability(&blind_spot);

    assert!(!blind_spot_evaluation.observable);
    assert!(
        blind_spot_evaluation
            .findings
            .contains(&ObservabilityFinding::MissingMetrics)
    );
    assert!(
        blind_spot_evaluation
            .findings
            .contains(&ObservabilityFinding::MissingTraces)
    );
}
