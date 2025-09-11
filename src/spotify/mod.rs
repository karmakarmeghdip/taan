use librespot_core::{Error, authentication::Credentials, cache::Cache};

const CACHE: &str = ".cache";
const CACHE_FILES: &str = ".cache/files";

pub const SPOTIFY_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";

static OAUTH_SCOPES: &[&str] = &[
    "playlist-modify",
    "playlist-modify-private",
    "playlist-modify-public",
    "playlist-read",
    "playlist-read-collaborative",
    "playlist-read-private",
    "streaming",
    "user-follow-modify",
    "user-follow-read",
    "user-library-modify",
    "user-library-read",
    "user-modify",
    "user-modify-playback-state",
    "user-modify-private",
    "user-personalized",
    "user-read-currently-playing",
    "user-read-email",
    "user-read-play-history",
    "user-read-playback-position",
    "user-read-playback-state",
    "user-read-private",
    "user-read-recently-played",
    "user-top-read",
];

#[derive(Default)]
pub struct SpotifyState {
    cache: Option<Cache>,
    creds: Option<Credentials>,
}
impl SpotifyState {
    pub fn init(&mut self) -> Result<(), Error> {
        let cache = Cache::new(Some(CACHE), Some(CACHE), Some(CACHE_FILES), None)?;
        self.cache = Some(cache);
        if let Some(c) = &self.cache {
            let creds = c
                .credentials()
                .ok_or(Error::unavailable("Credentials not cached"))?;
            self.creds = Some(creds);
        }
        Ok(())
    }

    pub fn get_username(&self) -> Result<String, Error> {
        Ok("Meghdip".to_string())
    }

    pub fn get_cache(&mut self) -> Option<&Cache> {
        self.cache.as_ref()
    }

    pub fn set_creds(&mut self, creds: Option<Credentials>) {
        self.creds = creds;
    }

    pub fn is_logged_in(&self) -> bool {
        self.creds.is_some()
    }
}

pub async fn auth(cache: Option<Cache>) -> Result<Credentials, Error> {
    let c = librespot_oauth::OAuthClientBuilder::new(
        SPOTIFY_CLIENT_ID,
        "http://127.0.0.1:8898/login",
        OAUTH_SCOPES.to_vec(),
    )
    .open_in_browser()
    .build()
    .map_err(|e| Error::unauthenticated(format!("Failed to run oAuth {}", e)))?
    .get_access_token_async()
    .await
    .map(|t| Credentials::with_access_token(t.access_token))
    .map_err(|e| Error::unauthenticated(format!("Failed to authenticate: {}", e)))?;
    if let Some(cache) = cache {
        cache.save_credentials(&c)
    };
    Ok(c)
}
