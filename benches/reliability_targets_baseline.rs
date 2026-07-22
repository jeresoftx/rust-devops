use std::time::Instant;

use rust_devops::reliability_targets::{
    AlertPolicy, ReliabilityAction, ServiceLevelIndicator, ServiceLevelObjective, SliKind,
    evaluate_reliability,
};

fn healthy_checkout_slo() -> (ServiceLevelObjective, AlertPolicy) {
    let sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
        .observed(99_950, 100_000);
    let slo = ServiceLevelObjective::new("checkout availability", sli, 99.9, 30);
    let alert = AlertPolicy::new("checkout fast burn")
        .owned_by("payments-oncall")
        .triggering(ReliabilityAction::PauseRollout)
        .with_burn_rate(2.0)
        .paging_human();

    (slo, alert)
}

fn exhausted_checkout_slo() -> (ServiceLevelObjective, AlertPolicy) {
    let sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
        .observed(99_700, 100_000);
    let slo = ServiceLevelObjective::new("checkout availability", sli, 99.9, 30);
    let alert = AlertPolicy::new("checkout slow burn")
        .owned_by("payments-oncall")
        .triggering(ReliabilityAction::Investigate)
        .with_burn_rate(1.0);

    (slo, alert)
}

fn noisy_latency_alert() -> (ServiceLevelObjective, AlertPolicy) {
    let sli = ServiceLevelIndicator::new("api_latency_under_threshold", SliKind::Latency)
        .observed(99_000, 100_000);
    let slo = ServiceLevelObjective::new("api latency", sli, 99.0, 28);
    let alert = AlertPolicy::new("api fast burn").with_burn_rate(3.0);

    (slo, alert)
}

fn main() {
    let healthy = healthy_checkout_slo();
    let exhausted = exhausted_checkout_slo();
    let noisy = noisy_latency_alert();
    let iterations = 100_000;
    let started = Instant::now();
    let mut reliable_targets = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for (slo, alert) in [&healthy, &exhausted, &noisy] {
            let evaluation = evaluate_reliability(slo, alert);
            if evaluation.reliable {
                reliable_targets += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(reliable_targets, iterations);
    assert_eq!(total_findings, iterations * 4);

    println!(
        "evaluated {iterations} reliability target triples in {:?}",
        started.elapsed()
    );
}
