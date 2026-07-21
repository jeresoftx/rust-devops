use rust_devops::deployment_strategies::{
    DeploymentDecision, DeploymentFinding, DeploymentStrategy, DeploymentStrategyKind,
    HealthSignal, evaluate_deployment,
};

fn main() {
    let unsafe_strategy = DeploymentStrategy::new(
        "billing-api-2.0.0",
        DeploymentStrategyKind::BigBang,
        100,
        100,
    );
    let unsafe_evaluation = evaluate_deployment(&unsafe_strategy);

    assert_eq!(unsafe_evaluation.decision, DeploymentDecision::RollBack);
    assert!(
        unsafe_evaluation
            .findings
            .contains(&DeploymentFinding::FullBlastRadius)
    );

    let hardened_strategy =
        DeploymentStrategy::new("billing-api-2.0.0", DeploymentStrategyKind::Canary, 5, 50)
            .observing(HealthSignal::ErrorRate)
            .observing(HealthSignal::Latency)
            .observing(HealthSignal::BusinessMetric)
            .with_rollback()
            .with_version_compatibility();
    let hardened_evaluation = evaluate_deployment(&hardened_strategy);

    assert_eq!(hardened_evaluation.decision, DeploymentDecision::Continue);
    assert!(hardened_evaluation.findings.is_empty());
}
