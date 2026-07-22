use rust_devops::reliability_targets::{
    AlertPolicy, ReliabilityAction, ServiceLevelIndicator, ServiceLevelObjective, SliKind,
    evaluate_reliability,
};

fn main() {
    let sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
        .observed(99_950, 100_000);
    let slo = ServiceLevelObjective::new("checkout availability", sli, 99.9, 30);
    let alert = AlertPolicy::new("checkout fast burn")
        .owned_by("payments-oncall")
        .triggering(ReliabilityAction::PauseRollout)
        .with_burn_rate(2.0)
        .paging_human();

    let evaluation = evaluate_reliability(&slo, &alert);

    assert!(evaluation.reliable);
    assert!(evaluation.findings.is_empty());
    assert_eq!(evaluation.success_percent, Some(99.95));
}
