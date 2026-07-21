use rust_devops::kubernetes::{
    ApplicationSpec, ClusterFinding, ObservedWorkload, ProbeSpec, ReconciliationAction,
    ResourceSpec, ServicePort, ServiceSpec, WorkloadSpec, reconcile,
};

fn main() {
    let unsafe_workload =
        WorkloadSpec::new("billing-api", "ghcr.io/jeresoftx/billing-api:latest", 0);
    let unsafe_app = ApplicationSpec::new(unsafe_workload).with_service(
        ServiceSpec::new("public-billing", "worker").exposing(ServicePort::new(80, 8080)),
    );
    let unsafe_plan = reconcile(&unsafe_app, ObservedWorkload::new("billing-api", 1, 0));

    assert!(
        unsafe_plan
            .findings
            .contains(&ClusterFinding::MissingReplicas)
    );
    assert!(
        unsafe_plan
            .findings
            .contains(&ClusterFinding::ServiceTargetsDifferentWorkload)
    );

    let hardened_workload =
        WorkloadSpec::new("billing-api", "ghcr.io/jeresoftx/billing-api:1.0.0", 3)
            .exposing(8080)
            .with_probes(ProbeSpec::new(true, true))
            .with_resources(ResourceSpec::bounded(250, 256));
    let hardened_app = ApplicationSpec::new(hardened_workload).with_service(
        ServiceSpec::new("billing-api", "billing-api").exposing(ServicePort::new(80, 8080)),
    );
    let hardened_plan = reconcile(&hardened_app, ObservedWorkload::new("billing-api", 2, 2));

    assert_eq!(
        hardened_plan.actions,
        vec![
            ReconciliationAction::ScaleUp(1),
            ReconciliationAction::WaitForReadiness(1),
        ]
    );
    assert!(hardened_plan.findings.is_empty());
}
