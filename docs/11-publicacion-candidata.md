# Publicación candidata

## Estado

`candidata-para-revisión`

Este documento registra el cierre técnico del curso `rust-devops` antes de la
revisión humana. No convierte ningún capítulo en `reviewed` ni `published`.

## Alcance verificado

La publicación candidata incluye:

- 10 capítulos educativos en estado `benchmarked`;
- navegación mdBook en `docs/SUMMARY.md`;
- enlaces anterior/siguiente en la introducción y capítulos;
- modelos Rust para cada capítulo;
- ejemplos ejecutables por capítulo;
- soluciones ejecutables de niveles 1 a 3;
- benchmarks educativos por capítulo;
- diagramas Mermaid por capítulo;
- manifiesto de curso para ingestión posterior en `academy-web`;
- pruebas unitarias, doctests, integración y verificación del manifiesto.

## Validación local requerida

Antes de pedir revisión humana, el repo debe pasar:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --all-targets
git diff --check
```

## Resultado de esta candidata

La candidata documentada por el issue #43 se preparó después de ejecutar la
suite completa localmente:

| Verificación | Resultado |
|--------------|-----------|
| `cargo fmt --check` | verde |
| `cargo clippy --all-targets --all-features -- -D warnings` | verde |
| `cargo test --all-targets` | verde |
| `cargo test --doc` | verde |
| `cargo bench --all-targets` | verde |
| `git diff --check` | verde |

GitHub no reportó checks remotos obligatorios para los PRs autónomos de este
bloque. La evidencia de validación vive en los cuerpos de los PRs fusionados y
en esta página de cierre.

## Frontera editorial

Esta candidata todavía requiere revisión humana de Joel para:

- aprobar lectura completa y consistencia pedagógica;
- decidir si algún capítulo puede pasar a `reviewed`;
- decidir cuándo una versión puede considerarse `published`;
- revisar ortografía, tono y coherencia con RFC-0001;
- preparar la ingestión final hacia `academy-web`.

La regla rectora sigue intacta: la IA acelera, el criterio humano decide.

---

[Anterior: 10. Operación en dominios regulados](10-operacion-en-dominios-regulados.md)
