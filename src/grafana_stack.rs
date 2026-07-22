//! Modelo mínimo para razonar sobre Stack Grafana.
//!
//! El módulo no levanta Grafana, Prometheus, Loki, Tempo ni Alloy. Representa
//! rutas de telemetría para evaluar si una señal tiene productor, recolección,
//! backend, visualización, etiquetas y retención coherentes.

/// Tipo de señal que viaja por el stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetrySignalKind {
    /// Serie temporal agregada, consultable con PromQL.
    Metric,
    /// Evento textual o estructurado, consultable como flujo de logs.
    Log,
    /// Recorrido de una petición entre componentes.
    Trace,
}

/// Componente del ecosistema Grafana usado en la ruta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrafanaComponent {
    /// Agente de recolección, transformación y envío.
    Alloy,
    /// Backend de métricas.
    Prometheus,
    /// Backend de logs.
    Loki,
    /// Backend de trazas.
    Tempo,
    /// Interfaz de consulta, dashboard y exploración.
    Grafana,
}

/// Cardinalidad esperada de una etiqueta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelCardinality {
    /// Pocos valores controlados, como ambiente o servicio.
    Low,
    /// Valor acotado pero cambiante, como versión o región.
    Bounded,
    /// Valor potencialmente ilimitado, como usuario, correo o request id.
    Unbounded,
}

/// Etiqueta declarada para una señal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryLabel {
    /// Nombre de la etiqueta.
    pub name: &'static str,
    /// Cardinalidad esperada.
    pub cardinality: LabelCardinality,
}

impl TelemetryLabel {
    /// Crea una etiqueta de telemetría.
    pub fn new(name: &'static str, cardinality: LabelCardinality) -> Self {
        Self { name, cardinality }
    }
}

/// Ruta educativa de una señal dentro de Stack Grafana.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryRoute {
    /// Tipo de señal.
    pub signal: TelemetrySignalKind,
    /// Servicio o proceso que produce la señal.
    pub producer: &'static str,
    /// Componente que recolecta o reenvía la señal.
    pub collector: Option<GrafanaComponent>,
    /// Backend donde se almacena la señal.
    pub backend: Option<GrafanaComponent>,
    /// Componente desde donde una persona consulta la señal.
    pub visualization: Option<GrafanaComponent>,
    /// Etiquetas disponibles.
    pub labels: Vec<TelemetryLabel>,
    /// Retención en días.
    pub retention_days: u16,
    /// Si la ruta conserva una llave para correlacionar con otras señales.
    pub correlation_key: bool,
    /// Pregunta operativa que la ruta ayuda a responder.
    pub question: &'static str,
}

impl TelemetryRoute {
    /// Crea una ruta mínima.
    ///
    /// ```
    /// let route = rust_devops::grafana_stack::TelemetryRoute::new(
    ///     rust_devops::grafana_stack::TelemetrySignalKind::Metric,
    ///     "checkout-api",
    /// );
    ///
    /// assert_eq!(route.producer, "checkout-api");
    /// ```
    pub fn new(signal: TelemetrySignalKind, producer: &'static str) -> Self {
        Self {
            signal,
            producer,
            collector: None,
            backend: None,
            visualization: None,
            labels: Vec::new(),
            retention_days: 0,
            correlation_key: false,
            question: "",
        }
    }

    /// Declara el componente recolector.
    pub fn collected_by(mut self, collector: GrafanaComponent) -> Self {
        self.collector = Some(collector);
        self
    }

    /// Declara el backend de almacenamiento.
    pub fn stored_in(mut self, backend: GrafanaComponent) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Declara el componente de visualización.
    pub fn visualized_in(mut self, visualization: GrafanaComponent) -> Self {
        self.visualization = Some(visualization);
        self
    }

    /// Agrega una etiqueta.
    pub fn labeled_with(mut self, label: TelemetryLabel) -> Self {
        self.labels.push(label);
        self
    }

    /// Declara retención.
    pub fn retained_for(mut self, days: u16) -> Self {
        self.retention_days = days;
        self
    }

    /// Declara correlación entre señales.
    pub fn with_correlation_key(mut self) -> Self {
        self.correlation_key = true;
        self
    }

    /// Declara la pregunta operativa asociada.
    pub fn answering(mut self, question: &'static str) -> Self {
        self.question = question;
        self
    }
}

/// Hallazgo de riesgo en una ruta de Stack Grafana.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrafanaStackFinding {
    /// Falta productor.
    MissingProducer,
    /// Falta recolector.
    MissingCollector,
    /// El recolector no es Alloy.
    CollectorIsNotAlloy,
    /// Falta backend.
    MissingBackend,
    /// El backend no corresponde al tipo de señal.
    WrongBackend {
        /// Backend esperado.
        expected: GrafanaComponent,
        /// Backend observado.
        actual: GrafanaComponent,
    },
    /// Falta visualización.
    MissingVisualization,
    /// La visualización no ocurre en Grafana.
    VisualizationIsNotGrafana,
    /// No hay etiquetas.
    MissingLabels,
    /// Etiqueta con cardinalidad ilimitada.
    UnboundedLabel(&'static str),
    /// Falta retención.
    MissingRetention,
    /// Falta correlación entre señales.
    MissingCorrelation,
    /// Falta pregunta operativa.
    MissingQuestion,
}

/// Resultado de evaluar una ruta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrafanaStackEvaluation {
    /// Si la ruta conserva las invariantes esperadas.
    pub ready: bool,
    /// Hallazgos encontrados.
    pub findings: Vec<GrafanaStackFinding>,
}

/// Evalúa si una ruta de telemetría usa Stack Grafana con intención.
///
/// ```
/// use rust_devops::grafana_stack::{
///     evaluate_route, GrafanaComponent, LabelCardinality, TelemetryLabel,
///     TelemetryRoute, TelemetrySignalKind,
/// };
///
/// let route = TelemetryRoute::new(TelemetrySignalKind::Metric, "checkout-api")
///     .collected_by(GrafanaComponent::Alloy)
///     .stored_in(GrafanaComponent::Prometheus)
///     .visualized_in(GrafanaComponent::Grafana)
///     .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
///     .labeled_with(TelemetryLabel::new("version", LabelCardinality::Bounded))
///     .retained_for(30)
///     .with_correlation_key()
///     .answering("¿la versión nueva está sana?");
///
/// assert!(evaluate_route(&route).ready);
/// ```
pub fn evaluate_route(route: &TelemetryRoute) -> GrafanaStackEvaluation {
    let mut findings = Vec::new();

    if route.producer.is_empty() {
        findings.push(GrafanaStackFinding::MissingProducer);
    }

    match route.collector {
        Some(GrafanaComponent::Alloy) => {}
        Some(_) => findings.push(GrafanaStackFinding::CollectorIsNotAlloy),
        None => findings.push(GrafanaStackFinding::MissingCollector),
    }

    let expected_backend = expected_backend_for(route.signal);
    match route.backend {
        Some(actual) if actual == expected_backend => {}
        Some(actual) => findings.push(GrafanaStackFinding::WrongBackend {
            expected: expected_backend,
            actual,
        }),
        None => findings.push(GrafanaStackFinding::MissingBackend),
    }

    match route.visualization {
        Some(GrafanaComponent::Grafana) => {}
        Some(_) => findings.push(GrafanaStackFinding::VisualizationIsNotGrafana),
        None => findings.push(GrafanaStackFinding::MissingVisualization),
    }

    if route.labels.is_empty() {
        findings.push(GrafanaStackFinding::MissingLabels);
    }

    for label in &route.labels {
        if label.cardinality == LabelCardinality::Unbounded {
            findings.push(GrafanaStackFinding::UnboundedLabel(label.name));
        }
    }

    if route.retention_days == 0 {
        findings.push(GrafanaStackFinding::MissingRetention);
    }

    if !route.correlation_key {
        findings.push(GrafanaStackFinding::MissingCorrelation);
    }

    if route.question.is_empty() {
        findings.push(GrafanaStackFinding::MissingQuestion);
    }

    GrafanaStackEvaluation {
        ready: findings.is_empty(),
        findings,
    }
}

/// Devuelve el backend esperado para cada señal.
pub fn expected_backend_for(signal: TelemetrySignalKind) -> GrafanaComponent {
    match signal {
        TelemetrySignalKind::Metric => GrafanaComponent::Prometheus,
        TelemetrySignalKind::Log => GrafanaComponent::Loki,
        TelemetrySignalKind::Trace => GrafanaComponent::Tempo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_complete_metric_route() {
        let route = TelemetryRoute::new(TelemetrySignalKind::Metric, "checkout-api")
            .collected_by(GrafanaComponent::Alloy)
            .stored_in(GrafanaComponent::Prometheus)
            .visualized_in(GrafanaComponent::Grafana)
            .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
            .labeled_with(TelemetryLabel::new("environment", LabelCardinality::Low))
            .labeled_with(TelemetryLabel::new("version", LabelCardinality::Bounded))
            .retained_for(30)
            .with_correlation_key()
            .answering("¿la versión nueva está sana?");

        let evaluation = evaluate_route(&route);

        assert!(evaluation.ready);
        assert!(evaluation.findings.is_empty());
    }

    #[test]
    fn detects_wrong_backend_for_logs() {
        let route = TelemetryRoute::new(TelemetrySignalKind::Log, "checkout-worker")
            .collected_by(GrafanaComponent::Alloy)
            .stored_in(GrafanaComponent::Prometheus)
            .visualized_in(GrafanaComponent::Grafana)
            .labeled_with(TelemetryLabel::new("service", LabelCardinality::Low))
            .retained_for(14)
            .with_correlation_key()
            .answering("¿qué falló durante el incidente?");

        let evaluation = evaluate_route(&route);

        assert!(!evaluation.ready);
        assert_eq!(
            evaluation.findings,
            vec![GrafanaStackFinding::WrongBackend {
                expected: GrafanaComponent::Loki,
                actual: GrafanaComponent::Prometheus,
            }]
        );
    }

    #[test]
    fn flags_unbounded_labels_and_missing_intention() {
        let route = TelemetryRoute::new(TelemetrySignalKind::Trace, "payments-api")
            .collected_by(GrafanaComponent::Alloy)
            .stored_in(GrafanaComponent::Tempo)
            .visualized_in(GrafanaComponent::Grafana)
            .labeled_with(TelemetryLabel::new(
                "user_email",
                LabelCardinality::Unbounded,
            ));

        let evaluation = evaluate_route(&route);

        assert!(!evaluation.ready);
        assert!(
            evaluation
                .findings
                .contains(&GrafanaStackFinding::UnboundedLabel("user_email"))
        );
        assert!(
            evaluation
                .findings
                .contains(&GrafanaStackFinding::MissingRetention)
        );
        assert!(
            evaluation
                .findings
                .contains(&GrafanaStackFinding::MissingCorrelation)
        );
        assert!(
            evaluation
                .findings
                .contains(&GrafanaStackFinding::MissingQuestion)
        );
    }

    #[test]
    fn maps_signals_to_their_expected_backends() {
        assert_eq!(
            expected_backend_for(TelemetrySignalKind::Metric),
            GrafanaComponent::Prometheus
        );
        assert_eq!(
            expected_backend_for(TelemetrySignalKind::Log),
            GrafanaComponent::Loki
        );
        assert_eq!(
            expected_backend_for(TelemetrySignalKind::Trace),
            GrafanaComponent::Tempo
        );
    }
}
