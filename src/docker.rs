//! Modelo mínimo para razonar sobre Docker como contrato de ejecución.
//!
//! El módulo no invoca Docker ni depende de su daemon. Representa las piezas
//! que el capítulo necesita enseñar: una imagen publicable, una especificación
//! de contenedor y las invariantes operativas que conviene revisar antes de
//! desplegar.

/// Protocolo de transporte expuesto por un contenedor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// TCP, usado por HTTP, gRPC, bases de datos y la mayoría de servicios.
    Tcp,
    /// UDP, usado por DNS, telemetría específica o protocolos de baja latencia.
    Udp,
}

/// Puerto declarado por el contrato de ejecución.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortMapping {
    /// Puerto dentro del contenedor.
    pub container_port: u16,
    /// Puerto publicado en el host, si se expone fuera del runtime.
    pub host_port: Option<u16>,
    /// Protocolo usado por el puerto.
    pub protocol: Protocol,
}

impl PortMapping {
    /// Declara un puerto TCP interno sin publicarlo en el host.
    ///
    /// ```
    /// let port = rust_devops::docker::PortMapping::tcp(8080);
    /// assert_eq!(port.host_port, None);
    /// ```
    pub fn tcp(container_port: u16) -> Self {
        Self {
            container_port,
            host_port: None,
            protocol: Protocol::Tcp,
        }
    }

    /// Devuelve una copia del puerto publicado en el host.
    ///
    /// ```
    /// let port = rust_devops::docker::PortMapping::tcp(8080).published_on(80);
    /// assert_eq!(port.host_port, Some(80));
    /// ```
    pub fn published_on(mut self, host_port: u16) -> Self {
        self.host_port = Some(host_port);
        self
    }
}

/// Variable de entorno declarada para imagen o contenedor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvironmentVariable {
    /// Nombre de la variable.
    pub key: &'static str,
    /// Valor visible de la variable.
    pub value: &'static str,
}

impl EnvironmentVariable {
    /// Crea una variable de entorno.
    pub fn new(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }

    fn looks_like_secret(self) -> bool {
        let key = self.key.to_ascii_uppercase();

        key.contains("SECRET")
            || key.contains("PASSWORD")
            || key.contains("TOKEN")
            || key.contains("PRIVATE_KEY")
    }
}

/// Tipo de montaje usado por el contenedor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountKind {
    /// Volumen administrado por el runtime.
    Volume,
    /// Directorio o archivo del host montado dentro del contenedor.
    Bind,
}

/// Montaje de datos fuera del filesystem efímero del contenedor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VolumeMount {
    /// Fuente del montaje: nombre del volumen o ruta del host.
    pub source: &'static str,
    /// Ruta de destino dentro del contenedor.
    pub target: &'static str,
    /// Tipo de montaje.
    pub kind: MountKind,
}

impl VolumeMount {
    /// Crea un volumen administrado por el runtime.
    pub fn volume(source: &'static str, target: &'static str) -> Self {
        Self {
            source,
            target,
            kind: MountKind::Volume,
        }
    }
}

/// Imagen Docker/OCI vista como artefacto inmutable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageSpec {
    /// Repositorio de la imagen, por ejemplo `ghcr.io/jeresoftx/api`.
    pub repository: &'static str,
    /// Tag explícito de la imagen.
    pub tag: &'static str,
    /// Usuario declarado para ejecutar el proceso principal.
    pub user: &'static str,
    /// Comando o binario principal de la imagen.
    pub entrypoint: &'static str,
    /// Variables horneadas dentro de la imagen.
    pub baked_environment: Vec<EnvironmentVariable>,
    /// Puertos que la imagen documenta como interfaz.
    pub exposed_ports: Vec<PortMapping>,
}

impl ImageSpec {
    /// Crea una imagen con los campos mínimos del contrato.
    pub fn new(
        repository: &'static str,
        tag: &'static str,
        user: &'static str,
        entrypoint: &'static str,
    ) -> Self {
        Self {
            repository,
            tag,
            user,
            entrypoint,
            baked_environment: Vec::new(),
            exposed_ports: Vec::new(),
        }
    }

    /// Agrega una variable horneada en la imagen.
    pub fn with_baked_env(mut self, variable: EnvironmentVariable) -> Self {
        self.baked_environment.push(variable);
        self
    }

    /// Declara un puerto expuesto por la imagen.
    pub fn exposing(mut self, port: PortMapping) -> Self {
        self.exposed_ports.push(port);
        self
    }
}

/// Especificación de ejecución de un contenedor creado desde una imagen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerSpec {
    /// Imagen base del contenedor.
    pub image: ImageSpec,
    /// Variables definidas en tiempo de ejecución.
    pub runtime_environment: Vec<EnvironmentVariable>,
    /// Puertos publicados por el contenedor.
    pub ports: Vec<PortMapping>,
    /// Montajes persistentes o compartidos.
    pub mounts: Vec<VolumeMount>,
    /// Límite de memoria en MiB, si se declara.
    pub memory_limit_mib: Option<u32>,
}

impl ContainerSpec {
    /// Crea una especificación de contenedor desde una imagen.
    pub fn from_image(image: ImageSpec) -> Self {
        Self {
            image,
            runtime_environment: Vec::new(),
            ports: Vec::new(),
            mounts: Vec::new(),
            memory_limit_mib: None,
        }
    }

    /// Agrega una variable de entorno de runtime.
    pub fn with_runtime_env(mut self, variable: EnvironmentVariable) -> Self {
        self.runtime_environment.push(variable);
        self
    }

    /// Publica un puerto.
    pub fn publishing(mut self, port: PortMapping) -> Self {
        self.ports.push(port);
        self
    }

    /// Agrega un montaje de datos.
    pub fn mounting(mut self, mount: VolumeMount) -> Self {
        self.mounts.push(mount);
        self
    }

    /// Declara un límite de memoria en MiB.
    pub fn with_memory_limit_mib(mut self, memory_limit_mib: u32) -> Self {
        self.memory_limit_mib = Some(memory_limit_mib);
        self
    }
}

/// Hallazgo producido al validar un contrato de ejecución.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractFinding {
    /// La imagen usa `latest` o deja el tag vacío.
    NonReproducibleTag,
    /// La imagen no declara proceso principal.
    MissingEntrypoint,
    /// La imagen ejecuta como root.
    RunsAsRoot,
    /// La imagen contiene una variable que parece secreto.
    SecretBakedIntoImage(&'static str),
    /// El contenedor publica un puerto no declarado por la imagen.
    UndeclaredPublishedPort(u16),
    /// El contenedor no declara límite de memoria.
    MissingMemoryLimit,
}

/// Valida las invariantes operativas mínimas del capítulo.
///
/// ```
/// use rust_devops::docker::{
///     validate_execution_contract, ContainerSpec, ImageSpec, PortMapping,
/// };
///
/// let image = ImageSpec::new("ghcr.io/jeresoftx/api", "1.0.0", "1000", "/app/api")
///     .exposing(PortMapping::tcp(8080));
/// let container = ContainerSpec::from_image(image)
///     .publishing(PortMapping::tcp(8080).published_on(8080))
///     .with_memory_limit_mib(256);
///
/// assert!(validate_execution_contract(&container).is_empty());
/// ```
pub fn validate_execution_contract(container: &ContainerSpec) -> Vec<ContractFinding> {
    let mut findings = Vec::new();

    if container.image.tag.is_empty() || container.image.tag == "latest" {
        findings.push(ContractFinding::NonReproducibleTag);
    }

    if container.image.entrypoint.trim().is_empty() {
        findings.push(ContractFinding::MissingEntrypoint);
    }

    if container.image.user == "root" || container.image.user == "0" {
        findings.push(ContractFinding::RunsAsRoot);
    }

    for variable in &container.image.baked_environment {
        if variable.looks_like_secret() {
            findings.push(ContractFinding::SecretBakedIntoImage(variable.key));
        }
    }

    for published_port in &container.ports {
        let declared_by_image = container.image.exposed_ports.iter().any(|image_port| {
            image_port.container_port == published_port.container_port
                && image_port.protocol == published_port.protocol
        });

        if !declared_by_image {
            findings.push(ContractFinding::UndeclaredPublishedPort(
                published_port.container_port,
            ));
        }
    }

    if container.memory_limit_mib.is_none() {
        findings.push(ContractFinding::MissingMemoryLimit);
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_contract_has_no_findings() {
        let image = ImageSpec::new("ghcr.io/jeresoftx/api", "1.0.0", "1000", "/app/api")
            .exposing(PortMapping::tcp(8080));
        let container = ContainerSpec::from_image(image)
            .publishing(PortMapping::tcp(8080).published_on(8080))
            .mounting(VolumeMount::volume("api-data", "/var/lib/api"))
            .with_memory_limit_mib(256);

        assert!(validate_execution_contract(&container).is_empty());
    }

    #[test]
    fn detects_non_reproducible_and_privileged_contract() {
        let image = ImageSpec::new("local/api", "latest", "root", "")
            .with_baked_env(EnvironmentVariable::new("DATABASE_PASSWORD", "secret"));
        let container = ContainerSpec::from_image(image).publishing(PortMapping::tcp(8080));

        assert_eq!(
            validate_execution_contract(&container),
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
}
