use rust_devops::release_management::{
    ChangeKind, ReleaseArtifact, ReleaseFinding, ReleasePlan, SemanticVersion, VersionBump,
    evaluate_release,
};

fn main() {
    let healthy_release =
        ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
            .with_change(ChangeKind::Feature)
            .with_artifact(ReleaseArtifact::new("booking-api", "abc1234", "v1.1.0"))
            .with_release_notes()
            .with_changelog()
            .with_rollback_plan()
            .with_communication_channel();

    let healthy_evaluation = evaluate_release(&healthy_release);
    assert!(healthy_evaluation.ready);
    assert!(healthy_evaluation.findings.is_empty());

    let risky_release =
        ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
            .with_change(ChangeKind::BreakingChange)
            .with_artifact(ReleaseArtifact::new("booking-api", "def5678", "v1.1.0"))
            .with_release_notes()
            .with_changelog()
            .with_rollback_plan()
            .with_communication_channel();

    let risky_evaluation = evaluate_release(&risky_release);
    assert!(!risky_evaluation.ready);
    assert!(
        risky_evaluation
            .findings
            .contains(&ReleaseFinding::IncorrectVersionBump {
                expected: VersionBump::Major,
                actual: Some(VersionBump::Minor),
            },)
    );
}
