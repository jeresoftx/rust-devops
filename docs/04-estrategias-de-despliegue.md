# Estrategias de despliegue

> **Curso:** DevOps · **Capítulo:** 04 · **Prerequisitos:** Pipelines de CI/CD
> **Código:** `src/deployment_strategies.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`planned`

## Intención

Este capítulo comparará rolling updates, blue-green, canary, feature flags y
rollback automático como formas de liberar cambios reduciendo impacto.

## Problema

Un deploy no termina cuando el artefacto llega al servidor. Termina cuando el
sistema nuevo funciona, el anterior puede retirarse y existe una salida clara
si algo sale mal.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama de tráfico para cada estrategia principal.
- Modelo Rust mínimo de riesgo, exposición y reversibilidad.
- Ejemplos progresivos y pruebas.
- Benchmarks o métricas operativas justificadas.
