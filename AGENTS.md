# AGENTS.md

Este repositorio es parte de los cursos complementarios de Jeresoft Academy y
se rige por la RFC-0001, el manual fundacional del ecosistema.

## Objetivo

Crear el mejor recurso educativo posible sobre DevOps: entrega continua,
contenedores, orquestación, despliegues, observabilidad, operación confiable y
prácticas de producción.

Todo cambio debe mejorar simultáneamente:

- calidad técnica
- claridad
- documentación
- mantenibilidad

## Antes de escribir código

Siempre, en este orden (RFC-0001 §2 y §13):

1. Explicar el concepto.
2. Explicar el problema.
3. Comparar alternativas.
4. Justificar la implementación.

## Código

Conforme a RFC-0001 §13:

- Rust idiomático.
- Clippy limpio y rustfmt sin diffs.
- Sin `unsafe` salvo justificación documentada y revisión humana explícita.
- Comentarios solo donde aporten valor.
- Ninguna dependencia externa sin justificación escrita.

## Documentación

Todo capítulo sigue RFC-0001 §14 y toda funcionalidad nueva incluye:

- README o ROADMAP actualizados si cambia el estado del curso.
- Diagramas Mermaid cuando ayuden a razonar.
- Ejemplos ejecutables.
- Tests.
- Benchmarks si hay costo observable; si no aplica, se declara.

## GitHub

Antes de tocar código de curso, el plan completo debe existir como milestones e
issues de GitHub:

- cada issue asignado a `jeresoftx`;
- cada issue con milestone y labels;
- 1 issue, 1 commit principal, 1 PR;
- cada PR asignado a `jeresoftx`, asociado al milestone del issue y con labels;
- no fusionar PR sin revisión humana, salvo autorización explícita de modo
  autónomo con revisión diferida.

Si se usa GitHub Project, debe estar asociado al repositorio, contener todos los
issues accionables y tener su vista principal agrupada por `Milestone`. No
basta con que los issues tengan milestone: la vista principal del Project debe
mostrar la agrupación activa. Si una herramienta no puede configurarla, el
agente debe pedir la intervención necesaria antes de declarar completo el
andamiaje de GitHub.

## Modo autónomo con revisión diferida

Cuando Joel lo autorice explícitamente para este repo o un bloque de trabajo,
la IA puede fusionar sus propios PRs solo si cumple todas las condiciones de
RFC-0001 §20:

- issue existente, asignado, etiquetado y con milestone;
- PR de un solo issue y un solo commit principal;
- verificaciones aplicables en verde;
- cambio dentro del plan aprobado;
- sin `unsafe`;
- sin dependencias externas no triviales;
- sin marcar capítulos como `reviewed` ni `published`;
- resumen del PR declarando revisión diferida.

La revisión humana no desaparece: se mueve al cierre del bloque.

## Nunca

- Agregar dependencias innecesarias.
- Optimizar prematuramente.
- Duplicar código.
- Omitir documentación.
- Convertir DevOps en una colección de comandos sin criterio operativo.
- Publicar capítulos parciales.

## Filosofía

Este repositorio debe poder utilizarse como un libro de ingeniería. Nunca
sacrificar claridad por ingenio. Explicar el porqué, no solo el cómo.
