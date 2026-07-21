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
