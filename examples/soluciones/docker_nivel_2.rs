use rust_devops::docker::{
    ContainerSpec, ContractFinding, EnvironmentVariable, ImageSpec, PortMapping,
    validate_execution_contract,
};

fn main() {
    let image = ImageSpec::new("local/catalog-api", "latest", "root", "").with_baked_env(
        EnvironmentVariable::new("DATABASE_PASSWORD", "not-for-images"),
    );
    let container = ContainerSpec::from_image(image).publishing(PortMapping::tcp(8080));

    let findings = validate_execution_contract(&container);

    assert_eq!(
        findings,
        vec![
            ContractFinding::NonReproducibleTag,
            ContractFinding::MissingEntrypoint,
            ContractFinding::RunsAsRoot,
            ContractFinding::SecretBakedIntoImage("DATABASE_PASSWORD"),
            ContractFinding::UndeclaredPublishedPort(8080),
            ContractFinding::MissingMemoryLimit,
        ]
    );
}
