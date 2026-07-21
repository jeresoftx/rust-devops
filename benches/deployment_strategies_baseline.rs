use std::time::Instant;

use rust_devops::deployment_strategies::{
    DeploymentStrategy, DeploymentStrategyKind, HealthSignal, evaluate_deployment,
};

fn canary() -> DeploymentStrategy {
    DeploymentStrategy::new("booking-api-1.0.0", DeploymentStrategyKind::Canary, 5, 50)
        .observing(HealthSignal::ErrorRate)
        .observing(HealthSignal::Latency)
        .with_rollback()
        .with_version_compatibility()
}

fn risky_rolling() -> DeploymentStrategy {
    DeploymentStrategy::new(
        "booking-api-1.1.0",
        DeploymentStrategyKind::Rolling,
        75,
        100,
    )
    .observing(HealthSignal::ErrorRate)
}

fn big_bang() -> DeploymentStrategy {
    DeploymentStrategy::new(
        "booking-api-2.0.0",
        DeploymentStrategyKind::BigBang,
        100,
        100,
    )
}

fn main() {
    let canary = canary();
    let rolling = risky_rolling();
    let big_bang = big_bang();
    let iterations = 100_000;
    let started = Instant::now();
    let mut continuing = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for strategy in [&canary, &rolling, &big_bang] {
            let evaluation = evaluate_deployment(strategy);
            if evaluation.findings.is_empty() {
                continuing += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(continuing, iterations);
    assert_eq!(total_findings, iterations * 7);

    println!(
        "evaluated {iterations} deployment strategy triples in {:?}",
        started.elapsed()
    );
}
