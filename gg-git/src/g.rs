use std::env;
use std::path::{PathBuf};
use tokio::process::Command;
use crate::result::{Error, Result};

pub struct G {
    pub(crate) dir: PathBuf,
}

impl G {
    pub fn new(dir: &str) -> G {
        G { dir: PathBuf::from(dir) }
    }

    // pub fn new_with_env(dir: &str, envs: Vec<(String, String)>) -> G {
    //     G { dir: PathBuf::from(dir) }
    // }
}

impl Default for G {
    fn default() -> Self {
        G { dir: env::current_dir().unwrap() }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SuccessOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl G {
    pub async fn run(&self, args: Vec<String>) -> Result<SuccessOutput> {
        let output = Command::new("git")
            .current_dir(&self.dir)
            .args(args)
            .output()
            .await?;

        if output.status.success() {
            Ok(SuccessOutput {
                stdout: output.stdout,
                stderr: output.stderr,
            })
        } else {
            Err(Error::Exit(output))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run() {
        let g = G::default();
        let output = g.run(vec!["--version".to_string()]).await.unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("{}", stdout);
        assert!(stdout.len() > 0);
    }

    #[tokio::test]
    async fn test_run_fail() {
        let g = G::default();
        let output = g.run(vec!["--bad-option".to_string()]).await;
        assert!(output.is_err());
        match output.err().unwrap() {
            Error::Exit(output) => {
                let stderr = String::from_utf8(output.stderr).unwrap();
                println!("{}", stderr);
                assert!(stderr.len() > 0);
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[tokio::test]
    async fn test_run_in_not_exist_dir() {
        let g = G::new("/not/exist/dir");
        let output = g.run(vec!["--version".to_string()]).await;
        assert!(output.is_err());

        match output.err().unwrap() {
            Error::IO(_) => {}
            _ => panic!("unexpected error type"),
        }
    }
}

