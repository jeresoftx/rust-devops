# Retención de telemetría

> **Curso:** DevOps · **Capítulo:** 09 · **Prerequisitos:** Stack Grafana
> **Código:** `src/telemetry_retention.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`planned`

## Intención

Este capítulo explicará políticas de retención hot, warm y cold para logs,
métricas y trazas: qué guardar, cuánto tiempo, dónde y con qué costo.

## Problema

Guardar toda la telemetría para siempre es caro; guardar demasiado poco vuelve
imposible investigar incidentes. La decisión correcta depende de valor
operativo, costo, cumplimiento y frecuencia de consulta.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama de ciclo de vida de telemetría.
- Modelo Rust mínimo de política de retención.
- Ejemplos progresivos y pruebas.
- Benchmarks o estimaciones de costo documentadas.
