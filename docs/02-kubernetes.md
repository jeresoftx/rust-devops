# Kubernetes

> **Curso:** DevOps · **Capítulo:** 02 · **Prerequisitos:** Docker
> **Código:** `src/kubernetes.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`planned`

## Intención

Este capítulo explicará Kubernetes como contrato de estado deseado: pods,
deployments, services, config maps, secrets, ingress, health checks y
reconciliación.

## Problema

Operar un contenedor aislado no basta cuando hay varias réplicas, fallas,
rollouts y comunicación entre servicios. Kubernetes introduce un modelo potente
pero costoso; el capítulo debe enseñar cuándo se justifica y cuándo no.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama del loop de reconciliación.
- Modelo Rust mínimo de estado deseado contra estado observado.
- Ejemplos progresivos y pruebas.
- Benchmarks o justificación explícita de no aplicabilidad.
