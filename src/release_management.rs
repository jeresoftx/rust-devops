//! Modelo mínimo para razonar sobre gestión de releases.
//!
//! El módulo no publica tags ni genera artefactos reales. Representa versión,
//! cambios, artefactos, comunicación y rollback para evaluar si un release está
//! listo para publicarse.

/// Tipo de incremento semántico esperado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionBump {
    /// Corrección compatible.
    Patch,
    /// Funcionalidad compatible.
    Minor,
    /// Cambio incompatible.
    Major,
}

/// Versión semántica mínima.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticVersion {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
    /// Patch version.
    pub patch: u16,
}

impl SemanticVersion {
    /// Crea una versión semántica.
    ///
    /// ```
    /// let version = rust_devops::release_management::SemanticVersion::new(1, 2, 3);
    /// assert_eq!(version.minor, 2);
    /// ```
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Calcula el tipo de incremento desde otra versión.
    pub fn bump_from(self, previous: Self) -> Option<VersionBump> {
        if self.major > previous.major {
            Some(VersionBump::Major)
        } else if self.major == previous.major && self.minor > previous.minor {
            Some(VersionBump::Minor)
        } else if self.major == previous.major
            && self.minor == previous.minor
            && self.patch > previous.patch
        {
            Some(VersionBump::Patch)
        } else {
            None
        }
    }
}

/// Cambio incluido en un release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// Corrección de bug.
    Fix,
    /// Nueva funcionalidad compatible.
    Feature,
    /// Cambio incompatible para consumidores.
    BreakingChange,
    /// Migración de datos o contrato operativo.
    Migration,
}

/// Artefacto asociado al release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifact {
    /// Nombre del artefacto.
    pub name: &'static str,
    /// Referencia al commit que lo produjo.
    pub commit_sha: &'static str,
    /// Tag o digest publicable.
    pub reference: &'static str,
}

impl ReleaseArtifact {
    /// Crea un artefacto trazable.
    pub fn new(name: &'static str, commit_sha: &'static str, reference: &'static str) -> Self {
        Self {
            name,
            commit_sha,
            reference,
        }
    }
}

/// Plan de release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasePlan {
    /// Versión previa conocida.
    pub previous_version: SemanticVersion,
    /// Versión que se publicará.
    pub next_version: SemanticVersion,
    /// Cambios incluidos.
    pub changes: Vec<ChangeKind>,
    /// Artefactos asociados.
    pub artifacts: Vec<ReleaseArtifact>,
    /// Si existen notas de release útiles.
    pub release_notes: bool,
    /// Si el changelog fue actualizado.
    pub changelog_updated: bool,
    /// Si existe rollback documentado.
    pub rollback_plan: bool,
    /// Si se declaró canal de comunicación.
    pub communication_channel: bool,
}

impl ReleasePlan {
    /// Crea un plan de release mínimo.
    pub fn new(previous_version: SemanticVersion, next_version: SemanticVersion) -> Self {
        Self {
            previous_version,
            next_version,
            changes: Vec::new(),
            artifacts: Vec::new(),
            release_notes: false,
            changelog_updated: false,
            rollback_plan: false,
            communication_channel: false,
        }
    }

    /// Agrega un cambio.
    pub fn with_change(mut self, change: ChangeKind) -> Self {
        self.changes.push(change);
        self
    }

    /// Agrega un artefacto.
    pub fn with_artifact(mut self, artifact: ReleaseArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    /// Declara notas de release.
    pub fn with_release_notes(mut self) -> Self {
        self.release_notes = true;
        self
    }

    /// Declara changelog actualizado.
    pub fn with_changelog(mut self) -> Self {
        self.changelog_updated = true;
        self
    }

    /// Declara rollback documentado.
    pub fn with_rollback_plan(mut self) -> Self {
        self.rollback_plan = true;
        self
    }

    /// Declara canal de comunicación.
    pub fn with_communication_channel(mut self) -> Self {
        self.communication_channel = true;
        self
    }
}

/// Hallazgo de riesgo en un release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseFinding {
    /// La versión no avanza.
    VersionDoesNotAdvance,
    /// El incremento semántico no corresponde al impacto.
    IncorrectVersionBump {
        /// Incremento esperado.
        expected: VersionBump,
        /// Incremento observado.
        actual: Option<VersionBump>,
    },
    /// No hay cambios declarados.
    MissingChanges,
    /// No hay artefactos.
    MissingArtifacts,
    /// Un artefacto no es trazable.
    NonTraceableArtifact(&'static str),
    /// Faltan notas de release.
    MissingReleaseNotes,
    /// Falta changelog.
    MissingChangelog,
    /// Falta rollback.
    MissingRollbackPlan,
    /// Falta canal de comunicación.
    MissingCommunicationChannel,
}

/// Resultado de evaluar un release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseEvaluation {
    /// Si el release está listo para publicarse.
    pub ready: bool,
    /// Hallazgos pendientes.
    pub findings: Vec<ReleaseFinding>,
}

/// Evalúa si un release tiene identidad, compatibilidad y comunicación.
///
/// ```
/// use rust_devops::release_management::{
///     evaluate_release, ChangeKind, ReleaseArtifact, ReleasePlan, SemanticVersion,
/// };
///
/// let release = ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
///     .with_change(ChangeKind::Feature)
///     .with_artifact(ReleaseArtifact::new("api", "abc1234", "v1.1.0"))
///     .with_release_notes()
///     .with_changelog()
///     .with_rollback_plan()
///     .with_communication_channel();
///
/// assert!(evaluate_release(&release).ready);
/// ```
pub fn evaluate_release(release: &ReleasePlan) -> ReleaseEvaluation {
    let mut findings = Vec::new();
    let actual_bump = release.next_version.bump_from(release.previous_version);

    if actual_bump.is_none() {
        findings.push(ReleaseFinding::VersionDoesNotAdvance);
    }

    if release.changes.is_empty() {
        findings.push(ReleaseFinding::MissingChanges);
    } else if let Some(expected) = expected_bump(&release.changes)
        && actual_bump != Some(expected)
    {
        findings.push(ReleaseFinding::IncorrectVersionBump {
            expected,
            actual: actual_bump,
        });
    }

    if release.artifacts.is_empty() {
        findings.push(ReleaseFinding::MissingArtifacts);
    }

    for artifact in &release.artifacts {
        if !artifact_is_traceable(artifact) {
            findings.push(ReleaseFinding::NonTraceableArtifact(artifact.name));
        }
    }

    if !release.release_notes {
        findings.push(ReleaseFinding::MissingReleaseNotes);
    }

    if !release.changelog_updated {
        findings.push(ReleaseFinding::MissingChangelog);
    }

    if !release.rollback_plan {
        findings.push(ReleaseFinding::MissingRollbackPlan);
    }

    if !release.communication_channel {
        findings.push(ReleaseFinding::MissingCommunicationChannel);
    }

    ReleaseEvaluation {
        ready: findings.is_empty(),
        findings,
    }
}

fn expected_bump(changes: &[ChangeKind]) -> Option<VersionBump> {
    if changes
        .iter()
        .any(|change| matches!(change, ChangeKind::BreakingChange | ChangeKind::Migration))
    {
        Some(VersionBump::Major)
    } else if changes
        .iter()
        .any(|change| matches!(change, ChangeKind::Feature))
    {
        Some(VersionBump::Minor)
    } else if changes
        .iter()
        .any(|change| matches!(change, ChangeKind::Fix))
    {
        Some(VersionBump::Patch)
    } else {
        None
    }
}

fn artifact_is_traceable(artifact: &ReleaseArtifact) -> bool {
    !artifact.name.is_empty() && artifact.commit_sha.len() >= 7 && !artifact.reference.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_well_documented_minor_release() {
        let release =
            ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
                .with_change(ChangeKind::Feature)
                .with_artifact(ReleaseArtifact::new("booking-api", "abc1234", "v1.1.0"))
                .with_release_notes()
                .with_changelog()
                .with_rollback_plan()
                .with_communication_channel();

        assert_eq!(
            evaluate_release(&release),
            ReleaseEvaluation {
                ready: true,
                findings: Vec::new(),
            }
        );
    }

    #[test]
    fn detects_breaking_change_with_minor_bump() {
        let release =
            ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
                .with_change(ChangeKind::BreakingChange)
                .with_artifact(ReleaseArtifact::new("booking-api", "abc1234", "v1.1.0"))
                .with_release_notes()
                .with_changelog()
                .with_rollback_plan()
                .with_communication_channel();

        assert_eq!(
            evaluate_release(&release).findings,
            vec![ReleaseFinding::IncorrectVersionBump {
                expected: VersionBump::Major,
                actual: Some(VersionBump::Minor),
            }]
        );
    }

    #[test]
    fn detects_uncommunicated_untraceable_release() {
        let release =
            ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 0, 0))
                .with_change(ChangeKind::Fix)
                .with_artifact(ReleaseArtifact::new("booking-api", "abc", ""));

        assert_eq!(
            evaluate_release(&release).findings,
            vec![
                ReleaseFinding::VersionDoesNotAdvance,
                ReleaseFinding::IncorrectVersionBump {
                    expected: VersionBump::Patch,
                    actual: None,
                },
                ReleaseFinding::NonTraceableArtifact("booking-api"),
                ReleaseFinding::MissingReleaseNotes,
                ReleaseFinding::MissingChangelog,
                ReleaseFinding::MissingRollbackPlan,
                ReleaseFinding::MissingCommunicationChannel,
            ]
        );
    }
}
