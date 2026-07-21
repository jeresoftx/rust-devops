use rust_devops::release_management::{
    ChangeKind, ReleaseArtifact, ReleasePlan, SemanticVersion, evaluate_release,
};

fn main() {
    let release = ReleasePlan::new(SemanticVersion::new(1, 0, 0), SemanticVersion::new(1, 1, 0))
        .with_change(ChangeKind::Feature)
        .with_artifact(ReleaseArtifact::new("booking-api", "abc1234", "v1.1.0"))
        .with_release_notes()
        .with_changelog()
        .with_rollback_plan()
        .with_communication_channel();

    let evaluation = evaluate_release(&release);

    assert!(evaluation.ready);
    assert!(evaluation.findings.is_empty());
}
