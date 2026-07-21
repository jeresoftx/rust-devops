# Benchmarks

Los benchmarks se agregan cuando un capítulo tenga costo observable.

En DevOps no todo se mide con microbenchmarks de Rust. Cuando el costo pertenezca
a latencia de despliegue, tiempo de recuperación, volumen de telemetría o
presupuesto de almacenamiento, el capítulo debe explicar la métrica y declarar
si `cargo bench` aplica o no.

## Docker

`docker_baseline.rs` mide el costo de validar contratos educativos de Docker.
No mide el daemon ni el build real de imágenes. Para operación real, el capítulo
documenta métricas más relevantes: tamaño de imagen, tiempo de build, tiempo de
pull, tiempo de arranque y calidad de señales operativas.

## Kubernetes

`kubernetes_baseline.rs` mide el costo de reconciliar especificaciones
educativas contra estado observado. No mide API server, scheduler, etcd ni
controllers reales.

Para operación real, el capítulo documenta métricas más relevantes: duración de
rollout, tiempo a readiness, eventos de scheduling, capacidad disponible,
latencia del control plane y frecuencia de reinicios.

## Pipelines de CI/CD

`cicd_baseline.rs` mide el costo de evaluar pipelines educativos contra reglas
de promoción. No mide runners, colas, red, registros de imágenes ni plataformas
reales de CI/CD.

Para operación real, el capítulo documenta métricas más relevantes: duración
total, tiempo en cola, cache hit rate, etapa más lenta, tasa de fallos por
etapa, tiempo de recuperación y frecuencia de reintentos manuales.

## Estrategias de despliegue

`deployment_strategies_baseline.rs` mide el costo de evaluar estrategias
educativas de exposición, señales y rollback. No mide balanceadores,
orquestadores, feature flag services ni rollouts reales.

Para operación real, el capítulo documenta métricas más relevantes: duración
del rollout, tiempo de detección, tiempo de rollback, porcentaje de tráfico
expuesto, tasa de error durante la promoción y número de usuarios afectados.

## Gestión de releases

`release_management_baseline.rs` mide el costo de evaluar planes educativos de
versión, artefactos, changelog, notas, rollback y comunicación. No mide GitHub
Releases, generación real de changelog, firmas ni publicación de paquetes.

Para operación real, el capítulo documenta métricas más relevantes: tiempo de
release, artefactos trazables, releases con rollback documentado, cambios
incompatibles detectados antes de publicar y tiempo para identificar la última
versión segura.
