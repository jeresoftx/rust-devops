use rust_devops::cicd::{
    BuildArtifact, Environment, PipelineFinding, PipelineSpec, PipelineStage, PipelineTrigger,
    StageKind, StageStatus, evaluate_pipeline,
};

fn main() {
    let pull_request = PipelineSpec::new(PipelineTrigger::PullRequest, Environment::Development)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format).taking_seconds(8))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint).taking_seconds(22))
        .with_stage(PipelineStage::required("cargo test", StageKind::Test).taking_seconds(40))
        .with_stage(PipelineStage::required("cargo build", StageKind::Build).taking_seconds(18));

    let pr_evaluation = evaluate_pipeline(&pull_request);
    assert!(pr_evaluation.can_promote);
    assert!(pr_evaluation.findings.is_empty());

    let staging = PipelineSpec::new(PipelineTrigger::PushToMain, Environment::Staging)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
        .with_stage(
            PipelineStage::required("cargo test", StageKind::Test).with_status(StageStatus::Failed),
        )
        .with_stage(PipelineStage::required("cargo build", StageKind::Build))
        .with_stage(PipelineStage::required("docker build", StageKind::Package))
        .with_stage(PipelineStage::required("deploy staging", StageKind::Deploy))
        .with_artifact(BuildArtifact::new("booking-api", "1.0.0", "abc1234"));

    let staging_evaluation = evaluate_pipeline(&staging);
    assert!(!staging_evaluation.can_promote);
    assert!(
        staging_evaluation
            .findings
            .contains(&PipelineFinding::FailedRequiredStage("cargo test"))
    );
}
