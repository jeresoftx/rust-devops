use rust_devops::reliability_targets::{
    AlertPolicy, ReliabilityAction, ReliabilityFinding, ServiceLevelIndicator,
    ServiceLevelObjective, SliKind, evaluate_reliability,
};

fn main() {
    let healthy_sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
        .observed(99_950, 100_000);
    let healthy_slo = ServiceLevelObjective::new("checkout availability", healthy_sli, 99.9, 30);
    let actionable_alert = AlertPolicy::new("checkout fast burn")
        .owned_by("payments-oncall")
        .triggering(ReliabilityAction::PauseRollout)
        .with_burn_rate(2.0)
        .paging_human();

    let healthy_evaluation = evaluate_reliability(&healthy_slo, &actionable_alert);
    assert!(healthy_evaluation.reliable);

    let exhausted_sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
        .observed(99_700, 100_000);
    let exhausted_slo =
        ServiceLevelObjective::new("checkout availability", exhausted_sli, 99.9, 30);
    let noisy_alert = AlertPolicy::new("checkout fast burn").with_burn_rate(3.0);

    let exhausted_evaluation = evaluate_reliability(&exhausted_slo, &noisy_alert);
    assert!(!exhausted_evaluation.reliable);
    assert!(
        exhausted_evaluation
            .findings
            .iter()
            .any(|finding| { matches!(finding, ReliabilityFinding::ErrorBudgetExhausted { .. }) })
    );
    assert!(
        exhausted_evaluation
            .findings
            .contains(&ReliabilityFinding::MissingAlertOwner)
    );
    assert!(
        exhausted_evaluation
            .findings
            .contains(&ReliabilityFinding::MissingAlertAction)
    );
}
