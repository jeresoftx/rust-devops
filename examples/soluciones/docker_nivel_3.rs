use rust_devops::docker::{
    ContainerSpec, EnvironmentVariable, ImageSpec, PortMapping, VolumeMount,
    validate_execution_contract,
};

fn risky_contract() -> ContainerSpec {
    let image = ImageSpec::new("local/catalog-api", "latest", "root", "").with_baked_env(
        EnvironmentVariable::new("DATABASE_PASSWORD", "not-for-images"),
    );

    ContainerSpec::from_image(image).publishing(PortMapping::tcp(8080))
}

fn hardened_contract() -> ContainerSpec {
    let image = ImageSpec::new(
        "ghcr.io/jeresoftx/catalog-api",
        "1.0.1",
        "1000",
        "/app/catalog",
    )
    .exposing(PortMapping::tcp(8080));

    ContainerSpec::from_image(image)
        .with_runtime_env(EnvironmentVariable::new(
            "DATABASE_URL",
            "postgres://catalog",
        ))
        .publishing(PortMapping::tcp(8080).published_on(8080))
        .mounting(VolumeMount::volume("catalog-data", "/var/lib/catalog"))
        .with_memory_limit_mib(256)
}

fn main() {
    let before = validate_execution_contract(&risky_contract());
    let after = validate_execution_contract(&hardened_contract());

    assert_eq!(before.len(), 6);
    assert!(after.is_empty());
}
