use rust_devops::docker::{
    ContainerSpec, ImageSpec, PortMapping, VolumeMount, validate_execution_contract,
};

fn main() {
    let image = ImageSpec::new(
        "ghcr.io/jeresoftx/catalog-api",
        "1.0.0",
        "1000",
        "/app/catalog",
    )
    .exposing(PortMapping::tcp(8080));

    let container = ContainerSpec::from_image(image)
        .publishing(PortMapping::tcp(8080).published_on(8080))
        .mounting(VolumeMount::volume("catalog-data", "/var/lib/catalog"))
        .with_memory_limit_mib(256);

    assert!(validate_execution_contract(&container).is_empty());
}
