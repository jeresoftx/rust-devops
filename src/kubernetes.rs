//! Modelo mínimo para razonar sobre Kubernetes como reconciliación.
//!
//! El módulo no implementa la API real de Kubernetes. Representa solo las
//! piezas necesarias para enseñar estado deseado, estado observado y hallazgos
//! operativos antes de introducir manifiestos completos.

/// Puerto declarado por un contenedor o servicio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServicePort {
    /// Puerto interno expuesto por el contenedor.
    pub target_port: u16,
    /// Puerto estable del servicio dentro del cluster.
    pub service_port: u16,
}

impl ServicePort {
    /// Crea un puerto de servicio que apunta a un puerto de contenedor.
    ///
    /// ```
    /// let port = rust_devops::kubernetes::ServicePort::new(80, 8080);
    /// assert_eq!(port.service_port, 80);
    /// assert_eq!(port.target_port, 8080);
    /// ```
    pub fn new(service_port: u16, target_port: u16) -> Self {
        Self {
            service_port,
            target_port,
        }
    }
}

/// Health checks mínimos de una carga de trabajo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeSpec {
    /// Si la réplica puede recibir tráfico.
    pub readiness: bool,
    /// Si la réplica debe reiniciarse al quedar rota.
    pub liveness: bool,
}

impl ProbeSpec {
    /// Declara readiness y liveness.
    pub fn new(readiness: bool, liveness: bool) -> Self {
        Self {
            readiness,
            liveness,
        }
    }
}

/// Recursos mínimos para evitar competencia invisible en el cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceSpec {
    /// CPU solicitada en milicores.
    pub cpu_request_millis: u32,
    /// Memoria solicitada en MiB.
    pub memory_request_mib: u32,
    /// Límite de CPU en milicores, si se declara.
    pub cpu_limit_millis: Option<u32>,
    /// Límite de memoria en MiB, si se declara.
    pub memory_limit_mib: Option<u32>,
}

impl ResourceSpec {
    /// Crea requests con límites equivalentes.
    pub fn bounded(cpu_millis: u32, memory_mib: u32) -> Self {
        Self {
            cpu_request_millis: cpu_millis,
            memory_request_mib: memory_mib,
            cpu_limit_millis: Some(cpu_millis),
            memory_limit_mib: Some(memory_mib),
        }
    }
}

/// Workload deseado por el equipo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkloadSpec {
    /// Nombre lógico del workload.
    pub name: &'static str,
    /// Imagen versionada que ejecutarán las réplicas.
    pub image: &'static str,
    /// Número deseado de réplicas.
    pub desired_replicas: u8,
    /// Puertos que exponen los contenedores.
    pub container_ports: Vec<u16>,
    /// Probes de salud.
    pub probes: ProbeSpec,
    /// Recursos declarados.
    pub resources: Option<ResourceSpec>,
}

impl WorkloadSpec {
    /// Crea un workload con campos mínimos.
    pub fn new(name: &'static str, image: &'static str, desired_replicas: u8) -> Self {
        Self {
            name,
            image,
            desired_replicas,
            container_ports: Vec::new(),
            probes: ProbeSpec::new(false, false),
            resources: None,
        }
    }

    /// Declara un puerto de contenedor.
    pub fn exposing(mut self, port: u16) -> Self {
        self.container_ports.push(port);
        self
    }

    /// Declara probes de salud.
    pub fn with_probes(mut self, probes: ProbeSpec) -> Self {
        self.probes = probes;
        self
    }

    /// Declara recursos.
    pub fn with_resources(mut self, resources: ResourceSpec) -> Self {
        self.resources = Some(resources);
        self
    }
}

/// Servicio que enruta tráfico hacia un workload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceSpec {
    /// Nombre lógico del servicio.
    pub name: &'static str,
    /// Workload al que apunta.
    pub workload_name: &'static str,
    /// Puertos publicados dentro del cluster.
    pub ports: Vec<ServicePort>,
}

impl ServiceSpec {
    /// Crea un servicio para un workload.
    pub fn new(name: &'static str, workload_name: &'static str) -> Self {
        Self {
            name,
            workload_name,
            ports: Vec::new(),
        }
    }

    /// Agrega un puerto al servicio.
    pub fn exposing(mut self, port: ServicePort) -> Self {
        self.ports.push(port);
        self
    }
}

/// Estado observado del cluster para un workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObservedWorkload {
    /// Nombre del workload observado.
    pub name: &'static str,
    /// Réplicas existentes.
    pub current_replicas: u8,
    /// Réplicas listas para recibir tráfico.
    pub ready_replicas: u8,
}

impl ObservedWorkload {
    /// Crea un estado observado.
    pub fn new(name: &'static str, current_replicas: u8, ready_replicas: u8) -> Self {
        Self {
            name,
            current_replicas,
            ready_replicas,
        }
    }
}

/// Especificación mínima de una aplicación en Kubernetes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationSpec {
    /// Workload principal.
    pub workload: WorkloadSpec,
    /// Servicio asociado, si existe.
    pub service: Option<ServiceSpec>,
}

impl ApplicationSpec {
    /// Crea una aplicación desde un workload.
    pub fn new(workload: WorkloadSpec) -> Self {
        Self {
            workload,
            service: None,
        }
    }

    /// Asocia un servicio.
    pub fn with_service(mut self, service: ServiceSpec) -> Self {
        self.service = Some(service);
        self
    }
}

/// Acción que un reconciliador educativo sugeriría.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconciliationAction {
    /// Crear réplicas faltantes.
    ScaleUp(u8),
    /// Retirar réplicas sobrantes.
    ScaleDown(u8),
    /// Esperar a que réplicas existentes pasen readiness.
    WaitForReadiness(u8),
}

/// Hallazgo de contrato operativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClusterFinding {
    /// La imagen no incluye tag explícito o usa `latest`.
    NonReproducibleImage,
    /// El workload no declara réplicas.
    MissingReplicas,
    /// Falta readiness probe.
    MissingReadinessProbe,
    /// Falta liveness probe.
    MissingLivenessProbe,
    /// Falta declaración de recursos.
    MissingResourceSpec,
    /// El servicio apunta a otro workload.
    ServiceTargetsDifferentWorkload,
    /// El servicio apunta a un puerto que el contenedor no declara.
    ServiceTargetsUndeclaredPort(u16),
}

/// Resultado de comparar estado deseado contra estado observado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconciliationPlan {
    /// Acciones necesarias para acercar la realidad al estado deseado.
    pub actions: Vec<ReconciliationAction>,
    /// Hallazgos del contrato declarativo.
    pub findings: Vec<ClusterFinding>,
}

/// Compara la aplicación deseada contra el estado observado.
///
/// ```
/// use rust_devops::kubernetes::{
///     reconcile, ApplicationSpec, ObservedWorkload, ProbeSpec, ResourceSpec,
///     ServicePort, ServiceSpec, WorkloadSpec,
/// };
///
/// let workload = WorkloadSpec::new("api", "ghcr.io/jeresoftx/api:1.0.0", 3)
///     .exposing(8080)
///     .with_probes(ProbeSpec::new(true, true))
///     .with_resources(ResourceSpec::bounded(250, 256));
/// let app = ApplicationSpec::new(workload)
///     .with_service(ServiceSpec::new("api", "api").exposing(ServicePort::new(80, 8080)));
///
/// let plan = reconcile(&app, ObservedWorkload::new("api", 2, 2));
/// assert_eq!(plan.actions.len(), 2);
/// assert!(plan.findings.is_empty());
/// ```
pub fn reconcile(app: &ApplicationSpec, observed: ObservedWorkload) -> ReconciliationPlan {
    let mut actions = Vec::new();
    let mut findings = validate_application(app);

    if app.workload.desired_replicas == 0 {
        findings.push(ClusterFinding::MissingReplicas);
    } else if observed.current_replicas < app.workload.desired_replicas {
        actions.push(ReconciliationAction::ScaleUp(
            app.workload.desired_replicas - observed.current_replicas,
        ));
    } else if observed.current_replicas > app.workload.desired_replicas {
        actions.push(ReconciliationAction::ScaleDown(
            observed.current_replicas - app.workload.desired_replicas,
        ));
    }

    if observed.ready_replicas < app.workload.desired_replicas {
        actions.push(ReconciliationAction::WaitForReadiness(
            app.workload.desired_replicas - observed.ready_replicas,
        ));
    }

    ReconciliationPlan { actions, findings }
}

fn validate_application(app: &ApplicationSpec) -> Vec<ClusterFinding> {
    let mut findings = Vec::new();

    if image_is_not_reproducible(app.workload.image) {
        findings.push(ClusterFinding::NonReproducibleImage);
    }

    if !app.workload.probes.readiness {
        findings.push(ClusterFinding::MissingReadinessProbe);
    }

    if !app.workload.probes.liveness {
        findings.push(ClusterFinding::MissingLivenessProbe);
    }

    if app.workload.resources.is_none() {
        findings.push(ClusterFinding::MissingResourceSpec);
    }

    if let Some(service) = &app.service {
        if service.workload_name != app.workload.name {
            findings.push(ClusterFinding::ServiceTargetsDifferentWorkload);
        }

        for port in &service.ports {
            if !app.workload.container_ports.contains(&port.target_port) {
                findings.push(ClusterFinding::ServiceTargetsUndeclaredPort(
                    port.target_port,
                ));
            }
        }
    }

    findings
}

fn image_is_not_reproducible(image: &str) -> bool {
    match image.rsplit_once(':') {
        Some((_, tag)) => tag.is_empty() || tag == "latest",
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconciles_missing_replica_without_findings() {
        let workload = WorkloadSpec::new("api", "ghcr.io/jeresoftx/api:1.0.0", 3)
            .exposing(8080)
            .with_probes(ProbeSpec::new(true, true))
            .with_resources(ResourceSpec::bounded(250, 256));
        let app = ApplicationSpec::new(workload)
            .with_service(ServiceSpec::new("api", "api").exposing(ServicePort::new(80, 8080)));

        assert_eq!(
            reconcile(&app, ObservedWorkload::new("api", 2, 2)),
            ReconciliationPlan {
                actions: vec![
                    ReconciliationAction::ScaleUp(1),
                    ReconciliationAction::WaitForReadiness(1),
                ],
                findings: Vec::new(),
            }
        );
    }

    #[test]
    fn detects_unsafe_or_incomplete_spec() {
        let workload = WorkloadSpec::new("api", "ghcr.io/jeresoftx/api:latest", 0);
        let app = ApplicationSpec::new(workload).with_service(
            ServiceSpec::new("public-api", "worker").exposing(ServicePort::new(80, 8080)),
        );

        assert_eq!(
            reconcile(&app, ObservedWorkload::new("api", 1, 0)).findings,
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
}
