# Pipelines de CI/CD

> **Curso:** DevOps · **Capítulo:** 03 · **Prerequisitos:** Git, Docker
> **Código:** `src/cicd.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`implemented`

## Concepto

Un pipeline de CI/CD es una **cadena de evidencias antes de cambiar un sistema
vivo**. Cada etapa responde una pregunta concreta: si el código tiene formato,
si compila, si pasa pruebas, si produce un artefacto reproducible, si cumple
controles mínimos y si puede promoverse a otro ambiente.

El pipeline no existe para "hacer deploy automático" como fin en sí mismo.
Existe para reducir incertidumbre entre un cambio en Git y una modificación en
producción.

La unidad mental no es el archivo YAML de GitHub Actions, GitLab CI o Jenkins.
La unidad mental es este ciclo:

1. recibir un cambio;
2. construir evidencia;
3. detener el cambio si falta una señal obligatoria;
4. producir un artefacto trazable;
5. promover solo cuando el riesgo aceptado sea explícito;
6. registrar qué se cambió, cuándo y con qué resultado.

## Problema

Sin automatización, cada release depende de memoria humana: recordar comandos,
correr pruebas, construir artefactos, copiar versiones y avisar al equipo. Esa
memoria falla justo cuando hay presión.

Pero automatizar sin criterio también crea problemas:

- pipelines lentos que nadie respeta;
- checks verdes que no prueban lo importante;
- etapas duplicadas que solo dan sensación de seguridad;
- secretos mal manejados;
- artefactos imposibles de rastrear;
- despliegues que avanzan sin gates adecuados;
- ambientes que no se parecen entre sí.

El problema real es diseñar una cadena mínima y suficiente de señales para
cambiar software con trazabilidad.

## Alternativas

### Release manual

Una persona ejecuta comandos localmente o desde un servidor compartido.

Ventaja: simple al inicio y fácil de entender.
Costo: baja trazabilidad, alto riesgo de omisiones y dependencia directa de una
persona.

### Scripts de automatización

El equipo crea scripts para compilar, probar, empaquetar o desplegar.

Ventaja: mejora repetibilidad sin adoptar una plataforma completa.
Costo: si no se integran con revisión, logs, permisos y artefactos, los scripts
siguen siendo operación manual con otro nombre.

### Plataforma de CI/CD

GitHub Actions, GitLab CI, Jenkins, Buildkite, CircleCI u otra plataforma
ejecutan workflows por evento.

Ventaja: historial, permisos, paralelismo, integración con pull requests y
ambientes.
Costo: dependencia de la plataforma, sintaxis específica y riesgo de pipelines
opacos o demasiado acoplados.

### GitOps

El estado deseado de despliegue vive en Git y un reconciliador aplica cambios al
ambiente.

Ventaja: trazabilidad fuerte y reconciliación declarativa.
Costo: agrega piezas operativas y requiere disciplina en separación de
artefacto, configuración y ambiente.

## Justificación

Este capítulo viene después de Docker y Kubernetes porque CI/CD conecta ambos:
construye artefactos reproducibles y decide cuándo promoverlos hacia un entorno
orquestado.

La decisión pedagógica es enseñar pipelines como diseño de evidencia, no como
recetas de YAML. Las herramientas cambian; el criterio de qué verificar, cuándo
bloquear y qué registrar dura más.

## Invariantes del capítulo

Un pipeline explicado en este capítulo debe declarar:

- **Evento de entrada:** push, pull request, tag, release manual o promoción.
- **Etapas explícitas:** formato, lint, pruebas, build, seguridad, artefacto y
  despliegue si aplica.
- **Gates obligatorios:** condiciones que bloquean promoción.
- **Artefacto trazable:** versión, commit y metadatos suficientes para auditar.
- **Separación de ambientes:** desarrollo, staging y producción no se mezclan.
- **Secretos fuera del código:** credenciales gestionadas por la plataforma.
- **Observabilidad del pipeline:** duración, fallas, etapa fallida y actor.
- **Rollback o recuperación:** el cambio debe tener una salida razonable.

## Fronteras con otros cursos

- `rust-software-architecture` decide límites, módulos y contratos internos.
- `rust-cloud` enseña infraestructura base: cuentas, redes, IAM, cómputo y
  servicios administrados.
- `rust-devops` enseña cómo verificar, empaquetar, promover, observar y reparar
  cambios.
- `rust-security` profundiza en threat modeling, supply chain y gestión de
  secretos; aquí se cubren controles mínimos para no enseñar malas prácticas.
- `rust-distributed-systems` profundiza en fallas distribuidas; aquí se mira el
  pipeline como sistema operativo de entrega.

## Fuera de alcance en este issue

Este issue no agrega todavía ejemplos completos, diagrama final ni ejercicios.
Esos pasos viven en los issues siguientes del milestone
`03. Pipelines de CI/CD`. El modelo Rust mínimo ya vive en `src/cicd.rs`.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama del flujo commit-artefacto-release.
- Modelo Rust mínimo de etapas, gates y resultados.
- Ejemplos progresivos y pruebas.
- Métricas de duración o justificación de no aplicabilidad.
