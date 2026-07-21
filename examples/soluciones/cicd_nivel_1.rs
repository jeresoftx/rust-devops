use rust_devops::cicd::{
    Environment, PipelineSpec, PipelineStage, PipelineTrigger, StageKind, evaluate_pipeline,
};

fn main() {
    let pipeline = PipelineSpec::new(PipelineTrigger::PullRequest, Environment::Development)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
        .with_stage(PipelineStage::required("cargo test", StageKind::Test))
        .with_stage(PipelineStage::required("cargo build", StageKind::Build));

    let evaluation = evaluate_pipeline(&pipeline);

    assert!(evaluation.can_promote);
    assert!(evaluation.findings.is_empty());
}
