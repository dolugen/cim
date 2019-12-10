mod api;
use api::{summarize_jobs, Job};

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_summarize_jobs() {
        let jobs = vec![
            Job {
                id: 1,
                name: String::from("Build"),
                stage: String::from("Build"),
                status: String::from("success")
            },
            Job {
                id: 2,
                name: String::from("Test 1"),
                stage: String::from("Test"),
                status: String::from("running")
            },
            Job {
                id: 3,
                name: String::from("Test 2"),
                stage: String::from("Test"),
                status: String::from("failed")
            }
        ];
        let summary = summarize_jobs(jobs);
        assert_eq!(summary, "1 failed, 1 running, 1 success");
    }
}
