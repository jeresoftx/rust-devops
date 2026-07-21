use rust_devops::kubernetes::{
    ApplicationSpec, ClusterFinding, ObservedWorkload, ProbeSpec, ReconciliationAction,
    ResourceSpec, ServicePort, ServiceSpec, WorkloadSpec, reconcile,
};

fn main() {
    let workload = WorkloadSpec::new("booking-api", "ghcr.io/jeresoftx/booking-api:1.0.0", 3)
        .exposing(8080)
        .with_probes(ProbeSpec::new(true, true))
        .with_resources(ResourceSpec::bounded(250, 256));
    let app = ApplicationSpec::new(workload).with_service(
        ServiceSpec::new("booking-api", "booking-api").exposing(ServicePort::new(80, 8080)),
    );

    let plan = reconcile(&app, ObservedWorkload::new("booking-api", 2, 2));
    assert_eq!(
        plan.actions,
        vec![
            ReconciliationAction::ScaleUp(1),
            ReconciliationAction::WaitForReadiness(1),
        ]
    );
    assert!(plan.findings.is_empty());

    let risky_workload = WorkloadSpec::new("booking-api", "booking-api:latest", 0);
    let risky_app = ApplicationSpec::new(risky_workload).with_service(
        ServiceSpec::new("public-api", "worker").exposing(ServicePort::new(80, 8080)),
    );
    let risky_plan = reconcile(&risky_app, ObservedWorkload::new("booking-api", 1, 0));

    assert!(
        risky_plan
            .findings
            .contains(&ClusterFinding::NonReproducibleImage)
    );
    assert!(
        risky_plan
            .findings
            .contains(&ClusterFinding::MissingResourceSpec)
    );
    assert!(
        risky_plan
            .findings
            .contains(&ClusterFinding::ServiceTargetsDifferentWorkload)
    );
}
