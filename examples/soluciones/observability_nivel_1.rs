use rust_devops::observability::{
    ObservabilityPlan, OperationalAction, OperationalQuestion, SignalKind, TelemetrySignal,
    evaluate_observability,
};

fn main() {
    let plan = ObservabilityPlan::new(OperationalQuestion::ReleaseHealth)
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

    let evaluation = evaluate_observability(&plan);

    assert!(evaluation.observable);
    assert!(evaluation.findings.is_empty());
}
