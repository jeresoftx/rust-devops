use rust_devops::docker::{
    ContainerSpec, ContractFinding, EnvironmentVariable, ImageSpec, PortMapping, VolumeMount,
    validate_execution_contract,
};

fn main() {
    let image = ImageSpec::new(
        "ghcr.io/jeresoftx/booking-api",
        "1.0.0",
        "1000",
        "/app/server",
    )
    .exposing(PortMapping::tcp(8080));

    let container = ContainerSpec::from_image(image)
        .with_runtime_env(EnvironmentVariable::new("RUST_LOG", "info"))
        .publishing(PortMapping::tcp(8080).published_on(8080))
        .mounting(VolumeMount::volume("booking-data", "/var/lib/booking"))
        .with_memory_limit_mib(512);

    let findings = validate_execution_contract(&container);
    assert!(findings.is_empty());

    let risky_image = ImageSpec::new("local/booking-api", "latest", "root", "").with_baked_env(
        EnvironmentVariable::new("DATABASE_PASSWORD", "not-for-images"),
    );
    let risky_container = ContainerSpec::from_image(risky_image).publishing(PortMapping::tcp(8080));
    let risky_findings = validate_execution_contract(&risky_container);

    assert!(risky_findings.contains(&ContractFinding::NonReproducibleTag));
    assert!(risky_findings.contains(&ContractFinding::RunsAsRoot));
    assert!(risky_findings.contains(&ContractFinding::MissingMemoryLimit));
}
