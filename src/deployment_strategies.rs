//! Modelo mínimo para razonar sobre estrategias de despliegue.
//!
//! El módulo no opera Kubernetes ni balanceadores reales. Representa decisiones
//! de exposición, señales y reversibilidad para evaluar si una estrategia
//! reduce riesgo antes de ampliar tráfico.

/// Estrategia usada para liberar una versión.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentStrategyKind {
    /// Todo el tráfico cambia de golpe.
    BigBang,
    /// Las instancias se reemplazan gradualmente.
    Rolling,
    /// Dos ambientes alternan el tráfico.
    BlueGreen,
    /// Una fracción pequeña recibe la versión nueva primero.
    Canary,
    /// La funcionalidad se activa por bandera después de desplegar código.
    FeatureFlag,
}

/// Señal observada durante el despliegue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthSignal {
    /// Errores de aplicación o HTTP.
    ErrorRate,
    /// Latencia percibida por usuario o servicio.
    Latency,
    /// Reinicios o fallos de proceso.
    Restarts,
    /// Métrica de negocio afectada por el cambio.
    BusinessMetric,
}

/// Acción recomendada por la evaluación.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentDecision {
    /// La estrategia puede avanzar.
    Continue,
    /// El despliegue debe detenerse para observar.
    Pause,
    /// El cambio debe revertirse.
    RollBack,
}

/// Especificación de una estrategia de despliegue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentStrategy {
    /// Nombre de la versión o release.
    pub release: &'static str,
    /// Tipo de estrategia.
    pub kind: DeploymentStrategyKind,
    /// Porcentaje inicial de usuarios o tráfico expuesto.
    pub initial_exposure_percent: u8,
    /// Porcentaje máximo que se permite alcanzar en este plan.
    pub max_exposure_percent: u8,
    /// Señales observadas antes de avanzar.
    pub health_signals: Vec<HealthSignal>,
    /// Si existe camino documentado de rollback.
    pub rollback_available: bool,
    /// Si la estrategia contempla convivencia temporal de versiones.
    pub version_compatibility: bool,
}

impl DeploymentStrategy {
    /// Crea una estrategia con campos mínimos.
    ///
    /// ```
    /// let strategy = rust_devops::deployment_strategies::DeploymentStrategy::new(
    ///     "checkout-api-1.0.0",
    ///     rust_devops::deployment_strategies::DeploymentStrategyKind::Canary,
    ///     5,
    ///     50,
    /// );
    /// assert_eq!(strategy.initial_exposure_percent, 5);
    /// ```
    pub fn new(
        release: &'static str,
        kind: DeploymentStrategyKind,
        initial_exposure_percent: u8,
        max_exposure_percent: u8,
    ) -> Self {
        Self {
            release,
            kind,
            initial_exposure_percent,
            max_exposure_percent,
            health_signals: Vec::new(),
            rollback_available: false,
            version_compatibility: false,
        }
    }

    /// Agrega una señal observada.
    pub fn observing(mut self, signal: HealthSignal) -> Self {
        self.health_signals.push(signal);
        self
    }

    /// Declara rollback disponible.
    pub fn with_rollback(mut self) -> Self {
        self.rollback_available = true;
        self
    }

    /// Declara compatibilidad temporal entre versiones.
    pub fn with_version_compatibility(mut self) -> Self {
        self.version_compatibility = true;
        self
    }
}

/// Hallazgo de riesgo operativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentFinding {
    /// El release no tiene identificador.
    MissingRelease,
    /// La exposición inicial es demasiado alta para la estrategia.
    ExcessiveInitialExposure,
    /// La exposición máxima es menor a la inicial.
    InvalidExposureRange,
    /// No hay señales suficientes para decidir.
    MissingHealthSignals,
    /// Falta rollback.
    MissingRollback,
    /// Falta compatibilidad temporal entre versiones.
    MissingVersionCompatibility,
    /// La estrategia no reduce blast radius.
    FullBlastRadius,
}

/// Resultado de evaluar una estrategia.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentEvaluation {
    /// Decisión recomendada.
    pub decision: DeploymentDecision,
    /// Hallazgos que explican el riesgo.
    pub findings: Vec<DeploymentFinding>,
}

/// Evalúa si una estrategia limita exposición y conserva reversibilidad.
///
/// ```
/// use rust_devops::deployment_strategies::{
///     evaluate_deployment, DeploymentDecision, DeploymentStrategy,
///     DeploymentStrategyKind, HealthSignal,
/// };
///
/// let strategy = DeploymentStrategy::new("api-1.0.0", DeploymentStrategyKind::Canary, 5, 50)
///     .observing(HealthSignal::ErrorRate)
///     .observing(HealthSignal::Latency)
///     .with_rollback()
///     .with_version_compatibility();
///
/// assert_eq!(evaluate_deployment(&strategy).decision, DeploymentDecision::Continue);
/// ```
pub fn evaluate_deployment(strategy: &DeploymentStrategy) -> DeploymentEvaluation {
    let mut findings = Vec::new();

    if strategy.release.is_empty() {
        findings.push(DeploymentFinding::MissingRelease);
    }

    if strategy.max_exposure_percent < strategy.initial_exposure_percent {
        findings.push(DeploymentFinding::InvalidExposureRange);
    }

    if strategy.initial_exposure_percent > recommended_initial_exposure(strategy.kind) {
        findings.push(DeploymentFinding::ExcessiveInitialExposure);
    }

    if strategy.health_signals.len() < minimum_signal_count(strategy.kind) {
        findings.push(DeploymentFinding::MissingHealthSignals);
    }

    if !strategy.rollback_available {
        findings.push(DeploymentFinding::MissingRollback);
    }

    if requires_version_compatibility(strategy.kind) && !strategy.version_compatibility {
        findings.push(DeploymentFinding::MissingVersionCompatibility);
    }

    if strategy.kind == DeploymentStrategyKind::BigBang
        && strategy.initial_exposure_percent == 100
        && strategy.max_exposure_percent == 100
    {
        findings.push(DeploymentFinding::FullBlastRadius);
    }

    DeploymentEvaluation {
        decision: decision_for(&findings),
        findings,
    }
}

fn recommended_initial_exposure(kind: DeploymentStrategyKind) -> u8 {
    match kind {
        DeploymentStrategyKind::BigBang => 100,
        DeploymentStrategyKind::Rolling => 50,
        DeploymentStrategyKind::BlueGreen => 100,
        DeploymentStrategyKind::Canary => 10,
        DeploymentStrategyKind::FeatureFlag => 10,
    }
}

fn minimum_signal_count(kind: DeploymentStrategyKind) -> usize {
    match kind {
        DeploymentStrategyKind::BigBang => 3,
        DeploymentStrategyKind::Rolling => 2,
        DeploymentStrategyKind::BlueGreen => 2,
        DeploymentStrategyKind::Canary => 2,
        DeploymentStrategyKind::FeatureFlag => 2,
    }
}

fn requires_version_compatibility(kind: DeploymentStrategyKind) -> bool {
    matches!(
        kind,
        DeploymentStrategyKind::Rolling
            | DeploymentStrategyKind::Canary
            | DeploymentStrategyKind::FeatureFlag
    )
}

fn decision_for(findings: &[DeploymentFinding]) -> DeploymentDecision {
    if findings
        .iter()
        .any(|finding| matches!(finding, DeploymentFinding::FullBlastRadius))
    {
        DeploymentDecision::RollBack
    } else if findings.is_empty() {
        DeploymentDecision::Continue
    } else {
        DeploymentDecision::Pause
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_guarded_canary() {
        let strategy =
            DeploymentStrategy::new("booking-api-1.0.0", DeploymentStrategyKind::Canary, 5, 50)
                .observing(HealthSignal::ErrorRate)
                .observing(HealthSignal::Latency)
                .with_rollback()
                .with_version_compatibility();

        assert_eq!(
            evaluate_deployment(&strategy),
            DeploymentEvaluation {
                decision: DeploymentDecision::Continue,
                findings: Vec::new(),
            }
        );
    }

    #[test]
    fn pauses_risky_rolling_update() {
        let strategy = DeploymentStrategy::new(
            "booking-api-1.1.0",
            DeploymentStrategyKind::Rolling,
            75,
            100,
        )
        .observing(HealthSignal::ErrorRate);

        assert_eq!(
            evaluate_deployment(&strategy).findings,
            vec![
                DeploymentFinding::ExcessiveInitialExposure,
                DeploymentFinding::MissingHealthSignals,
                DeploymentFinding::MissingRollback,
                DeploymentFinding::MissingVersionCompatibility,
            ]
        );
    }

    #[test]
    fn rolls_back_full_blast_radius_without_guardrails() {
        let strategy = DeploymentStrategy::new(
            "booking-api-2.0.0",
            DeploymentStrategyKind::BigBang,
            100,
            100,
        );

        assert_eq!(
            evaluate_deployment(&strategy).decision,
            DeploymentDecision::RollBack
        );
    }
}
