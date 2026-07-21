# Estrategias de despliegue

> **Curso:** DevOps · **Capítulo:** 04 · **Prerequisitos:** Pipelines de CI/CD
> **Código:** `src/deployment_strategies.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`draft`

## Concepto

Una estrategia de despliegue es una forma deliberada de **controlar exposición
y reversibilidad** cuando un artefacto nuevo entra a un sistema vivo. No basta
con copiar una versión al ambiente correcto: hay que decidir cuántos usuarios
la verán, qué señales se observarán, cómo se detendrá el cambio y cómo se
volverá a un estado sano si algo falla.

La unidad mental del capítulo es:

1. elegir el cambio que se va a liberar;
2. decidir cuánto tráfico o qué población lo recibirá;
3. observar señales durante la exposición;
4. continuar, pausar, revertir o ampliar el despliegue;
5. registrar la decisión y sus evidencias.

Una buena estrategia no elimina el riesgo. Lo vuelve visible, gradual y
reversible.

## Problema

Un deploy no termina cuando el artefacto llega al servidor. Termina cuando el
sistema nuevo funciona con usuarios reales, el anterior puede retirarse y
existe una salida clara si algo sale mal.

El problema real aparece cuando todos los usuarios reciben el cambio al mismo
tiempo, las señales llegan tarde o el equipo no tiene camino de regreso.
Entonces el despliegue se vuelve apuesta: si sale bien, nadie aprende; si sale
mal, producción paga el costo completo.

Las estrategias de despliegue existen para reducir blast radius: limitar el
impacto inicial mientras el equipo gana evidencia.

## Alternativas

### Big bang

Todo el tráfico cambia a la nueva versión en una sola acción.

Ventaja: simple de entender y operar.
Costo: máximo impacto si falla; rollback urgente y visible.

### Rolling update

Las instancias se reemplazan gradualmente.

Ventaja: no requiere duplicar todo el ambiente y reduce interrupciones.
Costo: durante el rollout conviven versiones; exige compatibilidad temporal.

### Blue-green

Dos ambientes completos existen en paralelo: uno atiende tráfico y el otro se
prepara con la nueva versión.

Ventaja: cambio rápido de tráfico y rollback claro.
Costo: mayor costo de infraestructura y cuidado con datos compartidos.

### Canary

La nueva versión recibe una fracción pequeña de tráfico antes de ampliar
exposición.

Ventaja: detecta problemas con usuarios reales y blast radius bajo.
Costo: requiere routing, métricas confiables y criterios de avance.

### Feature flags

El código se despliega separado de la activación de la funcionalidad.

Ventaja: permite liberar, activar y desactivar por segmento.
Costo: agrega complejidad, deuda de flags y riesgo de combinaciones no probadas.

## Justificación

Este capítulo viene después de CI/CD porque un pipeline puede entregar un
artefacto válido, pero no decide por sí solo cómo exponerlo a usuarios. La
estrategia de despliegue es la capa donde se conecta automatización con riesgo
operativo.

La decisión pedagógica es enseñar estrategias como decisiones de exposición,
señales y reversibilidad. La sintaxis de Kubernetes, flags o proveedores cloud
puede cambiar; el criterio de cuánto riesgo aceptar en cada paso permanece.

## Invariantes del capítulo

Una estrategia de despliegue explicada aquí debe declarar:

- **Artefacto objetivo:** qué versión entra al sistema.
- **Población expuesta:** porcentaje de tráfico, usuarios, región o ambiente.
- **Señales de salud:** métricas, logs, errores y comportamiento esperado.
- **Criterios de avance:** cuándo ampliar exposición.
- **Criterios de pausa:** cuándo detener el rollout.
- **Criterios de rollback:** cuándo volver atrás.
- **Compatibilidad temporal:** qué pasa cuando conviven versiones.
- **Responsable de decisión:** humano, pipeline, controller o regla automatizada.
- **Registro de evidencia:** qué se observó antes de continuar.

## Fronteras con otros cursos

- `rust-cicd` no existe como curso separado: este capítulo usa el modelo de
  Pipelines de CI/CD del propio `rust-devops`.
- `rust-cloud` enseña la plataforma donde ocurren los despliegues.
- `rust-distributed-systems` profundiza en compatibilidad, consistencia y
  fallas parciales.
- `rust-security` profundiza en permisos, cambios sensibles y exposición por
  segmento.
- `rust-observability` no es repo separado aquí; observabilidad aparecerá como
  capítulo posterior para medir señales en producción.

## Fuera de alcance en este issue

Este issue no agrega todavía el modelo Rust, ejemplos completos, diagrama final
ni ejercicios. Esos pasos viven en los issues siguientes del milestone
`04. Estrategias de despliegue`.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama de tráfico para cada estrategia principal.
- Modelo Rust mínimo de riesgo, exposición y reversibilidad.
- Ejemplos progresivos y pruebas.
- Benchmarks o métricas operativas justificadas.
