use rust_devops::observability::{
    ObservabilityFinding, ObservabilityPlan, OperationalAction, OperationalQuestion, SignalKind,
    TelemetrySignal, evaluate_observability,
};

fn supporting_log() -> TelemetrySignal {
    TelemetrySignal::new(SignalKind::Log, "checkout.error")
        .emitted_by("checkout-api", "production")
        .for_version("v1.2.0")
        .with_correlation_key()
        .structured()
        .retained_for(30)
        .enabling(OperationalAction::Investigate)
}

fn supporting_trace() -> TelemetrySignal {
    TelemetrySignal::new(SignalKind::Trace, "checkout.request")
        .emitted_by("checkout-api", "production")
        .for_version("v1.2.0")
        .with_correlation_key()
        .structured()
        .retained_for(7)
        .enabling(OperationalAction::RollBack)
}

fn main() {
    let risky_metric = ObservabilityPlan::new(OperationalQuestion::UserImpact)
        .observing(supporting_log())
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
        .observing(supporting_trace());
    let risky_evaluation = evaluate_observability(&risky_metric);

    assert_eq!(
        risky_evaluation.findings,
        vec![ObservabilityFinding::HighCardinalityMetric(
            "requests_by_user_id",
        )]
    );

    let hardened_metric = ObservabilityPlan::new(OperationalQuestion::UserImpact)
        .observing(supporting_log())
        .observing(
            TelemetrySignal::new(SignalKind::Metric, "requests_by_plan")
                .emitted_by("checkout-api", "production")
                .for_version("v1.2.0")
                .with_correlation_key()
                .structured()
                .retained_for(30)
                .enabling(OperationalAction::Escalate),
        )
        .observing(supporting_trace());
    let hardened_evaluation = evaluate_observability(&hardened_metric);

    assert!(hardened_evaluation.observable);
    assert!(hardened_evaluation.findings.is_empty());
}
