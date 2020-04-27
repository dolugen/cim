# cim

A simple command line tool to show summaries of your Gitlab pipelines.

## Usage

To get details of the pipeline jobs, please set the `GITLAB_PRIVATE_TOKEN` environment variable.
You can get your access token from https://gitlab.com/profile/personal_access_tokens (or your Gitlab instance's respective page).

Sample output:

```
$ cim
ID         URL                                                       CREATED_AT                STATUS   REF                              JOBS
140267762  https://gitlab.com/gitlab-org/gitlab/pipelines/140267762  2020-04-27T14:15:58.045Z  running  refs/merge-requests/30417/merge  2 manual, 9 created, 9 running
140267555  https://gitlab.com/gitlab-org/gitlab/pipelines/140267555  2020-04-27T14:15:22.322Z  running  refs/merge-requests/28331/head   2 manual, 9 created, 9 running
140267467  https://gitlab.com/gitlab-org/gitlab/pipelines/140267467  2020-04-27T14:15:16.545Z  running  master                           18 created, 2 running
```
