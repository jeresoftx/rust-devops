# Stack Grafana

> **Curso:** DevOps · **Capítulo:** 07 · **Prerequisitos:** Observabilidad
> **Código:** `src/grafana_stack.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`draft`

## Intención

Este capítulo aterrizará observabilidad en el stack Grafana: Grafana,
Prometheus, Loki, Tempo y Alloy, explicando el papel de cada pieza.

Stack Grafana no se estudia como una lista de productos. Se estudia como una
tubería operativa: un sistema emite señales, un agente las recolecta, un backend
las almacena con límites explícitos, una interfaz las consulta y una persona
toma una decisión.

El criterio central del capítulo es sencillo: una herramienta de observabilidad
vale si ayuda a responder una pregunta operativa con evidencia suficiente. Si
solo agrega pantallas, ruido o costo, no está observando el sistema; está
decorándolo.

## Problema

Nombrar herramientas no enseña observabilidad. El valor aparece cuando cada
componente se entiende como parte de una tubería de señales: capturar, etiquetar,
almacenar, consultar, correlacionar y visualizar.

El problema real aparece cuando el equipo ya decidió observar el sistema, pero
no sabe cómo conectar las piezas. Prometheus puede recolectar métricas, Loki
puede almacenar logs, Tempo puede conservar trazas, Alloy puede transportar
señales y Grafana puede consultar todo. Aun así, el resultado puede ser pobre:
dashboards que nadie usa, etiquetas de alta cardinalidad, logs imposibles de
buscar, trazas incompletas y costos de retención que crecen sin una pregunta
clara.

El stack no resuelve automáticamente la disciplina. Solo ofrece piezas. La
ingeniería está en decidir qué señal entra, con qué etiquetas, por cuánto
tiempo, para qué pregunta y con qué acción esperada.

## Concepto

Stack Grafana es una familia de herramientas que permite construir una ruta de
telemetría completa:

- **Alloy** recolecta, transforma y envía señales desde aplicaciones,
  servidores, contenedores o clústeres.
- **Prometheus** modela métricas como series temporales consultables.
- **Loki** modela logs como flujos etiquetados, evitando indexar cada palabra
  como si fuera un motor de búsqueda general.
- **Tempo** modela trazas distribuidas para seguir una petición entre
  componentes.
- **Grafana** consulta y visualiza las señales para responder preguntas
  operativas.

La idea importante no es memorizar nombres. La idea es entender la frontera de
cada componente: quién recolecta, quién almacena, quién consulta, quién
correlaciona y quién decide.

## Alternativas

Un equipo puede observar un sistema con varias familias de soluciones:

| Alternativa | Ventaja | Riesgo |
|-------------|---------|--------|
| Logs centralizados simples | Fácil de iniciar y suficiente para sistemas pequeños. | Se vuelve lento y caro si intenta responder todo solo con texto. |
| Métricas aisladas | Buen resumen temporal de salud y saturación. | Puede esconder causas si no se correlaciona con logs, trazas y cambios. |
| APM comercial administrado | Reduce operación del backend de observabilidad. | Puede ocultar costos, retención, muestreo y dependencia de proveedor. |
| OpenTelemetry con backend mixto | Da portabilidad conceptual y técnica. | Requiere disciplina para no producir telemetría sin propósito. |
| Stack Grafana | Integra métricas, logs, trazas y visualización con piezas explícitas. | Exige diseñar etiquetas, retención, consultas y dashboards con intención. |

Este capítulo usa Stack Grafana porque permite ver la tubería completa sin
ocultar las decisiones. No afirma que siempre sea la mejor solución. En equipos
pequeños, una plataforma administrada puede ser más prudente. En dominios
regulados, la retención, soberanía de datos y auditoría pueden pesar más que la
comodidad.

## Tradeoffs

El primer tradeoff es control contra operación. Ejecutar piezas propias da más
control sobre retención, etiquetas y costos, pero también agrega mantenimiento.
Consumir una plataforma administrada reduce operación, aunque puede volver
opacas algunas decisiones.

El segundo tradeoff es detalle contra costo. Más etiquetas, más trazas y más
logs pueden mejorar diagnóstico, pero también aumentan cardinalidad, volumen y
tiempo de búsqueda. La pregunta no es "¿podemos guardar más?", sino "¿qué
evidencia necesitamos para decidir mejor?".

El tercer tradeoff es cobertura contra ruido. Un dashboard con cincuenta
paneles no necesariamente informa mejor que uno con cinco señales accionables.
Un stack maduro separa exploración, monitoreo continuo, investigación de
incidentes y auditoría posterior.

## Invariantes

Una ruta de telemetría dentro del stack debe conservar estas invariantes:

- cada señal tiene dueño, servicio, ambiente y propósito visible;
- ninguna etiqueta tiene cardinalidad ilimitada por accidente;
- los logs conservan contexto suficiente sin convertirse en base de datos de
  negocio;
- las métricas responden preguntas agregadas y no sustituyen trazas;
- las trazas cruzan las fronteras importantes del flujo;
- la retención se decide por investigación, auditoría y costo, no por inercia;
- un dashboard debe responder una pregunta operativa concreta;
- una alerta no debe existir si nadie sabe qué acción tomar.

## Fronteras con cursos vecinos

`rust-cloud` enseña dónde corren los sistemas y qué servicios de plataforma
existen. Este capítulo no repite VPC, IAM, cómputo ni almacenamiento cloud,
salvo cuando ayudan a entender costo o despliegue de telemetría.

`rust-software-architecture` enseña cómo se organizan límites, componentes y
dependencias. Este capítulo asume esos límites y pregunta qué señales deben
cruzarlos para hacer el sistema operable.

El capítulo anterior, Observabilidad, define la disciplina: preguntas,
señales, contexto y acciones. Stack Grafana aterriza esa disciplina en una
tubería concreta.

SRE profundiza en objetivos de confiabilidad, error budgets y respuesta a
incidentes. Este capítulo prepara la infraestructura conceptual para ese
trabajo, pero no sustituye el diseño de SLOs ni el proceso humano de guardias.

## Entregables del capítulo

- Capítulo completo conforme a RFC-0001 §14.
- Diagrama de flujo de señales del stack.
- Modelo Rust mínimo de rutas de telemetría.
- Ejemplos progresivos y pruebas.
- Métricas de cardinalidad, volumen o justificación de no aplicabilidad.
