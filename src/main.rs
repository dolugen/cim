extern crate reqwest;

mod api;
use api::{Job, PipelineSummary, summarize_jobs};

use std::env;
use reqwest::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("GITLAB_PRIVATE_TOKEN").unwrap();

    let api_url = "https://gitlab.com/api/v4/";
    let project_id = "278964";
    let pipelines_url = format!("{}projects/{}/pipelines/", api_url, project_id);

    assert_eq!(pipelines_url, "https://gitlab.com/api/v4/projects/278964/pipelines/");

    let client = reqwest::Client::new();
    let resp = client.get(Url::parse(pipelines_url.as_str())?)
        .query(&[("per_page", "3")])
        .send()?
        .text()?;

    let pipelines: Vec<PipelineSummary> = serde_json::from_str(resp.as_str())?;
    println!("REF\tID\tCREATED AT\tSTATUS");
    for pipeline in pipelines {
        println!("{}\t{}\t{}\t{}", pipeline.ref_, pipeline.id, pipeline.created_at, pipeline.status);
        
        let job_url = format!("{}{}/jobs", pipelines_url.as_str(), pipeline.id);
        let job_resp = client.get(Url::parse(job_url.as_str())?)
            .header("PRIVATE-TOKEN", token.clone())
            .send()?
            .text()?;

        let jobs: Vec<Job> = serde_json::from_str(job_resp.as_str())?;
        let jobs_summary = summarize_jobs(jobs);
        println!("{}", jobs_summary);
    }

    Ok(())
}
