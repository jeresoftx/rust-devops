# Docker

> **Curso:** DevOps · **Capítulo:** 01 · **Prerequisitos:** fundamentos de línea de comandos y procesos
> **Código:** `src/docker.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`planned`

## Intención

Este capítulo explicará Docker como contrato de ejecución reproducible:
imagen, contenedor, capas, build context, filesystem, red, variables de entorno
y límites de recursos.

## Problema

Un sistema que solo funciona en la máquina de quien lo desarrolló no es
operable. Docker reduce esa distancia, pero introduce decisiones reales:
tamaño de imagen, seguridad, caché, secretos, puertos, volúmenes y proceso
principal.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama Mermaid del ciclo imagen-contenedor.
- Modelo Rust mínimo para razonar sobre artefactos de ejecución.
- Ejemplos progresivos y pruebas.
- Sección explícita de benchmarks o justificación de no aplicabilidad.
