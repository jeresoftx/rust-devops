//! Modelo mínimo para razonar sobre pipelines de CI/CD como cadena de evidencia.
//!
//! El módulo no ejecuta una plataforma real. Representa eventos, etapas, gates,
//! artefactos y ambientes para enseñar qué debe probarse antes de promover un
//! cambio.

/// Evento que dispara un pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineTrigger {
    /// Pull request o merge request.
    PullRequest,
    /// Push directo o merge hacia la rama principal.
    PushToMain,
    /// Tag versionado.
    Tag,
    /// Promoción manual autorizada.
    ManualPromotion,
}

/// Ambiente al que apunta el pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Validación temprana o ambiente efímero.
    Development,
    /// Ambiente previo a producción.
    Staging,
    /// Ambiente productivo.
    Production,
}

/// Tipo de etapa dentro de un pipeline educativo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageKind {
    /// Formato automático o verificación de estilo.
    Format,
    /// Lint o análisis estático.
    Lint,
    /// Pruebas automatizadas.
    Test,
    /// Compilación o build.
    Build,
    /// Revisión mínima de seguridad.
    SecurityScan,
    /// Empaquetado de artefacto.
    Package,
    /// Aprobación humana o gate explícito.
    Approval,
    /// Despliegue o promoción hacia un ambiente.
    Deploy,
}

/// Resultado observado de una etapa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageStatus {
    /// La etapa terminó correctamente.
    Passed,
    /// La etapa falló.
    Failed,
    /// La etapa no corrió.
    Skipped,
}

/// Etapa ejecutada o declarada por el pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStage {
    /// Nombre visible de la etapa.
    pub name: &'static str,
    /// Tipo conceptual de la etapa.
    pub kind: StageKind,
    /// Si esta etapa bloquea promoción cuando falla.
    pub required: bool,
    /// Resultado actual.
    pub status: StageStatus,
    /// Duración observada en segundos.
    pub duration_seconds: u32,
}

impl PipelineStage {
    /// Declara un gate obligatorio aprobado.
    ///
    /// ```
    /// let stage = rust_devops::cicd::PipelineStage::required(
    ///     "cargo test",
    ///     rust_devops::cicd::StageKind::Test,
    /// );
    /// assert!(stage.required);
    /// ```
    pub fn required(name: &'static str, kind: StageKind) -> Self {
        Self {
            name,
            kind,
            required: true,
            status: StageStatus::Passed,
            duration_seconds: 0,
        }
    }

    /// Declara una etapa informativa aprobada.
    pub fn optional(name: &'static str, kind: StageKind) -> Self {
        Self {
            name,
            kind,
            required: false,
            status: StageStatus::Passed,
            duration_seconds: 0,
        }
    }

    /// Cambia el resultado observado de una etapa.
    pub fn with_status(mut self, status: StageStatus) -> Self {
        self.status = status;
        self
    }

    /// Declara duración observada.
    pub fn taking_seconds(mut self, duration_seconds: u32) -> Self {
        self.duration_seconds = duration_seconds;
        self
    }
}

/// Artefacto producido por el pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildArtifact {
    /// Nombre del artefacto.
    pub name: &'static str,
    /// Versión publicable.
    pub version: &'static str,
    /// Commit exacto que produjo el artefacto.
    pub commit_sha: &'static str,
}

impl BuildArtifact {
    /// Crea un artefacto trazable.
    pub fn new(name: &'static str, version: &'static str, commit_sha: &'static str) -> Self {
        Self {
            name,
            version,
            commit_sha,
        }
    }
}

/// Especificación observada de un pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineSpec {
    /// Evento de entrada.
    pub trigger: PipelineTrigger,
    /// Ambiente destino.
    pub target_environment: Environment,
    /// Etapas declaradas.
    pub stages: Vec<PipelineStage>,
    /// Artefacto producido, si aplica.
    pub artifact: Option<BuildArtifact>,
    /// Si existe salida documentada ante fallo de release.
    pub rollback_plan: bool,
}

impl PipelineSpec {
    /// Crea un pipeline mínimo para un ambiente.
    pub fn new(trigger: PipelineTrigger, target_environment: Environment) -> Self {
        Self {
            trigger,
            target_environment,
            stages: Vec::new(),
            artifact: None,
            rollback_plan: false,
        }
    }

    /// Agrega una etapa.
    pub fn with_stage(mut self, stage: PipelineStage) -> Self {
        self.stages.push(stage);
        self
    }

    /// Asocia el artefacto producido.
    pub fn with_artifact(mut self, artifact: BuildArtifact) -> Self {
        self.artifact = Some(artifact);
        self
    }

    /// Declara que existe plan de rollback o recuperación.
    pub fn with_rollback_plan(mut self) -> Self {
        self.rollback_plan = true;
        self
    }
}

/// Hallazgo operativo del pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineFinding {
    /// Falta una etapa obligatoria para el ambiente.
    MissingRequiredStage(StageKind),
    /// Un gate obligatorio falló.
    FailedRequiredStage(&'static str),
    /// Un gate obligatorio fue omitido.
    SkippedRequiredStage(&'static str),
    /// Falta artefacto para promover a un ambiente persistente.
    MissingArtifact,
    /// El artefacto no tiene versión y commit suficientes.
    NonTraceableArtifact,
    /// Producción requiere aprobación explícita.
    MissingProductionApproval,
    /// Producción requiere una salida documentada.
    MissingRollbackPlan,
}

/// Resultado de evaluar un pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineEvaluation {
    /// Si el pipeline puede promover el cambio.
    pub can_promote: bool,
    /// Hallazgos que deben resolverse antes de promover.
    pub findings: Vec<PipelineFinding>,
}

/// Evalúa si un pipeline contiene la evidencia mínima para promover.
///
/// ```
/// use rust_devops::cicd::{
///     evaluate_pipeline, Environment, PipelineSpec, PipelineStage,
///     PipelineTrigger, StageKind,
/// };
///
/// let pipeline = PipelineSpec::new(PipelineTrigger::PullRequest, Environment::Development)
///     .with_stage(PipelineStage::required("fmt", StageKind::Format))
///     .with_stage(PipelineStage::required("lint", StageKind::Lint))
///     .with_stage(PipelineStage::required("test", StageKind::Test))
///     .with_stage(PipelineStage::required("build", StageKind::Build));
///
/// assert!(evaluate_pipeline(&pipeline).can_promote);
/// ```
pub fn evaluate_pipeline(pipeline: &PipelineSpec) -> PipelineEvaluation {
    let mut findings = Vec::new();

    for &required_kind in required_stage_kinds(pipeline.target_environment) {
        if !pipeline
            .stages
            .iter()
            .any(|stage| stage.kind == required_kind)
        {
            findings.push(PipelineFinding::MissingRequiredStage(required_kind));
        }
    }

    for stage in pipeline.stages.iter().filter(|stage| stage.required) {
        match stage.status {
            StageStatus::Passed => {}
            StageStatus::Failed => findings.push(PipelineFinding::FailedRequiredStage(stage.name)),
            StageStatus::Skipped => {
                findings.push(PipelineFinding::SkippedRequiredStage(stage.name));
            }
        }
    }

    if requires_artifact(pipeline.target_environment) {
        match &pipeline.artifact {
            Some(artifact) if artifact_is_traceable(artifact) => {}
            Some(_) => findings.push(PipelineFinding::NonTraceableArtifact),
            None => findings.push(PipelineFinding::MissingArtifact),
        }
    }

    if pipeline.target_environment == Environment::Production {
        if !has_passed_required_stage(pipeline, StageKind::Approval) {
            findings.push(PipelineFinding::MissingProductionApproval);
        }

        if !pipeline.rollback_plan {
            findings.push(PipelineFinding::MissingRollbackPlan);
        }
    }

    PipelineEvaluation {
        can_promote: findings.is_empty(),
        findings,
    }
}

fn required_stage_kinds(environment: Environment) -> &'static [StageKind] {
    match environment {
        Environment::Development => &[
            StageKind::Format,
            StageKind::Lint,
            StageKind::Test,
            StageKind::Build,
        ],
        Environment::Staging => &[
            StageKind::Format,
            StageKind::Lint,
            StageKind::Test,
            StageKind::Build,
            StageKind::Package,
            StageKind::Deploy,
        ],
        Environment::Production => &[
            StageKind::Format,
            StageKind::Lint,
            StageKind::Test,
            StageKind::Build,
            StageKind::SecurityScan,
            StageKind::Package,
            StageKind::Approval,
            StageKind::Deploy,
        ],
    }
}

fn requires_artifact(environment: Environment) -> bool {
    matches!(environment, Environment::Staging | Environment::Production)
}

fn artifact_is_traceable(artifact: &BuildArtifact) -> bool {
    !artifact.name.is_empty()
        && !artifact.version.is_empty()
        && artifact.version != "latest"
        && artifact.commit_sha.len() >= 7
}

fn has_passed_required_stage(pipeline: &PipelineSpec, kind: StageKind) -> bool {
    pipeline
        .stages
        .iter()
        .any(|stage| stage.kind == kind && stage.required && stage.status == StageStatus::Passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_development_pipeline_with_core_gates() {
        let pipeline = PipelineSpec::new(PipelineTrigger::PullRequest, Environment::Development)
            .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
            .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
            .with_stage(PipelineStage::required("cargo test", StageKind::Test))
            .with_stage(PipelineStage::required("cargo build", StageKind::Build));

        assert_eq!(
            evaluate_pipeline(&pipeline),
            PipelineEvaluation {
                can_promote: true,
                findings: Vec::new(),
            }
        );
    }

    #[test]
    fn blocks_failed_required_gate() {
        let pipeline = PipelineSpec::new(PipelineTrigger::PushToMain, Environment::Staging)
            .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
            .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
            .with_stage(
                PipelineStage::required("cargo test", StageKind::Test)
                    .with_status(StageStatus::Failed),
            )
            .with_stage(PipelineStage::required("cargo build", StageKind::Build))
            .with_stage(PipelineStage::required("docker build", StageKind::Package))
            .with_stage(PipelineStage::required("deploy staging", StageKind::Deploy))
            .with_artifact(BuildArtifact::new("booking-api", "1.0.0", "abc1234"));

        assert!(
            evaluate_pipeline(&pipeline)
                .findings
                .contains(&PipelineFinding::FailedRequiredStage("cargo test"))
        );
    }

    #[test]
    fn detects_incomplete_production_pipeline() {
        let pipeline = PipelineSpec::new(PipelineTrigger::Tag, Environment::Production)
            .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
            .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
            .with_stage(PipelineStage::required("cargo test", StageKind::Test))
            .with_stage(PipelineStage::required("cargo build", StageKind::Build))
            .with_artifact(BuildArtifact::new("booking-api", "latest", "abc1234"));

        assert_eq!(
            evaluate_pipeline(&pipeline).findings,
            vec![
                PipelineFinding::MissingRequiredStage(StageKind::SecurityScan),
                PipelineFinding::MissingRequiredStage(StageKind::Package),
                PipelineFinding::MissingRequiredStage(StageKind::Approval),
                PipelineFinding::MissingRequiredStage(StageKind::Deploy),
                PipelineFinding::NonTraceableArtifact,
                PipelineFinding::MissingProductionApproval,
                PipelineFinding::MissingRollbackPlan,
            ]
        );
    }
}
