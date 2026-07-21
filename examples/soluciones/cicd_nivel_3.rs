use rust_devops::cicd::{
    BuildArtifact, Environment, PipelineFinding, PipelineSpec, PipelineStage, PipelineTrigger,
    StageKind, evaluate_pipeline,
};

fn main() {
    let incomplete = PipelineSpec::new(PipelineTrigger::Tag, Environment::Production)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
        .with_stage(PipelineStage::required("cargo test", StageKind::Test))
        .with_stage(PipelineStage::required("cargo build", StageKind::Build))
        .with_artifact(BuildArtifact::new("billing-api", "latest", "abc1234"));

    let incomplete_evaluation = evaluate_pipeline(&incomplete);
    assert!(
        incomplete_evaluation
            .findings
            .contains(&PipelineFinding::MissingRequiredStage(
                StageKind::SecurityScan
            ))
    );
    assert!(
        incomplete_evaluation
            .findings
            .contains(&PipelineFinding::MissingProductionApproval)
    );
    assert!(
        incomplete_evaluation
            .findings
            .contains(&PipelineFinding::MissingRollbackPlan)
    );

    let hardened = PipelineSpec::new(PipelineTrigger::ManualPromotion, Environment::Production)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
        .with_stage(PipelineStage::required("cargo test", StageKind::Test))
        .with_stage(PipelineStage::required("cargo build", StageKind::Build))
        .with_stage(PipelineStage::required(
            "cargo audit",
            StageKind::SecurityScan,
        ))
        .with_stage(PipelineStage::required("docker build", StageKind::Package))
        .with_stage(PipelineStage::required(
            "release approval",
            StageKind::Approval,
        ))
        .with_stage(PipelineStage::required(
            "deploy production",
            StageKind::Deploy,
        ))
        .with_artifact(BuildArtifact::new("billing-api", "1.0.0", "abc1234"))
        .with_rollback_plan();

    let hardened_evaluation = evaluate_pipeline(&hardened);
    assert!(hardened_evaluation.can_promote);
    assert!(hardened_evaluation.findings.is_empty());
}
