use std::path::PathBuf;
use crate::g::G;
use crate::result::Result;

impl G {
    pub async fn root(&self) -> Result<PathBuf> {
        let output = self.run(vec!["rev-parse".to_string(), "--show-toplevel".to_string()]).await?;

        let stdout = String::from_utf8(output.stdout)?;
        Ok(PathBuf::from(stdout.trim()))
    }

    pub async fn is_root(&self) -> Result<bool> {
        // simply check if there's .git directory
        let d = self.dir.join(".git");
        Ok(d.try_exists()?)
    }
}

async fn get_root() -> Result<PathBuf> {
    let g = G::default();
    g.root().await
}

async fn is_root() -> Result<bool> {
    let g = G::default();
    g.is_root().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_root() {
        let root = get_root().await.unwrap();
        println!("{}", root.display());
    }

    #[tokio::test]
    async fn test_is_root() {
        let is_root = is_root().await.unwrap();
        println!("{}", is_root);
    }
}
