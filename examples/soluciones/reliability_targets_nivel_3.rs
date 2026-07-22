use rust_devops::reliability_targets::{
    AlertPolicy, ReliabilityAction, ReliabilityFinding, ServiceLevelIndicator,
    ServiceLevelObjective, SliKind, evaluate_reliability,
};

fn main() {
    let sli = ServiceLevelIndicator::new("api_latency_under_threshold", SliKind::Latency)
        .observed(99_000, 100_000);
    let slo = ServiceLevelObjective::new("api latency", sli, 99.0, 28);
    let noisy_alert = AlertPolicy::new("api fast burn").with_burn_rate(3.0);

    let noisy_evaluation = evaluate_reliability(&slo, &noisy_alert);
    assert!(!noisy_evaluation.reliable);
    assert!(
        noisy_evaluation
            .findings
            .contains(&ReliabilityFinding::MissingAlertOwner)
    );
    assert!(
        noisy_evaluation
            .findings
            .contains(&ReliabilityFinding::MissingAlertAction)
    );
    assert!(
        noisy_evaluation
            .findings
            .contains(&ReliabilityFinding::FastBurnWithoutPage)
    );

    let actionable_alert = AlertPolicy::new("api fast burn")
        .owned_by("platform-oncall")
        .triggering(ReliabilityAction::Escalate)
        .with_burn_rate(3.0)
        .paging_human();

    let actionable_evaluation = evaluate_reliability(&slo, &actionable_alert);
    assert!(actionable_evaluation.reliable);
}
