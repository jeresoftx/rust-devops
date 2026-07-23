use std::path::Path;

const MANIFEST: &str = include_str!("../course.manifest.json");

#[test]
fn manifest_status_matches_current_course_state() {
    assert_eq!(
        MANIFEST.matches("\"status\": \"benchmarked\"").count(),
        11,
        "el curso y sus 10 capítulos deben estar en estado benchmarked"
    );
    assert!(
        !MANIFEST.contains("\"status\": \"planned\""),
        "el manifiesto ya no debe declarar contenido planeado"
    );
    assert!(
        !MANIFEST.contains("\"status\": \"reviewed\"")
            && !MANIFEST.contains("\"status\": \"published\""),
        "la revisión humana sigue siendo requisito antes de reviewed/published"
    );
}

#[test]
fn manifest_paths_point_to_existing_files() {
    for line in MANIFEST.lines() {
        let trimmed = line.trim();
        if !declares_course_artifact_path(trimmed) {
            continue;
        }

        let path = trimmed
            .split_once(": ")
            .map(|(_, value)| value)
            .expect("la ruta del manifiesto debe tener formato JSON simple")
            .trim_end_matches(',')
            .trim_matches('"');

        assert!(
            Path::new(path).exists(),
            "el manifiesto apunta a una ruta inexistente: {path}"
        );
    }
}

fn declares_course_artifact_path(line: &str) -> bool {
    [
        "\"document\"",
        "\"module\"",
        "\"example\"",
        "\"tests\"",
        "\"bench\"",
        "\"diagram\"",
    ]
    .iter()
    .any(|key| line.starts_with(key))
}
