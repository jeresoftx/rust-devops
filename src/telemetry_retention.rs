//! Modelo mínimo para razonar sobre retención de telemetría.
//!
//! El módulo no consulta proveedores ni calcula facturas reales. Representa
//! políticas de retención para validar si una señal tiene propósito, dueño,
//! ciclo de vida, revisión y tratamiento razonable de sensibilidad y costo.

/// Tipo de señal operativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetrySignalKind {
    /// Serie temporal agregada.
    Metric,
    /// Evento textual o estructurado.
    Log,
    /// Recorrido de una petición entre componentes.
    Trace,
}

/// Sensibilidad de la telemetría.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSensitivity {
    /// Información pública o sintética.
    Public,
    /// Información interna sin datos personales o regulados.
    Internal,
    /// Información sensible que exige minimización o redacción.
    Sensitive,
    /// Información con obligaciones regulatorias explícitas.
    Regulated,
}

/// Propósito principal de retener una señal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionPurpose {
    /// Investigación de incidentes recientes.
    IncidentInvestigation,
    /// Tendencias operativas o de capacidad.
    TrendAnalysis,
    /// Evidencia para auditoría interna.
    Audit,
    /// Conservación por obligación normativa.
    Compliance,
}

/// Ciclo de vida por niveles de acceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetentionTier {
    /// Días de acceso rápido.
    pub hot_days: u16,
    /// Días de acceso intermedio.
    pub warm_days: u16,
    /// Días de archivo frío.
    pub cold_days: u16,
}

impl RetentionTier {
    /// Crea una política por niveles.
    pub fn new(hot_days: u16, warm_days: u16, cold_days: u16) -> Self {
        Self {
            hot_days,
            warm_days,
            cold_days,
        }
    }

    /// Total de días retenidos entre hot, warm y cold.
    pub fn total_days(&self) -> u32 {
        u32::from(self.hot_days) + u32::from(self.warm_days) + u32::from(self.cold_days)
    }
}

/// Política de retención para una señal.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryRetentionPolicy {
    /// Nombre estable de la política.
    pub name: &'static str,
    /// Tipo de telemetría.
    pub signal_kind: TelemetrySignalKind,
    /// Sensibilidad declarada.
    pub sensitivity: DataSensitivity,
    /// Propósito de conservación.
    pub purpose: Option<RetentionPurpose>,
    /// Responsable operativo.
    pub owner: &'static str,
    /// Ciclo de vida por niveles.
    pub tier: RetentionTier,
    /// Volumen estimado de ingesta por día.
    pub estimated_gib_per_day: f64,
    /// Si se redactan o minimizan campos sensibles.
    pub redacts_sensitive_fields: bool,
    /// Cada cuántos días se revisa la política.
    pub review_interval_days: u16,
}

impl TelemetryRetentionPolicy {
    /// Crea una política mínima sin propósito ni dueño.
    ///
    /// ```
    /// let policy = rust_devops::telemetry_retention::TelemetryRetentionPolicy::new(
    ///     "checkout_metrics",
    ///     rust_devops::telemetry_retention::TelemetrySignalKind::Metric,
    /// );
    ///
    /// assert_eq!(policy.name, "checkout_metrics");
    /// ```
    pub fn new(name: &'static str, signal_kind: TelemetrySignalKind) -> Self {
        Self {
            name,
            signal_kind,
            sensitivity: DataSensitivity::Internal,
            purpose: None,
            owner: "",
            tier: RetentionTier::new(0, 0, 0),
            estimated_gib_per_day: 0.0,
            redacts_sensitive_fields: false,
            review_interval_days: 0,
        }
    }

    /// Declara sensibilidad.
    pub fn with_sensitivity(mut self, sensitivity: DataSensitivity) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Declara propósito.
    pub fn for_purpose(mut self, purpose: RetentionPurpose) -> Self {
        self.purpose = Some(purpose);
        self
    }

    /// Declara responsable.
    pub fn owned_by(mut self, owner: &'static str) -> Self {
        self.owner = owner;
        self
    }

    /// Declara ciclo de vida.
    pub fn retained_as(mut self, tier: RetentionTier) -> Self {
        self.tier = tier;
        self
    }

    /// Declara volumen estimado.
    pub fn ingesting_gib_per_day(mut self, estimated_gib_per_day: f64) -> Self {
        self.estimated_gib_per_day = estimated_gib_per_day;
        self
    }

    /// Marca redacción o minimización de datos sensibles.
    pub fn redacting_sensitive_fields(mut self) -> Self {
        self.redacts_sensitive_fields = true;
        self
    }

    /// Declara cadencia de revisión.
    pub fn reviewed_every(mut self, review_interval_days: u16) -> Self {
        self.review_interval_days = review_interval_days;
        self
    }
}

/// Hallazgo de riesgo en una política de retención.
#[derive(Debug, Clone, PartialEq)]
pub enum TelemetryRetentionFinding {
    /// Falta nombre de política.
    MissingPolicyName,
    /// Falta dueño.
    MissingOwner,
    /// Falta propósito.
    MissingPurpose,
    /// No hay ningún día de retención.
    MissingRetention,
    /// El volumen estimado es inválido.
    InvalidEstimatedVolume,
    /// Datos sensibles sin redacción o minimización.
    SensitiveDataWithoutRedaction,
    /// Datos regulados sin archivo frío.
    RegulatedDataWithoutColdArchive,
    /// Falta cadencia de revisión.
    MissingReviewCadence,
    /// Retención hot excesiva para investigación cotidiana.
    ExcessiveHotRetention {
        /// Días hot declarados.
        hot_days: u16,
    },
}

/// Resultado de evaluar una política de retención.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryRetentionEvaluation {
    /// Total de días retenidos.
    pub total_retention_days: u32,
    /// Volumen estimado de datos hot.
    pub estimated_hot_gib: Option<f64>,
    /// Volumen estimado de todo el ciclo de vida.
    pub estimated_total_gib: Option<f64>,
    /// Si la política conserva invariantes operativas.
    pub ready: bool,
    /// Hallazgos encontrados.
    pub findings: Vec<TelemetryRetentionFinding>,
}

/// Evalúa una política de retención de telemetría.
///
/// ```
/// use rust_devops::telemetry_retention::{
///     evaluate_retention, RetentionPurpose, RetentionTier,
///     TelemetryRetentionPolicy, TelemetrySignalKind,
/// };
///
/// let policy = TelemetryRetentionPolicy::new("checkout_metrics", TelemetrySignalKind::Metric)
///     .for_purpose(RetentionPurpose::TrendAnalysis)
///     .owned_by("platform-observability")
///     .retained_as(RetentionTier::new(14, 46, 305))
///     .ingesting_gib_per_day(1.5)
///     .reviewed_every(90);
///
/// assert!(evaluate_retention(&policy).ready);
/// ```
pub fn evaluate_retention(policy: &TelemetryRetentionPolicy) -> TelemetryRetentionEvaluation {
    let mut findings = Vec::new();

    if policy.name.is_empty() {
        findings.push(TelemetryRetentionFinding::MissingPolicyName);
    }

    if policy.owner.is_empty() {
        findings.push(TelemetryRetentionFinding::MissingOwner);
    }

    if policy.purpose.is_none() {
        findings.push(TelemetryRetentionFinding::MissingPurpose);
    }

    if policy.tier.total_days() == 0 {
        findings.push(TelemetryRetentionFinding::MissingRetention);
    }

    if policy.estimated_gib_per_day <= 0.0 || !policy.estimated_gib_per_day.is_finite() {
        findings.push(TelemetryRetentionFinding::InvalidEstimatedVolume);
    }

    if matches!(
        policy.sensitivity,
        DataSensitivity::Sensitive | DataSensitivity::Regulated
    ) && !policy.redacts_sensitive_fields
    {
        findings.push(TelemetryRetentionFinding::SensitiveDataWithoutRedaction);
    }

    if policy.sensitivity == DataSensitivity::Regulated && policy.tier.cold_days == 0 {
        findings.push(TelemetryRetentionFinding::RegulatedDataWithoutColdArchive);
    }

    if policy.review_interval_days == 0 {
        findings.push(TelemetryRetentionFinding::MissingReviewCadence);
    }

    if policy.tier.hot_days > 30 {
        findings.push(TelemetryRetentionFinding::ExcessiveHotRetention {
            hot_days: policy.tier.hot_days,
        });
    }

    let total_retention_days = policy.tier.total_days();
    let estimated_hot_gib =
        if policy.estimated_gib_per_day > 0.0 && policy.estimated_gib_per_day.is_finite() {
            Some(policy.estimated_gib_per_day * f64::from(policy.tier.hot_days))
        } else {
            None
        };
    let estimated_total_gib =
        if policy.estimated_gib_per_day > 0.0 && policy.estimated_gib_per_day.is_finite() {
            Some(policy.estimated_gib_per_day * total_retention_days as f64)
        } else {
            None
        };

    TelemetryRetentionEvaluation {
        total_retention_days,
        estimated_hot_gib,
        estimated_total_gib,
        ready: findings.is_empty(),
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_purposeful_metric_retention() {
        let policy = TelemetryRetentionPolicy::new("checkout_metrics", TelemetrySignalKind::Metric)
            .for_purpose(RetentionPurpose::TrendAnalysis)
            .owned_by("platform-observability")
            .retained_as(RetentionTier::new(14, 46, 305))
            .ingesting_gib_per_day(1.5)
            .reviewed_every(90);

        let evaluation = evaluate_retention(&policy);

        assert!(evaluation.ready);
        assert!(evaluation.findings.is_empty());
        assert_eq!(evaluation.total_retention_days, 365);
        assert_eq!(evaluation.estimated_hot_gib, Some(21.0));
    }

    #[test]
    fn detects_sensitive_logs_without_redaction() {
        let policy = TelemetryRetentionPolicy::new("checkout_logs", TelemetrySignalKind::Log)
            .with_sensitivity(DataSensitivity::Sensitive)
            .for_purpose(RetentionPurpose::IncidentInvestigation)
            .owned_by("payments-oncall")
            .retained_as(RetentionTier::new(7, 23, 0))
            .ingesting_gib_per_day(8.0)
            .reviewed_every(30);

        let evaluation = evaluate_retention(&policy);

        assert!(!evaluation.ready);
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::SensitiveDataWithoutRedaction)
        );
    }

    #[test]
    fn flags_policy_without_owner_purpose_or_review() {
        let policy = TelemetryRetentionPolicy::new("", TelemetrySignalKind::Trace)
            .retained_as(RetentionTier::new(3, 0, 0))
            .ingesting_gib_per_day(2.0);

        let evaluation = evaluate_retention(&policy);

        assert!(!evaluation.ready);
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::MissingPolicyName)
        );
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::MissingOwner)
        );
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::MissingPurpose)
        );
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::MissingReviewCadence)
        );
    }

    #[test]
    fn detects_invalid_volume_and_regulated_data_without_archive() {
        let policy = TelemetryRetentionPolicy::new("audit_logs", TelemetrySignalKind::Log)
            .with_sensitivity(DataSensitivity::Regulated)
            .for_purpose(RetentionPurpose::Compliance)
            .owned_by("security")
            .retained_as(RetentionTier::new(45, 0, 0))
            .reviewed_every(180);

        let evaluation = evaluate_retention(&policy);

        assert!(!evaluation.ready);
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::InvalidEstimatedVolume)
        );
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::SensitiveDataWithoutRedaction)
        );
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::RegulatedDataWithoutColdArchive)
        );
        assert!(
            evaluation
                .findings
                .contains(&TelemetryRetentionFinding::ExcessiveHotRetention { hot_days: 45 })
        );
    }
}
