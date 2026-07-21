use rust_devops::deployment_strategies::{
    DeploymentDecision, DeploymentFinding, DeploymentStrategy, DeploymentStrategyKind,
    HealthSignal, evaluate_deployment,
};

fn main() {
    let strategy = DeploymentStrategy::new(
        "billing-api-1.1.0",
        DeploymentStrategyKind::Rolling,
        75,
        100,
    )
    .observing(HealthSignal::ErrorRate);

    let evaluation = evaluate_deployment(&strategy);

    assert_eq!(evaluation.decision, DeploymentDecision::Pause);
    assert_eq!(
        evaluation.findings,
        vec![
            DeploymentFinding::ExcessiveInitialExposure,
            DeploymentFinding::MissingHealthSignals,
            DeploymentFinding::MissingRollback,
            DeploymentFinding::MissingVersionCompatibility,
        ]
    );
}
