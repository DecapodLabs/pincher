pub mod agent;
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

pub use cli::DecapodCli;
pub use session::Session;

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
