# Gestión de releases

> **Curso:** DevOps · **Capítulo:** 05 · **Prerequisitos:** Pipelines de CI/CD
> **Código:** `src/release_management.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`draft`

## Concepto

Gestionar releases es convertir cambios técnicos en **versiones entendibles,
trazables y comunicables**. Un release no es solo un tag; es una decisión de
publicación con versión, alcance, riesgos, compatibilidad, notas, artefactos y
plan de reversión.

La unidad mental del capítulo es:

1. agrupar cambios que forman una versión;
2. clasificar el impacto;
3. decidir el incremento de versión;
4. producir artefactos y tags trazables;
5. comunicar qué cambió y quién queda afectado;
6. conservar una salida si el release degrada el sistema.

Un release bien gestionado permite que otros equipos entiendan qué ocurrió sin
leer todo el diff.

## Problema

Un cambio técnicamente correcto puede ser operacionalmente peligroso si nadie
entiende qué cambió, cómo revertirlo o qué consumidores quedan afectados.

El problema real aparece cuando los releases son una mezcla de commits, tags,
mensajes de chat y memoria humana. En ese contexto, un bug no solo rompe el
sistema: también rompe la capacidad de responder rápido. Nadie sabe si el
cambio era breaking, qué migración aplicaba, qué feature flag lo activó o cuál
era la última versión segura.

Release management reduce esa ambigüedad. Su trabajo no es burocracia: es
memoria operativa verificable.

## Alternativas

### Publicar sin versionar

Cada merge o deploy se considera "la versión actual".

Ventaja: simple para prototipos.
Costo: baja trazabilidad y rollback confuso.

### Tags manuales

Una persona crea tags cuando considera que hay una versión.

Ventaja: introduce puntos de referencia.
Costo: puede faltar criterio consistente, changelog o relación con artefactos.

### Semantic Versioning

La versión comunica compatibilidad: major, minor y patch.

Ventaja: lenguaje compartido para consumidores.
Costo: exige disciplina para clasificar impacto real.

### Changelog y release notes

El equipo mantiene un registro legible de cambios.

Ventaja: comunica intención, riesgos y migraciones.
Costo: si se hace tarde o con plantillas vacías, se vuelve ruido.

### Release trains

Los cambios se agrupan en ventanas de release regulares.

Ventaja: cadencia predecible.
Costo: puede retrasar fixes urgentes o juntar demasiado riesgo.

## Justificación

Este capítulo viene después de estrategias de despliegue porque desplegar una
versión no es lo mismo que gestionarla. La estrategia decide exposición; release
management decide identidad, compatibilidad y comunicación del cambio.

La decisión pedagógica es enseñar releases como contrato operativo: qué cambió,
qué significa, cómo se consume y qué hacer si falla. Las herramientas pueden
automatizar tags o notas, pero no sustituyen el criterio de clasificación.

## Invariantes del capítulo

Un release explicado aquí debe declarar:

- **Versión:** número o identificador estable.
- **Artefactos:** binario, imagen, paquete o manifiesto asociado.
- **Cambios incluidos:** features, fixes, breaking changes y migraciones.
- **Compatibilidad:** impacto hacia consumidores existentes.
- **Notas de release:** texto entendible para humanos.
- **Changelog:** historial acumulativo.
- **Tag trazable:** referencia a commit o artefacto.
- **Plan de rollback:** versión o acción segura de regreso.
- **Canal de comunicación:** dónde se avisa y a quién.
- **Criterio de aprobación:** quién decide que el release está listo.

## Fronteras con otros capítulos

- Pipelines de CI/CD producen evidencia y artefactos.
- Estrategias de despliegue controlan exposición y rollback operacional.
- Gestión de releases controla identidad, compatibilidad y comunicación.
- Observabilidad valida el comportamiento posterior al release.
- Operación regulada agregará auditoría, aprobaciones formales y retención.

## Fuera de alcance en este issue

Este issue no agrega todavía el modelo Rust, ejemplos completos, diagrama final
ni ejercicios. Esos pasos viven en los issues siguientes del milestone
`05. Gestión de releases`.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama del ciclo versión-release-comunicación.
- Modelo Rust mínimo de compatibilidad y riesgo de cambio.
- Ejemplos progresivos y pruebas.
- Métricas o justificación explícita de no aplicabilidad.
