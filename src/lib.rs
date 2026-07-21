//! Fundamentos ejecutables del curso `rust-devops`.
//!
//! Este crate empieza pequeño por diseño: el repositorio primero establece la
//! gobernanza, el plan y el contrato educativo. Los capítulos posteriores
//! agregarán módulos por concepto operativo conforme a RFC-0001 §14.

pub mod docker;
pub mod kubernetes;

/// Nombre público del curso dentro de Jeresoft Academy.
pub const COURSE_NAME: &str = "DevOps";

/// Identificador estable del repositorio/curso.
pub const COURSE_SLUG: &str = "rust-devops";

/// Estado editorial de un capítulo del curso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChapterStatus {
    /// El capítulo existe en el plan, pero aún no tiene implementación.
    Planned,
    /// El capítulo tiene una primera especificación editorial.
    Draft,
    /// El capítulo ya expone un modelo Rust mínimo verificable.
    Implemented,
    /// El capítulo tiene mediciones o justificación explícita de costos.
    Benchmarked,
}

/// Describe un capítulo planeado del curso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chapter {
    /// Número humano del capítulo.
    pub number: u8,
    /// Título editorial del capítulo.
    pub title: &'static str,
    /// Estado actual del capítulo.
    pub status: ChapterStatus,
}

/// Devuelve el mapa inicial de capítulos decidido por RFC-0001 §10.
///
/// # Examples
///
/// ```
/// let chapters = rust_devops::planned_chapters();
/// assert_eq!(chapters.len(), 10);
/// assert_eq!(chapters[0].title, "Docker");
/// ```
pub fn planned_chapters() -> &'static [Chapter] {
    &PLANNED_CHAPTERS
}

const PLANNED_CHAPTERS: [Chapter; 10] = [
    Chapter {
        number: 1,
        title: "Docker",
        status: ChapterStatus::Benchmarked,
    },
    Chapter {
        number: 2,
        title: "Kubernetes",
        status: ChapterStatus::Implemented,
    },
    Chapter {
        number: 3,
        title: "Pipelines de CI/CD",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 4,
        title: "Estrategias de despliegue",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 5,
        title: "Gestión de releases",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 6,
        title: "Observabilidad",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 7,
        title: "Stack Grafana",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 8,
        title: "Alertas, SLOs y SLIs",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 9,
        title: "Retención de telemetría",
        status: ChapterStatus::Planned,
    },
    Chapter {
        number: 10,
        title: "Operación en dominios regulados",
        status: ChapterStatus::Planned,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_course_identity() {
        assert_eq!(COURSE_NAME, "DevOps");
        assert_eq!(COURSE_SLUG, "rust-devops");
    }

    #[test]
    fn exposes_ten_planned_chapters() {
        let chapters = planned_chapters();

        assert_eq!(chapters.len(), 10);
        assert_eq!(chapters[0].title, "Docker");
        assert_eq!(chapters[9].title, "Operación en dominios regulados");
        assert!(
            chapters
                .iter()
                .skip(2)
                .all(|chapter| chapter.status == ChapterStatus::Planned)
        );
        assert_eq!(chapters[0].status, ChapterStatus::Benchmarked);
        assert_eq!(chapters[1].status, ChapterStatus::Implemented);
    }
}
