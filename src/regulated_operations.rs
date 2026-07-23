//! Modelo mínimo para razonar sobre operación en dominios regulados.
//!
//! El módulo no representa una regulación específica. Modela eventos auditables
//! y controles operativos para validar identidad, autorización, alcance,
//! evidencia, privacidad y revisión posterior.

/// Ambiente donde ocurre una acción operativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Ambiente de desarrollo.
    Development,
    /// Ambiente de pruebas.
    Testing,
    /// Ambiente previo a producción.
    Staging,
    /// Ambiente productivo.
    Production,
}

/// Clasificación del dato afectado por una acción.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataClassification {
    /// Datos públicos o sintéticos.
    Public,
    /// Datos internos sin sensibilidad especial.
    Internal,
    /// Datos sensibles que requieren minimización.
    Sensitive,
    /// Datos sujetos a controles regulatorios.
    Regulated,
}

/// Tipo de autorización que respalda una acción.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationKind {
    /// Aprobación humana explícita.
    HumanApproval,
    /// Automatización previamente aprobada.
    ApprovedAutomation,
    /// Acción de emergencia.
    Emergency,
    /// Excepción temporal.
    TemporaryException,
}

/// Evento auditable de una operación regulada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegulatedOperationEvent {
    /// Identificador estable del evento.
    pub id: &'static str,
    /// Actor humano o sistema responsable.
    pub actor: &'static str,
    /// Acción realizada.
    pub action: &'static str,
    /// Recurso afectado.
    pub resource: &'static str,
    /// Ambiente donde ocurrió.
    pub environment: Environment,
    /// Clasificación de datos potencialmente afectados.
    pub data_classification: DataClassification,
    /// Autorización asociada.
    pub authorization: Option<AuthorizationKind>,
    /// Identificador de correlación con cambio, incidente o solicitud.
    pub correlation_id: &'static str,
    /// Evidencia disponible para auditoría.
    pub evidence_uri: &'static str,
    /// Si minimiza o redacta datos sensibles.
    pub minimizes_sensitive_data: bool,
    /// Si requiere revisión posterior.
    pub post_review_required: bool,
    /// Días de retención de evidencia.
    pub evidence_retention_days: u16,
}

impl RegulatedOperationEvent {
    /// Crea un evento auditable mínimo.
    ///
    /// ```
    /// let event = rust_devops::regulated_operations::RegulatedOperationEvent::new(
    ///     "evt-1",
    ///     "release-bot",
    ///     "deploy",
    ///     "payments-api",
    /// );
    ///
    /// assert_eq!(event.id, "evt-1");
    /// ```
    pub fn new(
        id: &'static str,
        actor: &'static str,
        action: &'static str,
        resource: &'static str,
    ) -> Self {
        Self {
            id,
            actor,
            action,
            resource,
            environment: Environment::Development,
            data_classification: DataClassification::Internal,
            authorization: None,
            correlation_id: "",
            evidence_uri: "",
            minimizes_sensitive_data: false,
            post_review_required: false,
            evidence_retention_days: 0,
        }
    }

    /// Declara ambiente.
    pub fn in_environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Declara clasificación de datos.
    pub fn with_data_classification(mut self, data_classification: DataClassification) -> Self {
        self.data_classification = data_classification;
        self
    }

    /// Declara autorización.
    pub fn authorized_by(mut self, authorization: AuthorizationKind) -> Self {
        self.authorization = Some(authorization);
        self
    }

    /// Declara correlación.
    pub fn correlated_with(mut self, correlation_id: &'static str) -> Self {
        self.correlation_id = correlation_id;
        self
    }

    /// Declara evidencia.
    pub fn evidenced_by(mut self, evidence_uri: &'static str) -> Self {
        self.evidence_uri = evidence_uri;
        self
    }

    /// Marca minimización o redacción de datos sensibles.
    pub fn minimizing_sensitive_data(mut self) -> Self {
        self.minimizes_sensitive_data = true;
        self
    }

    /// Marca que requiere revisión posterior.
    pub fn requiring_post_review(mut self) -> Self {
        self.post_review_required = true;
        self
    }

    /// Declara retención de evidencia.
    pub fn retaining_evidence_for(mut self, evidence_retention_days: u16) -> Self {
        self.evidence_retention_days = evidence_retention_days;
        self
    }
}

/// Hallazgo en una operación regulada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegulatedOperationFinding {
    /// Falta identificador.
    MissingEventId,
    /// Falta actor.
    MissingActor,
    /// Falta acción.
    MissingAction,
    /// Falta recurso.
    MissingResource,
    /// Producción sin autorización explícita.
    ProductionWithoutAuthorization,
    /// Falta correlación con cambio, incidente o solicitud.
    MissingCorrelation,
    /// Falta evidencia de auditoría.
    MissingEvidence,
    /// Datos sensibles o regulados sin minimización.
    SensitiveDataWithoutMinimization,
    /// Emergencia o excepción sin revisión posterior.
    ExceptionWithoutPostReview,
    /// Falta retención de evidencia.
    MissingEvidenceRetention,
}

/// Resultado de evaluar una operación regulada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegulatedOperationEvaluation {
    /// Si la operación conserva invariantes mínimas.
    pub auditable: bool,
    /// Si ocurrió en producción.
    pub production_scope: bool,
    /// Si involucra datos sensibles o regulados.
    pub sensitive_scope: bool,
    /// Hallazgos encontrados.
    pub findings: Vec<RegulatedOperationFinding>,
}

/// Evalúa un evento auditable.
///
/// ```
/// use rust_devops::regulated_operations::{
///     evaluate_regulated_operation, AuthorizationKind, DataClassification,
///     Environment, RegulatedOperationEvent,
/// };
///
/// let event = RegulatedOperationEvent::new("evt-1", "release-bot", "deploy", "payments-api")
///     .in_environment(Environment::Production)
///     .with_data_classification(DataClassification::Sensitive)
///     .authorized_by(AuthorizationKind::HumanApproval)
///     .correlated_with("change-123")
///     .evidenced_by("audit://change-123")
///     .minimizing_sensitive_data()
///     .retaining_evidence_for(365);
///
/// assert!(evaluate_regulated_operation(&event).auditable);
/// ```
pub fn evaluate_regulated_operation(
    event: &RegulatedOperationEvent,
) -> RegulatedOperationEvaluation {
    let mut findings = Vec::new();

    if event.id.is_empty() {
        findings.push(RegulatedOperationFinding::MissingEventId);
    }

    if event.actor.is_empty() {
        findings.push(RegulatedOperationFinding::MissingActor);
    }

    if event.action.is_empty() {
        findings.push(RegulatedOperationFinding::MissingAction);
    }

    if event.resource.is_empty() {
        findings.push(RegulatedOperationFinding::MissingResource);
    }

    let production_scope = event.environment == Environment::Production;
    if production_scope && event.authorization.is_none() {
        findings.push(RegulatedOperationFinding::ProductionWithoutAuthorization);
    }

    if event.correlation_id.is_empty() {
        findings.push(RegulatedOperationFinding::MissingCorrelation);
    }

    if event.evidence_uri.is_empty() {
        findings.push(RegulatedOperationFinding::MissingEvidence);
    }

    let sensitive_scope = matches!(
        event.data_classification,
        DataClassification::Sensitive | DataClassification::Regulated
    );
    if sensitive_scope && !event.minimizes_sensitive_data {
        findings.push(RegulatedOperationFinding::SensitiveDataWithoutMinimization);
    }

    if matches!(
        event.authorization,
        Some(AuthorizationKind::Emergency | AuthorizationKind::TemporaryException)
    ) && !event.post_review_required
    {
        findings.push(RegulatedOperationFinding::ExceptionWithoutPostReview);
    }

    if event.evidence_retention_days == 0 {
        findings.push(RegulatedOperationFinding::MissingEvidenceRetention);
    }

    RegulatedOperationEvaluation {
        auditable: findings.is_empty(),
        production_scope,
        sensitive_scope,
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_auditable_production_change() {
        let event = RegulatedOperationEvent::new("evt-1", "release-bot", "deploy", "payments-api")
            .in_environment(Environment::Production)
            .with_data_classification(DataClassification::Sensitive)
            .authorized_by(AuthorizationKind::HumanApproval)
            .correlated_with("change-123")
            .evidenced_by("audit://change-123")
            .minimizing_sensitive_data()
            .retaining_evidence_for(365);

        let evaluation = evaluate_regulated_operation(&event);

        assert!(evaluation.auditable);
        assert!(evaluation.production_scope);
        assert!(evaluation.sensitive_scope);
        assert!(evaluation.findings.is_empty());
    }

    #[test]
    fn detects_production_change_without_authorization() {
        let event = RegulatedOperationEvent::new("evt-2", "release-bot", "deploy", "payments-api")
            .in_environment(Environment::Production)
            .with_data_classification(DataClassification::Internal)
            .correlated_with("change-124")
            .evidenced_by("audit://change-124")
            .retaining_evidence_for(180);

        let evaluation = evaluate_regulated_operation(&event);

        assert!(!evaluation.auditable);
        assert!(
            evaluation
                .findings
                .contains(&RegulatedOperationFinding::ProductionWithoutAuthorization)
        );
    }

    #[test]
    fn detects_emergency_without_post_review() {
        let event = RegulatedOperationEvent::new("evt-3", "oncall", "rotate_secret", "vault")
            .in_environment(Environment::Production)
            .with_data_classification(DataClassification::Sensitive)
            .authorized_by(AuthorizationKind::Emergency)
            .correlated_with("incident-77")
            .evidenced_by("audit://incident-77")
            .minimizing_sensitive_data()
            .retaining_evidence_for(365);

        let evaluation = evaluate_regulated_operation(&event);

        assert!(!evaluation.auditable);
        assert!(
            evaluation
                .findings
                .contains(&RegulatedOperationFinding::ExceptionWithoutPostReview)
        );
    }

    #[test]
    fn detects_regulated_data_without_minimization_or_retention() {
        let event = RegulatedOperationEvent::new("", "", "", "")
            .in_environment(Environment::Staging)
            .with_data_classification(DataClassification::Regulated)
            .authorized_by(AuthorizationKind::ApprovedAutomation);

        let evaluation = evaluate_regulated_operation(&event);

        assert!(!evaluation.auditable);
        assert!(
            evaluation
                .findings
                .contains(&RegulatedOperationFinding::MissingEventId)
        );
        assert!(
            evaluation
                .findings
                .contains(&RegulatedOperationFinding::MissingActor)
        );
        assert!(
            evaluation
                .findings
                .contains(&RegulatedOperationFinding::SensitiveDataWithoutMinimization)
        );
        assert!(
            evaluation
                .findings
                .contains(&RegulatedOperationFinding::MissingEvidenceRetention)
        );
    }
}
