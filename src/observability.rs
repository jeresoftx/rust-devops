//! Modelo mínimo para razonar sobre observabilidad.
//!
//! El módulo no recolecta telemetría real. Representa señales, contexto,
//! retención y acción operativa para evaluar si un sistema puede explicar su
//! comportamiento en producción.

/// Pregunta operativa que una estrategia de observabilidad debe responder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalQuestion {
    /// ¿Qué pasó durante un incidente?
    IncidentDiagnosis,
    /// ¿La versión nueva se comporta correctamente?
    ReleaseHealth,
    /// ¿A quién afectó la degradación?
    UserImpact,
    /// ¿Dónde apareció una regresión de rendimiento?
    PerformanceRegression,
}

/// Tipo de señal disponible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalKind {
    /// Evento textual o estructurado.
    Log,
    /// Valor agregado en el tiempo.
    Metric,
    /// Camino de una petición entre componentes.
    Trace,
    /// Evento de dominio u operación.
    Event,
}

/// Acción habilitada por una señal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalAction {
    /// Continuar con el cambio.
    Continue,
    /// Investigar antes de avanzar.
    Investigate,
    /// Pausar exposición o promoción.
    Pause,
    /// Regresar a una versión segura.
    RollBack,
    /// Escalar con otro equipo o responsable.
    Escalate,
}

/// Señal de telemetría declarada por un servicio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetrySignal {
    /// Tipo de señal.
    pub kind: SignalKind,
    /// Nombre estable de la señal.
    pub name: &'static str,
    /// Servicio que emite la señal.
    pub service: &'static str,
    /// Ambiente donde aplica.
    pub environment: &'static str,
    /// Versión, release o deployment asociado.
    pub version: Option<&'static str>,
    /// Si la señal puede correlacionarse con petición, release o entidad.
    pub correlation_key: bool,
    /// Si la señal es estructurada.
    pub structured: bool,
    /// Retención en días.
    pub retained_days: u16,
    /// Si la señal tiene cardinalidad riesgosa.
    pub high_cardinality: bool,
    /// Acción que la señal permite tomar.
    pub action: Option<OperationalAction>,
}

impl TelemetrySignal {
    /// Crea una señal mínima.
    ///
    /// ```
    /// let signal = rust_devops::observability::TelemetrySignal::new(
    ///     rust_devops::observability::SignalKind::Metric,
    ///     "http_request_duration_seconds",
    /// );
    ///
    /// assert_eq!(signal.name, "http_request_duration_seconds");
    /// ```
    pub fn new(kind: SignalKind, name: &'static str) -> Self {
        Self {
            kind,
            name,
            service: "",
            environment: "",
            version: None,
            correlation_key: false,
            structured: false,
            retained_days: 0,
            high_cardinality: false,
            action: None,
        }
    }

    /// Declara servicio y ambiente.
    pub fn emitted_by(mut self, service: &'static str, environment: &'static str) -> Self {
        self.service = service;
        self.environment = environment;
        self
    }

    /// Declara versión asociada.
    pub fn for_version(mut self, version: &'static str) -> Self {
        self.version = Some(version);
        self
    }

    /// Marca la señal como correlacionable.
    pub fn with_correlation_key(mut self) -> Self {
        self.correlation_key = true;
        self
    }

    /// Marca la señal como estructurada.
    pub fn structured(mut self) -> Self {
        self.structured = true;
        self
    }

    /// Declara retención en días.
    pub fn retained_for(mut self, days: u16) -> Self {
        self.retained_days = days;
        self
    }

    /// Marca cardinalidad riesgosa.
    pub fn with_high_cardinality(mut self) -> Self {
        self.high_cardinality = true;
        self
    }

    /// Declara acción operativa.
    pub fn enabling(mut self, action: OperationalAction) -> Self {
        self.action = Some(action);
        self
    }
}

/// Plan de observabilidad para una pregunta concreta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservabilityPlan {
    /// Pregunta que debe responderse.
    pub question: OperationalQuestion,
    /// Señales disponibles.
    pub signals: Vec<TelemetrySignal>,
}

impl ObservabilityPlan {
    /// Crea un plan sin señales.
    pub fn new(question: OperationalQuestion) -> Self {
        Self {
            question,
            signals: Vec::new(),
        }
    }

    /// Agrega una señal al plan.
    pub fn observing(mut self, signal: TelemetrySignal) -> Self {
        self.signals.push(signal);
        self
    }
}

/// Hallazgo de riesgo en una estrategia de observabilidad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservabilityFinding {
    /// No hay señales declaradas.
    MissingSignals,
    /// Falta al menos un log.
    MissingLogs,
    /// Falta al menos una métrica.
    MissingMetrics,
    /// Falta al menos una traza.
    MissingTraces,
    /// Falta servicio emisor.
    MissingServiceContext(&'static str),
    /// Falta ambiente.
    MissingEnvironmentContext(&'static str),
    /// Falta versión o release.
    MissingVersionContext(&'static str),
    /// Falta una llave para correlacionar señales.
    MissingCorrelation(&'static str),
    /// Un log no es estructurado.
    UnstructuredLog(&'static str),
    /// Falta retención.
    MissingRetention(&'static str),
    /// Una métrica tiene cardinalidad riesgosa.
    HighCardinalityMetric(&'static str),
    /// La señal no habilita una acción.
    MissingAction(&'static str),
}

/// Resultado de evaluar un plan de observabilidad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservabilityEvaluation {
    /// Si el plan tiene señales suficientes para operar.
    pub observable: bool,
    /// Hallazgos pendientes.
    pub findings: Vec<ObservabilityFinding>,
}

/// Evalúa si un plan de observabilidad tiene señales, contexto y acción.
///
/// ```
/// use rust_devops::observability::{
///     evaluate_observability, ObservabilityPlan, OperationalAction,
///     OperationalQuestion, SignalKind, TelemetrySignal,
/// };
///
/// let plan = ObservabilityPlan::new(OperationalQuestion::ReleaseHealth)
///     .observing(
///         TelemetrySignal::new(SignalKind::Log, "checkout.error")
///             .emitted_by("checkout-api", "production")
///             .for_version("v1.2.0")
///             .with_correlation_key()
///             .structured()
///             .retained_for(30)
///             .enabling(OperationalAction::Investigate),
///     )
///     .observing(
///         TelemetrySignal::new(SignalKind::Metric, "http_error_rate")
///             .emitted_by("checkout-api", "production")
///             .for_version("v1.2.0")
///             .with_correlation_key()
///             .structured()
///             .retained_for(30)
///             .enabling(OperationalAction::Pause),
///     )
///     .observing(
///         TelemetrySignal::new(SignalKind::Trace, "checkout.request")
///             .emitted_by("checkout-api", "production")
///             .for_version("v1.2.0")
///             .with_correlation_key()
///             .structured()
///             .retained_for(7)
///             .enabling(OperationalAction::RollBack),
///     );
///
/// assert!(evaluate_observability(&plan).observable);
/// ```
pub fn evaluate_observability(plan: &ObservabilityPlan) -> ObservabilityEvaluation {
    let mut findings = Vec::new();

    if plan.signals.is_empty() {
        findings.push(ObservabilityFinding::MissingSignals);
    }

    if !has_signal(plan, SignalKind::Log) {
        findings.push(ObservabilityFinding::MissingLogs);
    }

    if !has_signal(plan, SignalKind::Metric) {
        findings.push(ObservabilityFinding::MissingMetrics);
    }

    if !has_signal(plan, SignalKind::Trace) {
        findings.push(ObservabilityFinding::MissingTraces);
    }

    for signal in &plan.signals {
        if signal.service.is_empty() {
            findings.push(ObservabilityFinding::MissingServiceContext(signal.name));
        }

        if signal.environment.is_empty() {
            findings.push(ObservabilityFinding::MissingEnvironmentContext(signal.name));
        }

        if signal.version.is_none() {
            findings.push(ObservabilityFinding::MissingVersionContext(signal.name));
        }

        if !signal.correlation_key {
            findings.push(ObservabilityFinding::MissingCorrelation(signal.name));
        }

        if signal.kind == SignalKind::Log && !signal.structured {
            findings.push(ObservabilityFinding::UnstructuredLog(signal.name));
        }

        if signal.retained_days == 0 {
            findings.push(ObservabilityFinding::MissingRetention(signal.name));
        }

        if signal.kind == SignalKind::Metric && signal.high_cardinality {
            findings.push(ObservabilityFinding::HighCardinalityMetric(signal.name));
        }

        if signal.action.is_none() {
            findings.push(ObservabilityFinding::MissingAction(signal.name));
        }
    }

    ObservabilityEvaluation {
        observable: findings.is_empty(),
        findings,
    }
}

fn has_signal(plan: &ObservabilityPlan, kind: SignalKind) -> bool {
    plan.signals.iter().any(|signal| signal.kind == kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_complete_release_health_plan() {
        let plan = ObservabilityPlan::new(OperationalQuestion::ReleaseHealth)
            .observing(
                TelemetrySignal::new(SignalKind::Log, "checkout.error")
                    .emitted_by("checkout-api", "production")
                    .for_version("v1.2.0")
                    .with_correlation_key()
                    .structured()
                    .retained_for(30)
                    .enabling(OperationalAction::Investigate),
            )
            .observing(
                TelemetrySignal::new(SignalKind::Metric, "http_error_rate")
                    .emitted_by("checkout-api", "production")
                    .for_version("v1.2.0")
                    .with_correlation_key()
                    .structured()
                    .retained_for(30)
                    .enabling(OperationalAction::Pause),
            )
            .observing(
                TelemetrySignal::new(SignalKind::Trace, "checkout.request")
                    .emitted_by("checkout-api", "production")
                    .for_version("v1.2.0")
                    .with_correlation_key()
                    .structured()
                    .retained_for(7)
                    .enabling(OperationalAction::RollBack),
            );

        assert_eq!(
            evaluate_observability(&plan),
            ObservabilityEvaluation {
                observable: true,
                findings: Vec::new(),
            }
        );
    }

    #[test]
    fn detects_missing_core_signals_and_context() {
        let plan = ObservabilityPlan::new(OperationalQuestion::IncidentDiagnosis)
            .observing(TelemetrySignal::new(SignalKind::Log, "checkout failed"));

        assert_eq!(
            evaluate_observability(&plan).findings,
            vec![
                ObservabilityFinding::MissingMetrics,
                ObservabilityFinding::MissingTraces,
                ObservabilityFinding::MissingServiceContext("checkout failed"),
                ObservabilityFinding::MissingEnvironmentContext("checkout failed"),
                ObservabilityFinding::MissingVersionContext("checkout failed"),
                ObservabilityFinding::MissingCorrelation("checkout failed"),
                ObservabilityFinding::UnstructuredLog("checkout failed"),
                ObservabilityFinding::MissingRetention("checkout failed"),
                ObservabilityFinding::MissingAction("checkout failed"),
            ]
        );
    }

    #[test]
    fn flags_high_cardinality_metric() {
        let plan = ObservabilityPlan::new(OperationalQuestion::UserImpact)
            .observing(
                TelemetrySignal::new(SignalKind::Log, "checkout.error")
                    .emitted_by("checkout-api", "production")
                    .for_version("v1.2.0")
                    .with_correlation_key()
                    .structured()
                    .retained_for(30)
                    .enabling(OperationalAction::Investigate),
            )
            .observing(
                TelemetrySignal::new(SignalKind::Metric, "requests_by_user_id")
                    .emitted_by("checkout-api", "production")
                    .for_version("v1.2.0")
                    .with_correlation_key()
                    .structured()
                    .retained_for(30)
                    .with_high_cardinality()
                    .enabling(OperationalAction::Escalate),
            )
            .observing(
                TelemetrySignal::new(SignalKind::Trace, "checkout.request")
                    .emitted_by("checkout-api", "production")
                    .for_version("v1.2.0")
                    .with_correlation_key()
                    .structured()
                    .retained_for(7)
                    .enabling(OperationalAction::RollBack),
            );

        assert_eq!(
            evaluate_observability(&plan).findings,
            vec![ObservabilityFinding::HighCardinalityMetric(
                "requests_by_user_id",
            )]
        );
    }
}
