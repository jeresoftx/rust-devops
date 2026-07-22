use std::time::Instant;

use rust_devops::grafana_stack::{
    GrafanaComponent, LabelCardinality, TelemetryLabel, TelemetryRoute, TelemetrySignalKind,
    evaluate_route,
};

fn complete_metric_route() -> TelemetryRoute {
    TelemetryRoute::new(TelemetrySignalKind::Metric, "checkout-api")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("environment", LabelCardinality::Low))
        .labeled_with(TelemetryLabel::new("version", LabelCardinality::Bounded))
        .retained_for(30)
        .with_correlation_key()
        .answering("¿la versión nueva está sana?")
}

fn wrong_log_backend() -> TelemetryRoute {
    TelemetryRoute::new(TelemetrySignalKind::Log, "checkout-worker")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
        .retained_for(14)
        .with_correlation_key()
        .answering("¿qué falló durante el incidente?")
}

fn unbounded_metric_label() -> TelemetryRoute {
    TelemetryRoute::new(TelemetrySignalKind::Metric, "requests_by_user_email")
        .collected_by(GrafanaComponent::Alloy)
        .stored_in(GrafanaComponent::Prometheus)
        .visualized_in(GrafanaComponent::Grafana)
        .labeled_with(TelemetryLabel::new(
            "user_email",
            LabelCardinality::Unbounded,
        ))
        .retained_for(30)
        .with_correlation_key()
        .answering("¿qué usuarios están fallando?")
}

fn main() {
    let complete = complete_metric_route();
    let wrong_backend = wrong_log_backend();
    let cardinality = unbounded_metric_label();
    let iterations = 100_000;
    let started = Instant::now();
    let mut ready_routes = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for route in [&complete, &wrong_backend, &cardinality] {
            let evaluation = evaluate_route(route);
            if evaluation.ready {
                ready_routes += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(ready_routes, iterations);
    assert_eq!(total_findings, iterations * 2);

    println!(
        "evaluated {iterations} grafana stack route triples in {:?}",
        started.elapsed()
    );
}
