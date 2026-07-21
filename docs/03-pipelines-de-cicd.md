# Pipelines de CI/CD

> **Curso:** DevOps · **Capítulo:** 03 · **Prerequisitos:** Git, Docker
> **Código:** `src/cicd.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`planned`

## Intención

Este capítulo explicará un pipeline como una cadena de evidencias antes de
cambiar producción: formato, lint, pruebas, build, seguridad, artefactos y
promoción entre ambientes.

## Problema

Sin automatización, cada release depende de memoria humana. El pipeline reduce
riesgo, pero también puede volverse lento, frágil o ceremonial si no se diseña
con propósito.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama del flujo commit-artefacto-release.
- Modelo Rust mínimo de etapas, gates y resultados.
- Ejemplos progresivos y pruebas.
- Métricas de duración o justificación de no aplicabilidad.
