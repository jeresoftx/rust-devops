use rust_devops::observability::{
    ObservabilityFinding, ObservabilityPlan, OperationalQuestion, SignalKind, TelemetrySignal,
    evaluate_observability,
};

fn main() {
    let plan = ObservabilityPlan::new(OperationalQuestion::IncidentDiagnosis)
        .observing(TelemetrySignal::new(SignalKind::Log, "checkout failed"));

    let evaluation = evaluate_observability(&plan);

    assert!(!evaluation.observable);
    assert!(
        evaluation
            .findings
            .contains(&ObservabilityFinding::MissingMetrics)
    );
    assert!(
        evaluation
            .findings
            .contains(&ObservabilityFinding::MissingTraces)
    );
    assert!(
        evaluation
            .findings
            .contains(&ObservabilityFinding::UnstructuredLog("checkout failed"))
    );
    assert!(
        evaluation
            .findings
            .contains(&ObservabilityFinding::MissingAction("checkout failed"))
    );
}
