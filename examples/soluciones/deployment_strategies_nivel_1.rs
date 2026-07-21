use rust_devops::deployment_strategies::{
    DeploymentDecision, DeploymentStrategy, DeploymentStrategyKind, HealthSignal,
    evaluate_deployment,
};

fn main() {
    let strategy =
        DeploymentStrategy::new("billing-api-1.0.0", DeploymentStrategyKind::Canary, 5, 50)
            .observing(HealthSignal::ErrorRate)
            .observing(HealthSignal::Latency)
            .with_rollback()
            .with_version_compatibility();

    let evaluation = evaluate_deployment(&strategy);

    assert_eq!(evaluation.decision, DeploymentDecision::Continue);
    assert!(evaluation.findings.is_empty());
}
