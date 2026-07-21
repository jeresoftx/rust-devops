use std::time::Instant;

use rust_devops::release_management::{
    ChangeKind, ReleaseArtifact, ReleasePlan, SemanticVersion, evaluate_release,
};

fn healthy_minor() -> ReleasePlan {
    ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
        .with_change(ChangeKind::Feature)
        .with_artifact(ReleaseArtifact::new("booking-api", "abc1234", "v1.1.0"))
        .with_release_notes()
        .with_changelog()
        .with_rollback_plan()
        .with_communication_channel()
}

fn breaking_minor() -> ReleasePlan {
    ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
        .with_change(ChangeKind::BreakingChange)
        .with_artifact(ReleaseArtifact::new("booking-api", "def5678", "v1.1.0"))
        .with_release_notes()
        .with_changelog()
        .with_rollback_plan()
        .with_communication_channel()
}

fn unsafe_release() -> ReleasePlan {
    ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 0, 0))
        .with_change(ChangeKind::Fix)
        .with_artifact(ReleaseArtifact::new("booking-api", "abc", ""))
}

fn main() {
    let healthy = healthy_minor();
    let breaking = breaking_minor();
    let unsafe_plan = unsafe_release();
    let iterations = 100_000;
    let started = Instant::now();
    let mut ready = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for release in [&healthy, &breaking, &unsafe_plan] {
            let evaluation = evaluate_release(release);
            if evaluation.ready {
                ready += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(ready, iterations);
    assert_eq!(total_findings, iterations * 8);

    println!(
        "evaluated {iterations} release management triples in {:?}",
        started.elapsed()
    );
}
