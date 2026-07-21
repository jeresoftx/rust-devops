# Docker

> **Curso:** DevOps · **Capítulo:** 01 · **Prerequisitos:** fundamentos de línea de comandos y procesos
> **Código:** `src/docker.rs` · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Estado

`draft`

## Concepto

Docker se estudia aquí como un **contrato de ejecución reproducible**. Ese
contrato responde una pregunta concreta: si un servicio necesita archivos,
variables de entorno, puertos, usuario, proceso principal, sistema de archivos
y límites de recursos, ¿cómo se describe todo eso para que pueda ejecutarse de
forma consistente fuera de la laptop original?

La unidad central no es el comando `docker run`, sino la relación entre tres
piezas:

- **Imagen:** artefacto inmutable que empaqueta filesystem, metadata y punto de
  entrada.
- **Contenedor:** proceso aislado creado desde una imagen, con configuración de
  ejecución.
- **Registro:** lugar donde se publica y recupera la imagen que otros ambientes
  ejecutarán.

El capítulo no trata Docker como magia de infraestructura. Lo trata como una
frontera explícita entre construir software y operarlo.

## Problema

Un sistema que solo funciona en la máquina de quien lo desarrolló no es
operable. La frase "en mi máquina sí funciona" casi siempre es síntoma de que
el contrato de ejecución está escondido en estado local: una versión de runtime,
un archivo presente por accidente, una variable de entorno olvidada, una
dependencia instalada manualmente o un comando que nadie documentó.

Docker reduce esa ambigüedad, pero no elimina el criterio. Una mala imagen
puede ser enorme, insegura, lenta de construir, difícil de cachear o incapaz de
recibir señales correctamente. Enseñar Docker solo como lista de comandos
produce usuarios de herramienta; enseñar sus invariantes forma operadores de
sistemas.

## Alternativas

### Ejecutar directamente en el host

Es la alternativa más simple. El binario o runtime vive en la máquina destino y
el proceso se administra con herramientas del sistema operativo.

Ventaja: menos capas y menos abstracción.  
Costo: el ambiente queda implícito y es más fácil que desarrollo, CI y
producción diverjan.

### Máquinas virtuales

Una VM empaqueta un sistema operativo completo. Sirve cuando se necesita
aislamiento fuerte o kernels distintos.

Ventaja: frontera más amplia y familiar para infraestructura clásica.  
Costo: arranque más lento, mayor consumo y menor granularidad para servicios
pequeños.

### Contenedores OCI

Docker popularizó esta forma de empaquetar procesos aislados. Hoy el estándar
operativo relevante es OCI: imágenes y runtimes compatibles, no una sola
herramienta.

Ventaja: buen equilibrio entre reproducibilidad, portabilidad y costo
operativo.  
Costo: sigue compartiendo kernel con el host y requiere disciplina de seguridad,
capas, secretos y ciclo de vida.

### Plataformas PaaS o serverless

Algunas plataformas esconden el contenedor o lo convierten en detalle interno.

Ventaja: menos operación directa para el equipo.  
Costo: menor control del contrato de ejecución y dependencia de las decisiones
del proveedor.

## Justificación

Este curso empieza por Docker porque es la bisagra entre desarrollo y
operación. Antes de hablar de Kubernetes, pipelines, despliegues o
observabilidad, el estudiante necesita entender qué se está construyendo,
moviendo y ejecutando.

La decisión pedagógica es enseñar primero el modelo mental:

1. Una imagen debe ser reproducible.
2. Un contenedor debe comportarse como un proceso bien definido.
3. La configuración de ejecución debe ser explícita.
4. Los datos persistentes no deben confundirse con el filesystem efímero del
   contenedor.
5. La seguridad empieza en la imagen, no en producción.

Los comandos aparecen después, como consecuencia del modelo.

## Invariantes del capítulo

Un artefacto de ejecución enseñado en este capítulo debe cumplir estas reglas:

- **Reproducibilidad:** la misma fuente debe producir una imagen equivalente sin
  depender de archivos locales no declarados.
- **Inmutabilidad:** una imagen publicada no se modifica; se reemplaza por otra
  versión.
- **Proceso principal claro:** el contenedor ejecuta un proceso principal que
  recibe señales y termina de forma controlada.
- **Configuración externa:** lo que cambia por ambiente se declara como
  configuración, no se hornea dentro de la imagen.
- **Datos fuera del contenedor:** el estado duradero vive en volúmenes, bases de
  datos o servicios externos.
- **Mínimo privilegio:** la imagen evita permisos innecesarios, secretos
  incluidos y superficie de ataque accidental.
- **Observabilidad básica:** stdout/stderr son parte del contrato operativo; un
  contenedor silencioso ante fallas es difícil de operar.

## Fronteras con otros cursos

- `rust-cloud` enseña plataformas: cómputo, red, identidad, almacenamiento,
  servicios manejados y costos.
- `rust-software-architecture` enseña cómo se organiza el sistema y sus
  fronteras internas.
- `rust-devops` enseña cómo se empaqueta, libera, observa y repara ese sistema
  cuando vive fuera de la máquina de desarrollo.
- `rust-security` profundizará en hardening ofensivo/defensivo; aquí solo se
  cubren prácticas mínimas de imagen segura.

## Fuera de alcance en este issue

Este issue no implementa todavía el modelo Rust, no agrega ejemplos completos y
no entrega ejercicios. Esos pasos viven en los issues siguientes del milestone
`01. Docker`.
