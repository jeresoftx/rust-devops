# Introducción

DevOps es el curso complementario que enseña cómo un sistema cambia, se libera,
se observa y se repara cuando vive en producción.

## Concepto

DevOps no es un puesto ni una lista de herramientas. Es una disciplina de
operación: reducir el riesgo de los cambios, automatizar lo repetible, hacer
visible el estado real del sistema y responder con calma cuando algo falla.

## Problema

El material de DevOps suele envejecer rápido cuando se limita a copiar comandos
de Docker, manifiestos de Kubernetes o pantallas de una plataforma. Jeresoft
Academy necesita un curso que enseñe los fundamentos operativos que sobreviven
a las herramientas: empaquetado, estado deseado, verificación, despliegue,
observabilidad, alertas, retención y cumplimiento.

## Alternativas

- Enseñar herramientas primero y explicar los principios después.
- Enseñar solo cultura DevOps sin construir nada ejecutable.
- Enseñar operación como disciplina, usando herramientas concretas cuando
  vuelven visible el principio.

## Justificación

Se adopta la tercera alternativa porque conserva la frontera con `rust-cloud` y
`rust-software-architecture`. Cloud enseña dónde corre el software; arquitectura
enseña cómo se organiza; DevOps enseña cómo se opera sin perder trazabilidad.

## Frontera con SRE

SRE aparece dentro del curso donde ayuda a medir confiabilidad: SLOs, SLIs,
error budgets, alertas útiles y respuesta a incidentes. El foco del repo sigue
siendo el ciclo operativo completo: del cambio al despliegue, del despliegue a
la observación, y de la observación a la reparación.

## Navegación del curso

El curso se recorre como una secuencia operativa, no como una lista suelta de
herramientas:

1. **Fundamentos operativos:** Docker, Kubernetes, CI/CD, despliegues,
   releases y observabilidad.
2. **Confiabilidad y cumplimiento:** stack Grafana, alertas, SLOs, retención y
   operación en dominios regulados.

Cada capítulo declara su estado editorial, enlaza su modelo Rust, ejemplo,
diagrama, benchmark y soluciones cuando existen. Los enlaces de anterior y
siguiente al final de cada archivo ayudan a leer el material como libro y a
mantener `docs/SUMMARY.md` sincronizado con el flujo real.

Este material está preparado para revisión humana, pero ningún capítulo está
marcado como `reviewed` ni `published`.

---

[Siguiente: 01. Docker](01-docker.md)
