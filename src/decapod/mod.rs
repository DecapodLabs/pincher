pub mod broker;
pub mod capabilities;
pub mod cli;
pub mod commitment;
pub mod coordination;
pub mod docs;
pub mod governance;
pub mod rpc;
pub mod session;
pub mod todo;
pub mod validate;
pub mod workspace;
pub mod workunit;

pub use anyhow::Error as DecapodError;
pub use anyhow::Result;

use cli::DecapodCli;
use session::Session;
use std::process::Stdio;
use tokio::process::Command;

pub struct Decapod {
    cli: DecapodCli,
    session: Option<Session>,
}

impl Decapod {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cli: DecapodCli::new()?,
            session: None,
        })
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Pincher agent engine starting...");
        
        if let Some(ref session) = self.session {
            tracing::info!("Session active: {}", session.token);
        }

        Ok(())
    }

    pub fn cli(&self) -> &DecapodCli {
        &self.cli
    }

    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }
}

async fn run_decapod_command(args: &[&str]) -> Result<String> {
    let output = Command::new("decapod")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow::anyhow!(
            "decapod command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub fn find_project_root() -> Option<std::path::PathBuf> {
    let mut path = std::env::current_dir().ok()?;
    
    loop {
        if path.join(".decapod").exists() {
            return Some(path);
        }
        if !path.pop() {
            return None;
        }
    }
}
