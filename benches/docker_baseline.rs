use std::time::Instant;

use rust_devops::docker::{
    ContainerSpec, EnvironmentVariable, ImageSpec, PortMapping, VolumeMount,
    validate_execution_contract,
};

fn valid_contract() -> ContainerSpec {
    let image = ImageSpec::new(
        "ghcr.io/jeresoftx/catalog-api",
        "1.0.0",
        "1000",
        "/app/catalog",
    )
    .exposing(PortMapping::tcp(8080));

    ContainerSpec::from_image(image)
        .publishing(PortMapping::tcp(8080).published_on(8080))
        .mounting(VolumeMount::volume("catalog-data", "/var/lib/catalog"))
        .with_memory_limit_mib(256)
}

fn risky_contract() -> ContainerSpec {
    let image = ImageSpec::new("local/catalog-api", "latest", "root", "").with_baked_env(
        EnvironmentVariable::new("DATABASE_PASSWORD", "not-for-images"),
    );

    ContainerSpec::from_image(image).publishing(PortMapping::tcp(8080))
}

fn main() {
    let contracts = [valid_contract(), risky_contract()];
    let iterations = 50_000;
    let started = Instant::now();
    let mut finding_count = 0;

    for _ in 0..iterations {
        for contract in &contracts {
            finding_count += validate_execution_contract(contract).len();
        }
    }

    assert_eq!(finding_count, iterations * 6);
    println!(
        "validated {} contracts in {:?}",
        iterations * contracts.len(),
        started.elapsed()
    );
}
