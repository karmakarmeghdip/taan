use std::{sync::Arc, time::Duration};

use http_cache_reqwest::{CACacheManager, CacheMode, CacheOptions, HttpCache, HttpCacheOptions};
use librespot_core::{
    Error, Session, SessionConfig, SpotifyId, authentication::Credentials, cache::Cache,
};
use librespot_playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::NoOpVolume,
    player::Player,
};
use rspotify::{
    AuthCodeSpotify, ClientError,
    http::HttpError,
    model::{PlaylistId, PlaylistItem, SimplifiedPlaylist},
    prelude::{BaseClient, OAuthClient},
};

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

#[derive(Clone)]
pub struct SpotifyService {
    session: Session,
    pub player: Arc<Player>,
    client: Arc<AuthCodeSpotify>,
}
impl Default for SpotifyService {
    fn default() -> SpotifyService {
        let path = robius_directories::ProjectDirs::from("com", "meghdip", "taan")
            .expect("Failed to get project directories, fatal");
        let cache = Cache::new(
            Some(path.cache_dir()),
            Some(path.cache_dir()),
            Some(&path.cache_dir().join("audio_cache")),
            None,
        )
        .expect("Failed to initialise cache, fatal");
        let session = Session::new(SessionConfig::default(), Some(cache));
        let player = Player::new(
            PlayerConfig::default(),
            session.clone(),
            Box::new(NoOpVolume),
            || {
                audio_backend::find(None).expect("Failed to initialise audio backend, fatal")(
                    None,
                    AudioFormat::default(),
                )
            },
        );
        let mut client = AuthCodeSpotify::default().with_middleware_arc(Arc::new(
            http_cache_reqwest::Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::new(path.cache_dir().join("http_cache"), false),
                options: HttpCacheOptions {
                    cache_options: Some(CacheOptions {
                        shared: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            }),
        ));
        client.config.token_refreshing = false;
        SpotifyService {
            session,
            player,
            client: Arc::new(client),
        }
    }
}
impl SpotifyService {
    pub async fn init(&self) -> anyhow::Result<()> {
        let creds = self
            .session
            .cache()
            .ok_or(Error::unauthenticated("No cache in session"))?
            .credentials()
            .ok_or(Error::unauthenticated("No cache in session"))?;
        self.session.connect(creds, true).await?;
        Ok(())
    }
    pub async fn connect(&self, creds: Credentials) -> anyhow::Result<()> {
        self.session.connect(creds, true).await?;
        self.web_auth().await?;
        Ok(())
    }

    pub async fn web_auth(&self) -> anyhow::Result<()> {
        let token = self.session.login5().auth_token().await?;

        let rtoken = rspotify::Token {
            access_token: token.access_token,
            expires_in: chrono::TimeDelta::from_std(token.expires_in)
                .expect("Invalid expiry, fatal"),
            scopes: token.scopes.into_iter().collect(),
            ..Default::default()
        };

        *self.client.token.lock().await.unwrap() = Some(rtoken);
        Ok(())
    }

    pub async fn get_me(&self) -> Result<rspotify::model::PrivateUser, Error> {
        self.client
            .current_user()
            .await
            .map_err(Error::unauthenticated)
    }

    pub fn is_connected(&self) -> bool {
        self.session.username().is_empty()
    }

    pub async fn get_user_playlists(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<SimplifiedPlaylist>, Error> {
        loop {
            match self
                .client
                .current_user_playlists_manual(Some(limit), Some(offset))
                .await
            {
                Ok(playlists) => {
                    break Ok(playlists.items);
                }
                Err(e) => {
                    if self.requires_refresh(e).await {
                        continue;
                    }
                    break Err(Error::unauthenticated("Failed to refresh client"));
                }
            }
        }
    }

    pub async fn get_playlist(
        &self,
        id: PlaylistId<'_>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<PlaylistItem>, Error> {
        loop {
            match self
                .client
                .playlist_items_manual(id.clone(), None, None, Some(limit), Some(offset))
                .await
            {
                Ok(tracks) => {
                    break Ok(tracks.items);
                }
                Err(e) => {
                    if self.requires_refresh(e).await {
                        continue;
                    }
                    break Err(Error::unauthenticated("Failed to refresh client"));
                }
            }
        }
    }

    pub fn load_track(&self, id: String) -> Result<(), Error> {
        let track_id = SpotifyId::from_uri(&id)?;
        self.player.load(track_id, false, 0);
        println!("Loaded track {}", id);
        Ok(())
    }

    pub async fn auth(&self) -> Result<(), Error> {
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
        self.connect(c)
            .await
            .map_err(|e| Error::unauthenticated(format!("Failed to authenticate: {}", e)))?;
        Ok(())
    }
    async fn requires_refresh(&self, e: ClientError) -> bool {
        if let ClientError::Http(e) = e {
            if let HttpError::StatusCode(res) = *e {
                if res.status() == 401 {
                    self.web_auth().await.unwrap_or_else(|e| {
                        eprintln!("Failed to refresh client: {}", e);
                    });
                    return true;
                }
                if res.status() == 429 {
                    let wait = res.headers().get("Retry-After").and_then(|v| {
                        v.to_str()
                            .expect("Failed to convert header to string, fatal")
                            .parse::<u64>()
                            .ok()
                    });
                    println!("rate limit hit, waiting for {}", wait.unwrap_or_default());
                    tokio::time::sleep(Duration::from_secs(wait.unwrap_or_default())).await;
                    return true;
                }
            }
        }
        false
    }
}
