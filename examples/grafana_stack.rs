use rust_devops::grafana_stack::{
    GrafanaComponent, GrafanaStackFinding, LabelCardinality, TelemetryLabel, TelemetryRoute,
    TelemetrySignalKind, evaluate_route,
};

fn main() {
    let healthy_metric_route = TelemetryRoute::new(TelemetrySignalKind::Metric, "checkout-api")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("environment", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("version", LabelCardinality::Bounded))
        .retained_for(30)
        .with_correlation_key()
        .answering("¿la versión nueva está sana?");

    let healthy_evaluation = evaluate_route(&healthy_metric_route);
    assert!(healthy_evaluation.ready);

    let noisy_log_route = TelemetryRoute::new(TelemetrySignalKind::Log, "checkout-worker")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new(
            "user_email",
            LabelCardinality::Unbounded,
        ))
        .retained_for(7)
        .answering("¿qué falló durante el incidente?");

    let noisy_evaluation = evaluate_route(&noisy_log_route);
    assert!(!noisy_evaluation.ready);
    assert!(
        noisy_evaluation
            .findings
            .contains(&GrafanaStackFinding::WrongBackend {
                expected: GrafanaComponent::Loki,
                actual: GrafanaComponent::Prometheus,
            })
    );
    assert!(
        noisy_evaluation
            .findings
            .contains(&GrafanaStackFinding::UnboundedLabel("user_email"))
    );
    assert!(
        noisy_evaluation
            .findings
            .contains(&GrafanaStackFinding::MissingCorrelation)
    );
}
