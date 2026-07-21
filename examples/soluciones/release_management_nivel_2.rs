use rust_devops::release_management::{
    ChangeKind, ReleaseArtifact, ReleaseFinding, ReleasePlan, SemanticVersion, VersionBump,
    evaluate_release,
};

fn main() {
    let release = ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
        .with_change(ChangeKind::BreakingChange)
        .with_artifact(ReleaseArtifact::new("booking-api", "def5678", "v1.1.0"))
        .with_release_notes()
        .with_changelog()
        .with_rollback_plan()
        .with_communication_channel();

    let evaluation = evaluate_release(&release);

    assert!(!evaluation.ready);
    assert!(
        evaluation
            .findings
            .contains(&ReleaseFinding::IncorrectVersionBump {
                expected: VersionBump::Major,
                actual: Some(VersionBump::Minor),
            })
    );
}
