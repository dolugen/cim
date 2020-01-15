extern crate reqwest;

mod api;
use api::{summarize_jobs, Job, PipelineSummary};

use reqwest::Url;
use structopt::StructOpt;
use tabular::{Row, Table};

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Cli {
    #[structopt(short, long, default_value = "gitlab.com")]
    hostname: String,
    #[structopt(short = "p", long, default_value = "278964")]
    project_id: String,
    #[structopt(short = "t", long = "token", help = "Gitlab personal access token. Can also be set as an environment variable called GITLAB_PRIVATE_TOKEN. Get yours from https://gitlab.com/profile/personal_access_tokens", env, hide_env_values = true)]
    gitlab_private_token: String,
    #[structopt(short = "n", long = "pipelines-count", default_value = "3")]
    pipelines_count: u32,
    #[structopt(long, help = "Hide jobs summary for pipelines")]
    hide_jobs: bool,
    #[structopt(short, long)]
    verbose: bool,
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

    let mut row_format = String::from("{:<}  {:<}  {:<}  {:<}  {:<}");
    if !args.hide_jobs {
        row_format.push_str("  {:<}")
    }

    let mut table = Table::new(row_format.as_str());
    let mut heading = Row::new()
            .with_cell("ID")
            .with_cell("URL")
            .with_cell("CREATED_AT")
            .with_cell("STATUS")
            .with_cell("REF");
    if !args.hide_jobs {
        heading.add_cell("JOBS");
    }
    table.add_row(heading);

    for pipeline in pipelines {
        let mut row = Row::new()
                .with_cell(pipeline.id)
                .with_cell(pipeline.web_url)
                .with_cell(pipeline.created_at)
                .with_cell(pipeline.status)
                .with_cell(pipeline.ref_);

        if !args.hide_jobs {
            let job_url = format!("{}{}/jobs", pipelines_url.as_str(), pipeline.id);
            let job_resp = client
                .get(Url::parse(job_url.as_str())?)
                .header("PRIVATE-TOKEN", args.gitlab_private_token.clone())
                .send()?
                .text()?;

            let jobs: Vec<Job> = serde_json::from_str(job_resp.as_str())?;
            let jobs_summary = summarize_jobs(jobs);
            row.add_cell(jobs_summary);
        }
        table.add_row(row);
    }
    println!("{}", table);

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
