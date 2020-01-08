use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// pipelines: GET /projects/:id/pipelines
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineSummary {
    pub id: u32,
    pub sha: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub web_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PipelineUser {
    name: String,
    username: String,
    id: i32,
    state: String,
    avatar_url: String,
    web_url: String,
}

// pipeline detail: GET /projects/:id/pipelines/:pipeline_id
#[derive(Debug, Serialize, Deserialize)]
struct PipelineDetail {
    id: i32,
    status: String,
    #[serde(rename = "ref")]
    ref_: String,
    sha: String,
    before_sha: String,
    tag: bool,
    // yaml_errors:
    user: PipelineUser,
    created_at: String,
    updated_at: String,
    // started_at:
    finished_at: String,
    // commited_at:
    // duration:
    // coverage:
    web_url: String, // detailed_status:
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub stage: String,
    // commit
    // coverage
    // allow_failure: bool,
    // created_at: String,
    // started_at: String,
    // finished_at: Option<String>,
    // duration: f32,
    // artifacts_expire_at: String,
    // pipeline:
    // ref: String,
    // artifacts:
    // runner
    // tag: bool,
    // web_url: String,
    // user:
}

pub fn summarize_jobs(jobs: Vec<Job>) -> String {
    let mut counter = HashMap::new();
    for job in jobs {
        let count = counter.entry(job.status).or_insert(0);
        *count += 1;
    }
    let mut summary = vec![];
    for (status, count) in counter {
        summary.push(format!("{} {}", count, status));
    }
    summary.sort();
    summary.join(", ")
}
