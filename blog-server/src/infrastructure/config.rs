use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) database_url: String,
    pub(crate) host: String,
    pub(crate) port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct JwtConfig {
    pub(crate) secret: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CorsConfig {
    pub(crate) origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()?;
        Ok(Self {
            database_url,
            host,
            port,
        })
    }
}

impl JwtConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let secret = std::env::var("JWT_SECRET")?;
        Ok(Self { secret })
    }
}

impl CorsConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "*".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(Self {
            origins: cors_origins,
        })
    }
}
