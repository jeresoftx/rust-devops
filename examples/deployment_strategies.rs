use rust_devops::deployment_strategies::{
    DeploymentDecision, DeploymentFinding, DeploymentStrategy, DeploymentStrategyKind,
    HealthSignal, evaluate_deployment,
};

fn main() {
    let canary =
        DeploymentStrategy::new("booking-api-1.0.0", DeploymentStrategyKind::Canary, 5, 50)
            .observing(HealthSignal::ErrorRate)
            .observing(HealthSignal::Latency)
            .with_rollback()
            .with_version_compatibility();

    let canary_evaluation = evaluate_deployment(&canary);
    assert_eq!(canary_evaluation.decision, DeploymentDecision::Continue);
    assert!(canary_evaluation.findings.is_empty());

    let big_bang = DeploymentStrategy::new(
        "booking-api-2.0.0",
        DeploymentStrategyKind::BigBang,
        100,
        100,
    );

    let big_bang_evaluation = evaluate_deployment(&big_bang);
    assert_eq!(big_bang_evaluation.decision, DeploymentDecision::RollBack);
    assert!(
        big_bang_evaluation
            .findings
            .contains(&DeploymentFinding::FullBlastRadius)
    );
}
