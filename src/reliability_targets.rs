//! Modelo mínimo para razonar sobre SLIs, SLOs, error budgets y alertas.
//!
//! El módulo no consulta Prometheus ni dispara notificaciones reales. Representa
//! indicadores, objetivos y políticas de alerta para evaluar si una señal de
//! confiabilidad está conectada con experiencia de usuario y acción humana.

/// Experiencia que un indicador intenta aproximar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliKind {
    /// Porcentaje de operaciones exitosas.
    Availability,
    /// Porcentaje de operaciones debajo de un umbral de latencia.
    Latency,
    /// Porcentaje de datos frescos dentro de una ventana esperada.
    Freshness,
    /// Porcentaje de respuestas correctas para el usuario.
    Correctness,
}

/// Indicador de nivel de servicio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceLevelIndicator {
    /// Nombre estable del indicador.
    pub name: &'static str,
    /// Experiencia que mide.
    pub kind: SliKind,
    /// Eventos buenos observados.
    pub good_events: u64,
    /// Eventos totales observados.
    pub total_events: u64,
}

impl ServiceLevelIndicator {
    /// Crea un SLI sin eventos.
    ///
    /// ```
    /// let sli = rust_devops::reliability_targets::ServiceLevelIndicator::new(
    ///     "checkout_success_ratio",
    ///     rust_devops::reliability_targets::SliKind::Availability,
    /// );
    ///
    /// assert_eq!(sli.name, "checkout_success_ratio");
    /// ```
    pub fn new(name: &'static str, kind: SliKind) -> Self {
        Self {
            name,
            kind,
            good_events: 0,
            total_events: 0,
        }
    }

    /// Declara eventos buenos y totales.
    pub fn observed(mut self, good_events: u64, total_events: u64) -> Self {
        self.good_events = good_events;
        self.total_events = total_events;
        self
    }

    /// Calcula el porcentaje de eventos buenos.
    pub fn success_percent(&self) -> Option<f64> {
        if self.total_events == 0 || self.good_events > self.total_events {
            None
        } else {
            Some((self.good_events as f64 / self.total_events as f64) * 100.0)
        }
    }
}

/// Objetivo de nivel de servicio.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceLevelObjective {
    /// Nombre del objetivo.
    pub name: &'static str,
    /// Indicador asociado.
    pub indicator: ServiceLevelIndicator,
    /// Porcentaje objetivo.
    pub target_percent: f64,
    /// Ventana de evaluación en días.
    pub window_days: u16,
}

impl ServiceLevelObjective {
    /// Crea un SLO para un indicador.
    pub fn new(
        name: &'static str,
        indicator: ServiceLevelIndicator,
        target_percent: f64,
        window_days: u16,
    ) -> Self {
        Self {
            name,
            indicator,
            target_percent,
            window_days,
        }
    }

    /// Presupuesto de error permitido en porcentaje.
    pub fn error_budget_percent(&self) -> Option<f64> {
        if self.target_percent <= 0.0 || self.target_percent >= 100.0 {
            None
        } else {
            Some(100.0 - self.target_percent)
        }
    }
}

/// Acción esperada cuando una alerta se dispara.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReliabilityAction {
    /// Investigar sin interrumpir a todo el equipo.
    Investigate,
    /// Pausar un despliegue o promoción.
    PauseRollout,
    /// Revertir a una versión segura.
    RollBack,
    /// Escalar a responsable o equipo.
    Escalate,
}

/// Política de alerta asociada a un SLO.
#[derive(Debug, Clone, PartialEq)]
pub struct AlertPolicy {
    /// Nombre de la alerta.
    pub name: &'static str,
    /// Responsable primario.
    pub owner: &'static str,
    /// Acción esperada.
    pub action: Option<ReliabilityAction>,
    /// Burn rate observado o esperado.
    pub burn_rate: f64,
    /// Si amerita interrupción inmediata.
    pub page_human: bool,
}

impl AlertPolicy {
    /// Crea una política de alerta mínima.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            owner: "",
            action: None,
            burn_rate: 0.0,
            page_human: false,
        }
    }

    /// Declara responsable.
    pub fn owned_by(mut self, owner: &'static str) -> Self {
        self.owner = owner;
        self
    }

    /// Declara acción.
    pub fn triggering(mut self, action: ReliabilityAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Declara burn rate.
    pub fn with_burn_rate(mut self, burn_rate: f64) -> Self {
        self.burn_rate = burn_rate;
        self
    }

    /// Marca que debe interrumpir a una persona.
    pub fn paging_human(mut self) -> Self {
        self.page_human = true;
        self
    }
}

/// Hallazgo de riesgo en un objetivo de confiabilidad.
#[derive(Debug, Clone, PartialEq)]
pub enum ReliabilityFinding {
    /// Falta nombre del SLI.
    MissingSliName,
    /// No hay eventos suficientes.
    MissingEvents,
    /// Los eventos buenos exceden el total.
    InvalidEventCount,
    /// Falta nombre del SLO.
    MissingSloName,
    /// Objetivo fuera de rango.
    InvalidTargetPercent,
    /// Falta ventana de evaluación.
    MissingWindow,
    /// Se consumió más de 100% del presupuesto.
    ErrorBudgetExhausted {
        /// Consumo observado del presupuesto.
        consumed_percent: f64,
    },
    /// Falta dueño de la alerta.
    MissingAlertOwner,
    /// Falta acción esperada.
    MissingAlertAction,
    /// Hay burn rate rápido, pero no interrupción humana.
    FastBurnWithoutPage,
}

/// Resultado de evaluar un SLO y su alerta.
#[derive(Debug, Clone, PartialEq)]
pub struct ReliabilityEvaluation {
    /// Porcentaje de éxito observado.
    pub success_percent: Option<f64>,
    /// Porcentaje de presupuesto de error consumido.
    pub error_budget_consumed_percent: Option<f64>,
    /// Si el objetivo conserva invariantes operativas.
    pub reliable: bool,
    /// Hallazgos encontrados.
    pub findings: Vec<ReliabilityFinding>,
}

/// Evalúa un SLO y su política de alerta.
///
/// ```
/// use rust_devops::reliability_targets::{
///     evaluate_reliability, AlertPolicy, ReliabilityAction,
///     ServiceLevelIndicator, ServiceLevelObjective, SliKind,
/// };
///
/// let sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
///     .observed(99_950, 100_000);
/// let slo = ServiceLevelObjective::new("checkout availability", sli, 99.9, 30);
/// let alert = AlertPolicy::new("checkout fast burn")
///     .owned_by("payments-oncall")
///     .triggering(ReliabilityAction::PauseRollout)
///     .with_burn_rate(2.0)
///     .paging_human();
///
/// assert!(evaluate_reliability(&slo, &alert).reliable);
/// ```
pub fn evaluate_reliability(
    slo: &ServiceLevelObjective,
    alert: &AlertPolicy,
) -> ReliabilityEvaluation {
    let mut findings = Vec::new();

    if slo.indicator.name.is_empty() {
        findings.push(ReliabilityFinding::MissingSliName);
    }

    if slo.indicator.total_events == 0 {
        findings.push(ReliabilityFinding::MissingEvents);
    }

    if slo.indicator.good_events > slo.indicator.total_events {
        findings.push(ReliabilityFinding::InvalidEventCount);
    }

    if slo.name.is_empty() {
        findings.push(ReliabilityFinding::MissingSloName);
    }

    if slo.error_budget_percent().is_none() {
        findings.push(ReliabilityFinding::InvalidTargetPercent);
    }

    if slo.window_days == 0 {
        findings.push(ReliabilityFinding::MissingWindow);
    }

    let success_percent = slo.indicator.success_percent();
    let consumed = match (success_percent, slo.error_budget_percent()) {
        (Some(success), Some(error_budget)) => {
            let observed_error = 100.0 - success;
            Some((observed_error / error_budget) * 100.0)
        }
        _ => None,
    };

    if let Some(consumed_percent) = consumed
        && consumed_percent > 100.0
    {
        findings.push(ReliabilityFinding::ErrorBudgetExhausted { consumed_percent });
    }

    if alert.owner.is_empty() {
        findings.push(ReliabilityFinding::MissingAlertOwner);
    }

    if alert.action.is_none() {
        findings.push(ReliabilityFinding::MissingAlertAction);
    }

    if alert.burn_rate >= 2.0 && !alert.page_human {
        findings.push(ReliabilityFinding::FastBurnWithoutPage);
    }

    ReliabilityEvaluation {
        success_percent,
        error_budget_consumed_percent: consumed,
        reliable: findings.is_empty(),
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_slo_with_remaining_budget_and_actionable_alert() {
        let sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
            .observed(99_950, 100_000);
        let slo = ServiceLevelObjective::new("checkout availability", sli, 99.9, 30);
        let alert = AlertPolicy::new("checkout fast burn")
            .owned_by("payments-oncall")
            .triggering(ReliabilityAction::PauseRollout)
            .with_burn_rate(2.0)
            .paging_human();

        let evaluation = evaluate_reliability(&slo, &alert);

        assert!(evaluation.reliable);
        assert!(evaluation.findings.is_empty());
        assert_eq!(evaluation.success_percent, Some(99.95));
    }

    #[test]
    fn detects_exhausted_error_budget() {
        let sli = ServiceLevelIndicator::new("checkout_success_ratio", SliKind::Availability)
            .observed(99_700, 100_000);
        let slo = ServiceLevelObjective::new("checkout availability", sli, 99.9, 30);
        let alert = AlertPolicy::new("checkout slow burn")
            .owned_by("payments-oncall")
            .triggering(ReliabilityAction::Investigate)
            .with_burn_rate(1.0);

        let evaluation = evaluate_reliability(&slo, &alert);

        assert!(!evaluation.reliable);
        assert!(
            evaluation.findings.iter().any(|finding| {
                matches!(finding, ReliabilityFinding::ErrorBudgetExhausted { .. })
            })
        );
    }

    #[test]
    fn flags_alert_without_owner_action_or_page() {
        let sli = ServiceLevelIndicator::new("api_latency_under_threshold", SliKind::Latency)
            .observed(99_000, 100_000);
        let slo = ServiceLevelObjective::new("api latency", sli, 99.0, 28);
        let alert = AlertPolicy::new("api fast burn").with_burn_rate(3.0);

        let evaluation = evaluate_reliability(&slo, &alert);

        assert!(!evaluation.reliable);
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::MissingAlertOwner)
        );
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::MissingAlertAction)
        );
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::FastBurnWithoutPage)
        );
    }

    #[test]
    fn rejects_invalid_measurements_and_targets() {
        let sli = ServiceLevelIndicator::new("", SliKind::Correctness).observed(2, 1);
        let slo = ServiceLevelObjective::new("", sli, 100.0, 0);
        let alert = AlertPolicy::new("invalid").owned_by("platform");

        let evaluation = evaluate_reliability(&slo, &alert);

        assert!(!evaluation.reliable);
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::MissingSliName)
        );
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::InvalidEventCount)
        );
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::MissingSloName)
        );
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::InvalidTargetPercent)
        );
        assert!(
            evaluation
                .findings
                .contains(&ReliabilityFinding::MissingWindow)
        );
    }
}
