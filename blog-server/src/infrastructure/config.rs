use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct AppConfig {
    pub(crate) database_url: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) secret: String,
    pub(crate) origins: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()?;
        let secret = std::env::var("JWT_SECRET")?;
        let origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "*".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(Self {
            database_url,
            host,
            port,
            secret,
            origins
            
        })
    }
}
