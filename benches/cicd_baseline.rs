use std::time::Instant;

use rust_devops::cicd::{
    BuildArtifact, Environment, PipelineSpec, PipelineStage, PipelineTrigger, StageKind,
    StageStatus, evaluate_pipeline,
};

fn pull_request_pipeline() -> PipelineSpec {
    PipelineSpec::new(PipelineTrigger::PullRequest, Environment::Development)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
        .with_stage(PipelineStage::required("cargo test", StageKind::Test))
        .with_stage(PipelineStage::required("cargo build", StageKind::Build))
}

fn blocked_staging_pipeline() -> PipelineSpec {
    PipelineSpec::new(PipelineTrigger::PushToMain, Environment::Staging)
        .with_stage(PipelineStage::required("cargo fmt", StageKind::Format))
        .with_stage(PipelineStage::required("cargo clippy", StageKind::Lint))
        .with_stage(
            PipelineStage::required("cargo test", StageKind::Test).with_status(StageStatus::Failed),
        )
        .with_stage(PipelineStage::required("cargo build", StageKind::Build))
        .with_stage(PipelineStage::required("docker build", StageKind::Package))
        .with_stage(PipelineStage::required("deploy staging", StageKind::Deploy))
        .with_artifact(BuildArtifact::new("booking-api", "1.0.0", "abc1234"))
}

fn production_pipeline() -> PipelineSpec {
    PipelineSpec::new(PipelineTrigger::ManualPromotion, Environment::Production)
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
        .with_artifact(BuildArtifact::new("booking-api", "1.0.0", "abc1234"))
        .with_rollback_plan()
}

fn main() {
    let pull_request = pull_request_pipeline();
    let staging = blocked_staging_pipeline();
    let production = production_pipeline();
    let iterations = 100_000;
    let started = Instant::now();
    let mut promotable = 0;
    let mut total_findings = 0;

    for _ in 0..iterations {
        for pipeline in [&pull_request, &staging, &production] {
            let evaluation = evaluate_pipeline(pipeline);
            if evaluation.can_promote {
                promotable += 1;
            }
            total_findings += evaluation.findings.len();
        }
    }

    assert_eq!(promotable, iterations * 2);
    assert_eq!(total_findings, iterations);

    println!(
        "evaluated {iterations} cicd pipeline triples in {:?}",
        started.elapsed()
    );
}
