# Kubernetes

> **Curso:** DevOps · **Capítulo:** 02 · **Prerequisitos:** Docker
> **Código:** `src/kubernetes.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`draft`

## Concepto

Kubernetes se estudia aquí como un **sistema de reconciliación de estado
deseado**. El usuario declara cómo quiere que exista una aplicación: cuántas
réplicas, qué imagen ejecutan, qué puertos exponen, qué configuración reciben y
qué señales indican salud. El cluster observa la realidad y ejecuta acciones
para acercarla a esa declaración.

La unidad mental no es el archivo YAML. La unidad mental es este ciclo:

1. declarar intención;
2. observar estado real;
3. comparar diferencias;
4. reconciliar;
5. repetir mientras el sistema viva.

Kubernetes es útil porque convierte operación repetible en control loops. Es
peligroso cuando se adopta antes de necesitar esa complejidad.

## Problema

Un contenedor individual resuelve reproducibilidad de ejecución, pero no
resuelve por sí solo preguntas de producción:

- ¿qué pasa si el proceso muere?
- ¿cómo se reemplaza una réplica sin cortar todo el tráfico?
- ¿cómo se descubre un servicio desde otro?
- ¿cómo se aplica configuración sin reconstruir la imagen?
- ¿cómo se separa estado deseado de estado observado?
- ¿cómo se sabe si una instancia está lista para recibir tráfico?

Kubernetes aparece cuando administrar manualmente esas respuestas se vuelve más
costoso y riesgoso que adoptar un orquestador.

## Alternativas

### Docker Compose

Excelente para desarrollo local, demos y stacks pequeños. Permite declarar
servicios relacionados sin operar un cluster completo.

Ventaja: simple, cercano al flujo de desarrollo.  
Costo: no modela toda la operación de producción: scheduling, reconciliación,
rollouts, tolerancia a fallas y políticas de cluster.

### Scripts de despliegue

Un equipo puede automatizar `ssh`, `docker pull`, `docker stop` y `docker run`
con scripts propios.

Ventaja: control directo y poca infraestructura inicial.  
Costo: la lógica de reconciliación, rollback y recuperación queda repartida en
scripts difíciles de auditar.

### PaaS administrado

Plataformas como Heroku, Fly.io o servicios administrados equivalentes ocultan
parte de la orquestación.

Ventaja: menor carga operativa para equipos pequeños.  
Costo: menos control del modelo de ejecución y mayor dependencia del proveedor.

### Kubernetes

Introduce objetos declarativos, control plane, scheduler, controllers,
services, health checks y mecanismos de rollout.

Ventaja: modelo uniforme para operar aplicaciones distribuidas.  
Costo: curva de aprendizaje, superficie operativa amplia y riesgo de usar una
plataforma compleja para problemas simples.

## Justificación

Este capítulo viene después de Docker porque Kubernetes no reemplaza el contrato
de ejecución: lo orquesta. Si la imagen está mal diseñada, Kubernetes solo
automatiza el problema con más maquinaria alrededor.

La decisión pedagógica es enseñar Kubernetes desde el ciclo de reconciliación,
no desde comandos. Los manifiestos son evidencia de una intención; el concepto
duradero es el sistema que compara intención contra realidad y actúa.

## Invariantes del capítulo

Una carga de trabajo explicada en este capítulo debe declarar:

- **Imagen versionada:** no se despliega `latest` como contrato de producción.
- **Réplicas explícitas:** el número deseado de instancias no queda implícito.
- **Puertos claros:** los contenedores exponen puertos coherentes con el
  servicio que los enruta.
- **Readiness:** una réplica no recibe tráfico antes de estar lista.
- **Liveness:** una réplica puede reiniciarse si queda en estado roto.
- **Configuración externa:** ConfigMaps y Secrets no se hornean en la imagen.
- **Límites de recursos:** CPU y memoria se declaran para evitar competencia
  invisible dentro del cluster.
- **Rollout reversible:** el cambio debe poder pausarse, observarse o revertirse.

## Fronteras con otros cursos

- `rust-cloud` enseña infraestructura de plataforma: cómputo, redes, IAM,
  almacenamiento y servicios gestionados.
- `rust-devops` enseña la operación sobre esa plataforma: orquestación,
  despliegues, señales y recuperación.
- `rust-distributed-systems` profundiza en consenso, relojes, gossip y fallas
  distribuidas. Kubernetes usa ideas distribuidas, pero este capítulo se queda
  en el modelo operativo.
- `rust-security` profundizará en hardening; aquí solo se cubren mínimos de
  configuración segura.

## Fuera de alcance en este issue

Este issue no implementa todavía el modelo Rust, no agrega ejemplos completos y
no entrega ejercicios. Esos pasos viven en los issues siguientes del milestone
`02. Kubernetes`.
