extern crate reqwest;

use serde::{Deserialize, Serialize};
use reqwest::Url;

// pipelines: GET /projects/:id/pipelines
#[derive(Debug, Serialize, Deserialize)]
struct PipelineSummary {
    id: i32,
    sha: String,
    #[serde(rename="ref")]
    ref_: String,
    status: String,
    created_at: String,
    updated_at: String,
    web_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PipelineUser {
    name: String,
    username: String,
    id: i32,
    state: String,
    avatar_url: String,
    web_url: String
}

// pipeline detail: GET /projects/:id/pipelines/:pipeline_id
#[derive(Debug, Serialize, Deserialize)]
struct PipelineDetail {
    id: i32,
    status: String,
    #[serde(rename="ref")]
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
    web_url: String
    // detailed_status:
}

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    id: i32,
    name: String,
    status: String,
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
    stage: String,
    // tag: bool,
    // web_url: String,
    // user: 

}

// pipeline jobs: GET /projects/:id/pipelines/:pipeline_id/jobs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "";
    let api_url = Url::parse("https://gitlab.com/api/v4/")?;
    let project_id = "278964";
    let project_pipelines_url = api_url.join(format!("projects/{}/pipelines/", project_id).as_str())?;

    assert_eq!(project_pipelines_url.as_str(), "https://gitlab.com/api/v4/projects/278964/pipelines/");
    println!("{}", project_pipelines_url);

    let client = reqwest::Client::new();
    let resp = client.get(project_pipelines_url)
        .query(&[("per_page", "3")])
        .send()?
        .text()?;

    let pipelines: Vec<PipelineSummary> = serde_json::from_str(resp.as_str())?;
    println!("REF\tID\tCREATED AT\tSTATUS");
    for pipeline in pipelines {
        println!("{}\t{}\t{}\t{}", pipeline.ref_, pipeline.id, pipeline.created_at, pipeline.status);
        
        // let pipeline_jobs_url = Url::parse(
        //     format!("https://gitlab.com/api/v4/projects/278964/pipelines/{}/jobs", pipeline.id).as_str())?;
        
        // let job_resp = client.get(pipeline_jobs_url)
        //     .header("PRIVATE-TOKEN", token)
        //     .send()?
        //     .text()?;
        // // println!("{}", job_resp);
        // let jobs: Vec<Job> = serde_json::from_str(job_resp.as_str())?;
        // println!("job id\tstatus\tstage\tname");
        // for job in jobs {
        //     println!("{}\t{}\t{}\t{}", job.id, job.status, job.stage, job.name);
        // }
        // // println!("{}", job_resp);
        // println!();
    }

    Ok(())
}
