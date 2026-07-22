use rust_devops::reliability_targets::{
    AlertPolicy, ReliabilityAction, ReliabilityFinding, ServiceLevelIndicator,
    ServiceLevelObjective, SliKind, evaluate_reliability,
};

fn main() {
    let sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
        .observed(99_700, 100_000);
    let slo = ServiceLevelObjective::new("checkout availability", sli, 99.9, 30);
    let alert = AlertPolicy::new("checkout slow burn")
        .owned_by("payments-oncall")
        .triggering(ReliabilityAction::Investigate)
        .with_burn_rate(1.0);

    let evaluation = evaluate_reliability(&slo, &alert);

    assert!(!evaluation.reliable);
    assert!(
        evaluation
            .findings
            .iter()
            .any(|finding| { matches!(finding, ReliabilityFinding::ErrorBudgetExhausted { .. }) })
    );
}
