use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub session_id: String,
    pub expires_at: Option<String>,
    pub created_at: String,
}

impl Session {
    pub async fn acquire(password: &str) -> anyhow::Result<Self> {
        let output = Command::new("decapod")
            .args(["session", "acquire"])
            .env("DECAPOD_SESSION_PASSWORD", password)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "session acquire failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        if let Ok(session) = serde_json::from_str::<Session>(&output_str) {
            return Ok(session);
        }

        for line in output_str.lines() {
            if let Ok(session) = serde_json::from_str::<Session>(line) {
                return Ok(session);
            }
        }

        Err(anyhow::anyhow!("failed to parse session response"))
    }

    pub async fn validate(&self) -> anyhow::Result<bool> {
        let output = Command::new("decapod")
            .args(["session", "validate"])
            .env("DECAPOD_SESSION_PASSWORD", &self.token)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        Ok(output.status.success())
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = &self.expires_at {
            if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(expires_at) {
                return expires < chrono::Utc::now();
            }
        }
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub password: Option<String>,
    pub ttl_secs: Option<u64>,
    pub persist_path: Option<PathBuf>,
}

impl SessionConfig {
    pub fn new() -> Self {
        Self {
            password: std::env::var("DECAPOD_SESSION_PASSWORD").ok(),
            ttl_secs: std::env::var("DECAPOD_SESSION_TTL_SECS")
                .ok()
                .and_then(|s| s.parse().ok()),
            persist_path: dirs::data_dir().map(|p| p.join("pincher").join("session.json")),
        }
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn with_ttl(mut self, ttl_secs: u64) -> Self {
        self.ttl_secs = Some(ttl_secs);
        self
    }

    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    pub fn ttl_secs(&self) -> Option<u64> {
        self.ttl_secs
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_session_password() -> Option<String> {
    std::env::var("DECAPOD_SESSION_PASSWORD").ok()
}

pub fn set_session_password(password: &str) {
    unsafe { std::env::set_var("DECAPOD_SESSION_PASSWORD", password) };
}

pub fn clear_session_password() {
    unsafe { std::env::remove_var("DECAPOD_SESSION_PASSWORD") };
}
