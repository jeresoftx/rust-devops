use rust_devops::cicd::{
    BuildArtifact, Environment, PipelineFinding, PipelineSpec, PipelineStage, PipelineTrigger,
    StageKind, StageStatus, evaluate_pipeline,
};

fn main() {
    let pipeline = PipelineSpec::new(PipelineTrigger::PushToMain, Environment::Staging)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
        .with_stage(
            PipelineStage::required("cargo test", StageKind::Test).with_status(StageStatus::Failed),
        )
        .with_stage(PipelineStage::required("cargo build", StageKind::Build))
        .with_stage(PipelineStage::required("docker build", StageKind::Package))
        .with_stage(PipelineStage::required("deploy staging", StageKind::Deploy))
        .with_artifact(BuildArtifact::new("billing-api", "1.0.0", "abc1234"));

    let evaluation = evaluate_pipeline(&pipeline);

    assert!(!evaluation.can_promote);
    assert_eq!(
        evaluation.findings,
        vec![PipelineFinding::FailedRequiredStage("cargo test")]
    );
}
