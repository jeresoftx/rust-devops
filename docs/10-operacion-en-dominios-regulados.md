# Operación en dominios regulados

> **Curso:** DevOps · **Capítulo:** 10 · **Prerequisitos:** Observabilidad, releases y retención
> **Código:** `src/regulated_operations.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`planned`

## Intención

Este capítulo aplicará DevOps a dominios regulados: trazabilidad de
transacciones, auditoría, gestión de secretos, cumplimiento, segregación de
ambientes y respuesta a incidentes.

## Problema

En dominios sensibles no basta con que el sistema funcione. Debe explicar qué
ocurrió, quién cambió qué, con qué autorización, qué datos fueron afectados y
cómo se recupera evidencia sin romper privacidad ni seguridad.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama de auditoría y trazabilidad.
- Modelo Rust mínimo de evento auditable.
- Ejemplos progresivos y pruebas.
- Benchmarks, métricas o justificación explícita de no aplicabilidad.
