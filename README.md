# Rust DevOps

Repositorio complementario de Jeresoft Academy para estudiar DevOps con
criterio de ingeniería. Conecta directamente con `rust-software-architecture`
y `rust-cloud`: arquitectura decide cómo se organiza el sistema, Cloud decide
dónde corre, y DevOps decide cómo se entrega, observa, repara y opera.

El objetivo no es memorizar comandos de Docker, Kubernetes o GitHub Actions. El
objetivo es aprender a razonar sobre operación: automatizar cambios, reducir
riesgo, observar sistemas vivos, responder a fallas y sostener software en
producción con trazabilidad.

## Qué contiene

- Capítulos en Markdown compatibles con publicación posterior.
- Modelos Rust pequeños para representar decisiones operativas.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks cuando una decisión tenga costo observable.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar en el camino

Este curso complementa el Semestre 5. Recibe fundamentos de
`rust-networking`, `rust-operating-systems`, `rust-distributed-systems`,
`rust-system-design`, `rust-software-architecture` y `rust-cloud`.

Alimenta `rust-ai-engineering`, `rust-travel`, dominios aplicados y cursos
como `software-engineering-handbook`, porque todo sistema serio necesita una
forma disciplinada de cambiar, liberar, observar y reparar.

## Capítulos planeados

| # | Capítulo | Módulo sugerido | Estado |
|---|----------|-----------------|--------|
| 01 | Docker | `src/docker.rs` | benchmarked |
| 02 | Kubernetes | `src/kubernetes.rs` | benchmarked |
| 03 | Pipelines de CI/CD | `src/cicd.rs` | benchmarked |
| 04 | Estrategias de despliegue | `src/deployment_strategies.rs` | benchmarked |
| 05 | Gestión de releases | `src/release_management.rs` | benchmarked |
| 06 | Observabilidad | `src/observability.rs` | benchmarked |
| 07 | Stack Grafana | `src/grafana_stack.rs` | planned |
| 08 | Alertas, SLOs y SLIs | `src/reliability_targets.rs` | planned |
| 09 | Retención de telemetría | `src/telemetry_retention.rs` | planned |
| 10 | Operación en dominios regulados | `src/regulated_operations.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

## Estructura

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
course.manifest.json
docs/
src/
examples/
tests/
benches/
diagrams/
assets/
```

## Cómo usarlo

Ejecutar pruebas:

```bash
cargo test
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `course.manifest.json` expone el mapa estructurado del curso para
  `academy-web`.
- `docs/SUMMARY.md` contiene la navegación inicial del curso.
- `docs/00-introduccion.md` define la frontera conceptual de DevOps frente a
  Cloud, arquitectura y SRE.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha
  límite.
- El GitHub Project del curso vive en
  `https://github.com/users/jeresoftx/projects/11`.
- Ese Project está asociado al repositorio, contiene todos los issues
  accionables y su vista principal está agrupada por `Milestone`.
- Antes de tocar código de curso, el plan completo debe existir como milestones
  e issues de GitHub.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
