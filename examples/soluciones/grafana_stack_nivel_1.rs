use rust_devops::grafana_stack::{
    GrafanaComponent, LabelCardinality, TelemetryLabel, TelemetryRoute, TelemetrySignalKind,
    evaluate_route,
};

fn main() {
    let route = TelemetryRoute::new(TelemetrySignalKind::Metric, "checkout-api")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("environment", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("version", LabelCardinality::Bounded))
        .retained_for(30)
        .with_correlation_key()
        .answering("¿la versión nueva está sana?");

    let evaluation = evaluate_route(&route);

    assert!(evaluation.ready);
    assert!(evaluation.findings.is_empty());
}
