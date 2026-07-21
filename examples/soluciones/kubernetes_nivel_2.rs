use rust_devops::kubernetes::{
    ApplicationSpec, ClusterFinding, ObservedWorkload, ServicePort, ServiceSpec, WorkloadSpec,
    reconcile,
};

fn main() {
    let workload = WorkloadSpec::new("billing-api", "ghcr.io/jeresoftx/billing-api:latest", 0);
    let app = ApplicationSpec::new(workload).with_service(
        ServiceSpec::new("public-billing", "worker").exposing(ServicePort::new(80, 8080)),
    );

    let plan = reconcile(&app, ObservedWorkload::new("billing-api", 1, 0));

    assert!(plan.actions.is_empty());
    assert_eq!(
        plan.findings,
        vec![
            ClusterFinding::NonReproducibleImage,
            ClusterFinding::MissingReadinessProbe,
            ClusterFinding::MissingLivenessProbe,
            ClusterFinding::MissingResourceSpec,
            ClusterFinding::ServiceTargetsDifferentWorkload,
            ClusterFinding::ServiceTargetsUndeclaredPort(8080),
            ClusterFinding::MissingReplicas,
        ]
    );
}
