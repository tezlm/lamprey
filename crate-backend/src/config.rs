use std::collections::HashMap;

use ipnet::IpNet;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rust_log: String,
    pub database_url: String,
    pub api_url: Url,
    pub cdn_url: Url,
    /// for media/file uploads
    pub s3: ConfigS3,
    pub oauth_provider: HashMap<String, ConfigOauthProvider>,
    pub url_preview: ConfigUrlPreview,
    pub smtp: ConfigSmtp,
    pub otel_trace_endpoint: Option<String>,
    pub sfu_token: String,
    #[serde(default = "default_max_user_emails")]
    pub max_user_emails: usize,
    #[serde(default = "default_email_queue_workers")]
    pub email_queue_workers: usize,
    #[serde(default = "default_require_server_invite")]
    pub require_server_invite: bool,
}

fn default_require_server_invite() -> bool {
    true
}

fn default_max_user_emails() -> usize {
    50
}

fn default_email_queue_workers() -> usize {
    5
}

#[derive(Debug, Deserialize)]
pub struct ConfigS3 {
    pub bucket: String,
    pub endpoint: Url,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    // /// alternative host instead of going though the reverse proxy
    // pub local_endpoint: Option<Url>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigOauthProvider {
    pub client_id: String,
    pub client_secret: String,
    pub authorization_url: String,
    pub token_url: String,
    pub revocation_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigUrlPreview {
    pub user_agent: String,
    pub deny: Vec<IpNet>,
    pub max_parallel_jobs: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigSmtp {
    pub username: String,
    pub password: String,
    pub host: String,
    pub from: String,
}
