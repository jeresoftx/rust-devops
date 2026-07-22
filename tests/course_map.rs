use rust_devops::{COURSE_NAME, COURSE_SLUG, ChapterStatus, planned_chapters};

#[test]
fn course_identity_is_stable() {
    assert_eq!(COURSE_NAME, "DevOps");
    assert_eq!(COURSE_SLUG, "rust-devops");
}

#[test]
fn roadmap_starts_with_operational_foundations() {
    let chapters = planned_chapters();

    assert_eq!(chapters.len(), 10);
    assert_eq!(chapters[0].title, "Docker");
    assert_eq!(chapters[1].title, "Kubernetes");
    assert_eq!(chapters[2].title, "Pipelines de CI/CD");
    assert_eq!(chapters[6].status, ChapterStatus::Benchmarked);
    assert_eq!(chapters[7].status, ChapterStatus::Benchmarked);
    assert!(
        chapters
            .iter()
            .skip(8)
            .all(|chapter| chapter.status == ChapterStatus::Planned)
    );
    assert_eq!(chapters[0].status, ChapterStatus::Benchmarked);
    assert_eq!(chapters[1].status, ChapterStatus::Benchmarked);
    assert_eq!(chapters[2].status, ChapterStatus::Benchmarked);
    assert_eq!(chapters[3].status, ChapterStatus::Benchmarked);
    assert_eq!(chapters[4].status, ChapterStatus::Benchmarked);
    assert_eq!(chapters[5].status, ChapterStatus::Benchmarked);
}
