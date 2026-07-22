use rust_devops::grafana_stack::{
    GrafanaComponent, GrafanaStackFinding, LabelCardinality, TelemetryLabel, TelemetryRoute,
    TelemetrySignalKind, evaluate_route,
};

fn main() {
    let risky_route = TelemetryRoute::new(TelemetrySignalKind::Metric, "requests_by_user_email")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new(
            "user_email",
            LabelCardinality::Unbounded,
        ))
        .retained_for(30)
        .with_correlation_key()
        .answering("¿qué usuarios están fallando?");

    let risky_evaluation = evaluate_route(&risky_route);
    assert!(!risky_evaluation.ready);
    assert!(
        risky_evaluation
            .findings
            .contains(&GrafanaStackFinding::UnboundedLabel("user_email"))
    );

    let bounded_route = TelemetryRoute::new(TelemetrySignalKind::Metric, "http_error_rate")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("environment", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("version", LabelCardinality::Bounded))
        .retained_for(30)
        .with_correlation_key()
        .answering("¿la versión nueva está sana?");

    let bounded_evaluation = evaluate_route(&bounded_route);
    assert!(bounded_evaluation.ready);
}
