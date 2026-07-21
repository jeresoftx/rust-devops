use rust_devops::kubernetes::{
    ApplicationSpec, ObservedWorkload, ProbeSpec, ReconciliationAction, ResourceSpec, ServicePort,
    ServiceSpec, WorkloadSpec, reconcile,
};

fn main() {
    let workload = WorkloadSpec::new("billing-api", "ghcr.io/jeresoftx/billing-api:1.0.0", 3)
        .exposing(8080)
        .with_probes(ProbeSpec::new(true, true))
        .with_resources(ResourceSpec::bounded(250, 256));
    let app = ApplicationSpec::new(workload).with_service(
        ServiceSpec::new("billing-api", "billing-api").exposing(ServicePort::new(80, 8080)),
    );

    let plan = reconcile(&app, ObservedWorkload::new("billing-api", 2, 2));

    assert_eq!(
        plan.actions,
        vec![
            ReconciliationAction::ScaleUp(1),
            ReconciliationAction::WaitForReadiness(1),
        ]
    );
    assert!(plan.findings.is_empty());
}
