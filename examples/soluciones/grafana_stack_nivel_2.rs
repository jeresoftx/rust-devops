use rust_devops::grafana_stack::{
    GrafanaComponent, GrafanaStackFinding, LabelCardinality, TelemetryLabel, TelemetryRoute,
    TelemetrySignalKind, evaluate_route,
};

fn main() {
    let route = TelemetryRoute::new(TelemetrySignalKind::Log, "checkout-worker")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
        .retained_for(14)
        .with_correlation_key()
        .answering("¿qué falló durante el incidente?");

    let evaluation = evaluate_route(&route);

    assert!(!evaluation.ready);
    assert_eq!(
        evaluation.findings,
        vec![GrafanaStackFinding::WrongBackend {
            expected: GrafanaComponent::Loki,
            actual: GrafanaComponent::Prometheus,
        }]
    );
}
