extern crate reqwest;

mod api;
use api::{summarize_jobs, Job, PipelineSummary};

use reqwest::Url;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("GITLAB_PRIVATE_TOKEN").unwrap();

    let api_url = "https://gitlab.com/api/v4/";
    let project_id = "278964";
    let pipelines_url = format!("{}projects/{}/pipelines/", api_url, project_id);

    assert_eq!(
        pipelines_url,
        "https://gitlab.com/api/v4/projects/278964/pipelines/"
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(Url::parse(pipelines_url.as_str())?)
        .query(&[("per_page", "3")])
        .send()?
        .text()?;

    let pipelines: Vec<PipelineSummary> = serde_json::from_str(resp.as_str())?;
    println!("REF\tID\tCREATED AT\tSTATUS");
    for pipeline in pipelines {
        println!(
            "{}\t{}\t{}\t{}",
            pipeline.ref_, pipeline.id, pipeline.created_at, pipeline.status
        );

        let job_url = format!("{}{}/jobs", pipelines_url.as_str(), pipeline.id);
        let job_resp = client
            .get(Url::parse(job_url.as_str())?)
            .header("PRIVATE-TOKEN", token.clone())
            .send()?
            .text()?;

        let jobs: Vec<Job> = serde_json::from_str(job_resp.as_str())?;
        let jobs_summary = summarize_jobs(jobs);
        println!("{}", jobs_summary);
    }

    Ok(())
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use api::{summarize_jobs, Job};


    #[test]
    fn test_summarize_jobs() {
        let jobs = vec![
            Job {
                id: 1,
                name: String::from("Build"),
                stage: String::from("Build"),
                status: String::from("success"),
            },
            Job {
                id: 2,
                name: String::from("Test 1"),
                stage: String::from("Test"),
                status: String::from("running"),
            },
            Job {
                id: 3,
                name: String::from("Test 2"),
                stage: String::from("Test"),
                status: String::from("failed"),
            },
        ];
        let summary = summarize_jobs(jobs);
        assert_eq!(summary, "1 failed, 1 running, 1 success");
    }
}

