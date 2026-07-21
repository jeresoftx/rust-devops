# Observabilidad

> **Curso:** DevOps · **Capítulo:** 06 · **Prerrequisitos:** sistemas distribuidos y despliegues
> **Código:** `src/observability.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`draft`

## Concepto

Observabilidad es la capacidad de **entender el estado interno de un sistema a
partir de sus señales externas**. En DevOps, no basta con que un servicio
compile, pase pruebas y se despliegue; también debe poder explicar qué está
pasando cuando recibe tráfico real.

La unidad mental del capítulo es:

1. emitir eventos relevantes;
2. medir comportamiento agregado;
3. seguir una petición entre componentes;
4. correlacionar señales con cambios recientes;
5. convertir evidencia en una decisión operativa.

Logs, métricas y trazas no compiten entre sí. Cada señal responde una pregunta
distinta y juntas permiten leer un sistema vivo sin depender de adivinanzas.

## Problema

Cuando producción falla, el código fuente no basta. El equipo necesita señales
que permitan reconstruir qué pasó, dónde pasó, a quién afectó y qué cambió
antes del incidente.

El problema real aparece cuando el sistema tiene "datos" pero no tiene
observabilidad. Hay logs sin estructura, métricas sin dueño, dashboards que no
responden preguntas y trazas que no cruzan fronteras importantes. En ese
contexto, cada incidente se diagnostica por intuición, búsqueda manual o
conocimiento tribal.

Observabilidad reduce esa ceguera operativa. Su trabajo no es llenar pantallas:
es producir evidencia suficiente para tomar decisiones.

## Alternativas

### Logs sueltos

El equipo imprime mensajes y busca texto cuando algo falla.

Ventaja: rápido de iniciar.
Costo: difícil de correlacionar, filtrar y usar bajo presión.

### Métricas aisladas

El equipo grafica CPU, memoria, errores o latencia sin conectarlas con causas.

Ventaja: buena vista agregada.
Costo: puede mostrar síntomas sin explicar qué petición, usuario o cambio los
produjo.

### Dashboards manuales

Una persona arma pantallas para revisar producción.

Ventaja: mejora visibilidad cotidiana.
Costo: si no nacen de preguntas operativas, se vuelven decoración.

### Trazas distribuidas

Cada petición conserva contexto al cruzar servicios.

Ventaja: permite seguir causalidad entre componentes.
Costo: exige instrumentación consistente y propagación de contexto.

### Observabilidad como contrato

Cada servicio declara qué señales emite, qué preguntas responde y qué acción
habilita.

Ventaja: conecta ingeniería, operación y respuesta a incidentes.
Costo: requiere criterio para no medir todo sin propósito.

## Justificación

Este capítulo viene después de releases porque una versión publicada debe poder
observarse. Release management responde "qué cambió"; observabilidad responde
"qué está ocurriendo después del cambio".

La decisión pedagógica es enseñar observabilidad como lectura del sistema, no
como catálogo de herramientas. Prometheus, Grafana, OpenTelemetry o cualquier
plataforma concreta aparecen después; primero importa entender qué señal
necesita el equipo y qué decisión permite tomar.

## Invariantes del capítulo

Una estrategia de observabilidad explicada aquí debe declarar:

- **Pregunta operativa:** qué queremos poder responder.
- **Señales:** logs, métricas, trazas o eventos relevantes.
- **Contexto:** servicio, ambiente, versión, región, usuario o correlación.
- **Calidad de señal:** estructura, cardinalidad, latencia y completitud.
- **Relación con cambios:** release, deployment, feature flag o incidente.
- **Acción habilitada:** continuar, investigar, pausar, revertir o escalar.
- **Dueño:** quién mantiene la señal y responde cuando alerta.
- **Retención:** cuánto tiempo debe conservarse la evidencia.

## Fronteras con otros capítulos

- Pipelines de CI/CD producen evidencia antes de publicar.
- Estrategias de despliegue usan señales para promover o revertir.
- Gestión de releases identifica qué versión cambió.
- Observabilidad interpreta comportamiento en producción.
- Stack Grafana profundizará herramientas concretas de visualización.
- Alertas, SLOs y SLIs convertirán señales en objetivos y notificaciones.
- Retención de telemetría discutirá almacenamiento, costo y cumplimiento.

## Fuera de alcance en este issue

Este issue no agrega todavía modelo Rust, ejemplo completo, diagrama final,
ejercicios ni benchmark. Esos pasos viven en los issues siguientes del
milestone `06. Observabilidad`.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama de correlación log-métrica-traza.
- Modelo Rust mínimo de señales operativas.
- Ejemplos progresivos y pruebas.
- Benchmarks de volumen o justificación de no aplicabilidad.
