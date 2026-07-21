use std::time::Instant;

use rust_devops::kubernetes::{
    ApplicationSpec, ObservedWorkload, ProbeSpec, ResourceSpec, ServicePort, ServiceSpec,
    WorkloadSpec, reconcile,
};

fn healthy_app() -> ApplicationSpec {
    let workload = WorkloadSpec::new("booking-api", "ghcr.io/jeresoftx/booking-api:1.0.0", 3)
        .exposing(8080)
        .with_probes(ProbeSpec::new(true, true))
        .with_resources(ResourceSpec::bounded(250, 256));

    ApplicationSpec::new(workload).with_service(
        ServiceSpec::new("booking-api", "booking-api").exposing(ServicePort::new(80, 8080)),
    )
}

fn risky_app() -> ApplicationSpec {
    let workload = WorkloadSpec::new("booking-api", "ghcr.io/jeresoftx/booking-api:latest", 0);

    ApplicationSpec::new(workload)
        .with_service(ServiceSpec::new("public-api", "worker").exposing(ServicePort::new(80, 8080)))
}

fn main() {
    let healthy = healthy_app();
    let risky = risky_app();
    let iterations = 100_000;
    let started = Instant::now();
    let mut total_actions = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        let healthy_plan = reconcile(&healthy, ObservedWorkload::new("booking-api", 2, 2));
        let risky_plan = reconcile(&risky, ObservedWorkload::new("booking-api", 1, 0));

        total_actions += healthy_plan.actions.len() + risky_plan.actions.len();
        total_findings += healthy_plan.findings.len() + risky_plan.findings.len();
    }

    assert_eq!(total_actions, iterations * 2);
    assert_eq!(total_findings, iterations * 7);

    println!(
        "validated {iterations} kubernetes reconciliation pairs in {:?}",
        started.elapsed()
    );
}
