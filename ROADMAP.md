# ROADMAP

Estado de avance de `rust-devops`, repositorio complementario de Jeresoft
Academy para DevOps en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado actual

El repositorio acaba de entrar en desarrollo. La estructura inicial declara la
frontera del curso, el mapa de capítulos, el contrato para `academy-web`, las
licencias, el crate Rust mínimo y el flujo de trabajo con GitHub.

El plan de trabajo vive en GitHub como milestones, issues, labels y un GitHub
Project asociado al repositorio. Cada paso accionable queda asignado a
`jeresoftx`, asociado al milestone correspondiente y etiquetado para conservar
la regla del repositorio: un issue, un commit y un PR.

El GitHub Project del curso vive en
`https://github.com/users/jeresoftx/projects/11`. Debe permanecer asociado al
repositorio, contener todos los issues accionables y tener la vista principal
agrupada por `Milestone`. Esta agrupación es requisito de aceptación del
andamiaje de GitHub, no una recomendación visual.

Ningún capítulo está marcado como `reviewed` ni `published`, porque la revisión
humana de Joel sigue siendo obligatoria según RFC-0001 §20.

## Progresión del curso

DevOps se estudia como disciplina operativa, no como lista de herramientas. La
progresión esperada es:

1. **Empaquetar y ejecutar:** Docker como unidad reproducible de ejecución.
2. **Orquestar:** Kubernetes como contrato de estado deseado.
3. **Automatizar cambios:** CI/CD y verificaciones antes de producción.
4. **Liberar con control:** estrategias de despliegue, releases y feature
   flags.
5. **Observar:** logs, métricas y trazas como lectura del sistema vivo.
6. **Responder:** alertas, SLOs, retención y operación en dominios regulados.

## Capítulos planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Docker | planned |
| 02 | Kubernetes | planned |
| 03 | Pipelines de CI/CD | planned |
| 04 | Estrategias de despliegue | planned |
| 05 | Gestión de releases | planned |
| 06 | Observabilidad | planned |
| 07 | Stack Grafana | planned |
| 08 | Alertas, SLOs y SLIs | planned |
| 09 | Retención de telemetría | planned |
| 10 | Operación en dominios regulados | planned |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.
- El orden del curso sigue RFC-0001 §10: Cloud enseña plataforma; DevOps enseña
  operación.

## Fuera de alcance por ahora

- Convertir el curso en tutorial de consola de un proveedor cloud.
- Enseñar VPC, IAM, cómputo o almacenamiento como capítulos canónicos: eso vive
  en `rust-cloud`.
- Publicar recetas dependientes de versiones actuales sin fecha explícita y
  revisión humana.
- Agregar dependencias externas antes de justificar su valor educativo.
- Publicar capítulos parciales como si estuvieran completos.

## Siguiente paso natural

Trabajar el primer milestone, `01. Docker`, con el flujo completo:
especificación conceptual, modelo Rust mínimo, capítulo/diagrama/ejemplos,
ejercicios, soluciones y validación automatizada.
