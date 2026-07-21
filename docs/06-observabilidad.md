# Observabilidad

> **Curso:** DevOps · **Capítulo:** 06 · **Prerequisitos:** sistemas distribuidos y despliegues
> **Código:** `src/observability.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`planned`

## Intención

Este capítulo explicará logs, métricas y trazas como tres formas
complementarias de leer un sistema vivo.

## Problema

Cuando producción falla, el código fuente no basta. El equipo necesita señales
que permitan reconstruir qué pasó, dónde pasó, a quién afectó y qué cambió
antes del incidente.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama de correlación log-métrica-traza.
- Modelo Rust mínimo de señales operativas.
- Ejemplos progresivos y pruebas.
- Benchmarks de volumen o justificación de no aplicabilidad.
