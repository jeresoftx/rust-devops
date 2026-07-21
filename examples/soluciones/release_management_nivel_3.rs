use rust_devops::release_management::{
    ChangeKind, ReleaseArtifact, ReleaseFinding, ReleasePlan, SemanticVersion, VersionBump,
    evaluate_release,
};

fn main() {
    let unsafe_release =
        ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 0, 0))
            .with_change(ChangeKind::Fix)
            .with_artifact(ReleaseArtifact::new("booking-api", "abc", ""));
    let unsafe_evaluation = evaluate_release(&unsafe_release);

    assert!(!unsafe_evaluation.ready);
    assert_eq!(
        unsafe_evaluation.findings,
        vec![
            ReleaseFinding::VersionDoesNotAdvance,
            ReleaseFinding::IncorrectVersionBump {
                expected: VersionBump::Patch,
                actual: None,
            },
            ReleaseFinding::NonTraceableArtifact("booking-api"),
            ReleaseFinding::MissingReleaseNotes,
            ReleaseFinding::MissingChangelog,
            ReleaseFinding::MissingRollbackPlan,
            ReleaseFinding::MissingCommunicationChannel,
        ]
    );

    let hardened_release =
        ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 0, 1))
            .with_change(ChangeKind::Fix)
            .with_artifact(ReleaseArtifact::new("booking-api", "abc1234", "v1.0.1"))
            .with_release_notes()
            .with_changelog()
            .with_rollback_plan()
            .with_communication_channel();
    let hardened_evaluation = evaluate_release(&hardened_release);

    assert!(hardened_evaluation.ready);
    assert!(hardened_evaluation.findings.is_empty());
}
