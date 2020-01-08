extern crate reqwest;

mod api;
use api::{summarize_jobs, Job, PipelineSummary};

use reqwest::Url;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Keep an eye on your Gitlab pipelines")]
struct Cli {
    #[structopt(short, long, default_value = "gitlab.com")]
    hostname: String,
    #[structopt(short = "p", long, default_value = "278964")]
    project_id: String,
    #[structopt(short = "t", long = "token", env, hide_env_values = true)]
    gitlab_private_token: String,
    #[structopt(short = "n", long = "pipelines-count", default_value = "3")]
    pipelines_count: u32,
    #[structopt(long, help = "Hide jobs summary for pipelines")]
    hide_jobs: bool,
    #[structopt(short, long)]
    verbose: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    if args.verbose {
        println!("Running in verbose mode. Using host {}", args.hostname);
    }

    let api_url = format!("https://{}/api/v4/", args.hostname);
    let pipelines_url = format!("{}projects/{}/pipelines/", api_url, args.project_id);

    let client = reqwest::Client::new();
    let resp = client
        .get(Url::parse(pipelines_url.as_str())?)
        .query(&[("per_page", args.pipelines_count)])
        .send()?
        .text()?;

    let pipelines: Vec<PipelineSummary> = serde_json::from_str(resp.as_str())?;
    println!("ID\tURL\tCREATED AT\tSTATUS\tREF");
    for pipeline in pipelines {
        println!(
            "{}\t{}\t{}\t{}\t{}",
            pipeline.id, pipeline.web_url, pipeline.created_at, pipeline.status, pipeline.ref_
        );

        if !args.hide_jobs {
            let job_url = format!("{}{}/jobs", pipelines_url.as_str(), pipeline.id);
            let job_resp = client
                .get(Url::parse(job_url.as_str())?)
                .header("PRIVATE-TOKEN", args.gitlab_private_token.clone())
                .send()?
                .text()?;

            let jobs: Vec<Job> = serde_json::from_str(job_resp.as_str())?;
            let jobs_summary = summarize_jobs(jobs);
            println!("{}", jobs_summary);
        }
    }

    Ok(())
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
