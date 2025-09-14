use std::sync::Arc;

use librespot_core::{Error, Session, SessionConfig, authentication::Credentials, cache::Cache};
use librespot_playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::NoOpVolume,
    player::Player,
};
use rspotify::{AuthCodeSpotify, ClientError, model::SimplifiedPlaylist, prelude::OAuthClient};

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

pub mod async_loop;

pub struct SpotifyState {
    session: Session,
    creds: Option<Credentials>,
    player: Arc<Player>,
    client: AuthCodeSpotify,
}

impl Default for SpotifyState {
    fn default() -> Self {
        let cache = Cache::new(Some(CACHE), Some(CACHE), Some(CACHE_FILES), None)
            .expect("Failed to initalise cache, fatal");
        let creds = cache.credentials();
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
        let mut client = AuthCodeSpotify::default();
        client.config.token_refreshing = false;
        SpotifyState {
            session,
            creds,
            player,
            client,
        }
    }
}
impl SpotifyState {
    pub async fn connect(&self) -> Result<(), Error> {
        self.session
            .connect(
                self.creds
                    .clone()
                    .ok_or(Error::unauthenticated("Creds not available"))?,
                true,
            )
            .await?;
        self.web_auth().await;
        Ok(())
    }

    pub async fn web_auth(&self) {
        let token = self
            .session
            .login5()
            .auth_token()
            .await
            .expect("Not Logged in");

        let rtoken = rspotify::Token {
            access_token: token.access_token,
            expires_in: chrono::TimeDelta::from_std(token.expires_in)
                .expect("Invalid expiry, fatal"),
            scopes: token.scopes.into_iter().collect(),
            ..Default::default()
        };

        *self.client.token.lock().await.unwrap() = Some(rtoken);
    }
    pub fn get_username(&self) -> String {
        self.session.username() // Replace with Rspotify user name
    }

    pub async fn get_me(&self) -> Result<rspotify::model::PrivateUser, Error> {
        self.client
            .current_user()
            .await
            .map_err(|e| Error::unauthenticated(e))
    }

    pub async fn get_playlists(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<SimplifiedPlaylist>, ClientError> {
        let playlists = self
            .client
            .current_user_playlists_manual(Some(limit), Some(offset))
            .await?;
        Ok(playlists.items)
    }

    pub fn is_logged_in(&self) -> bool {
        self.creds.is_some()
    }
    pub async fn auth(&mut self) -> Result<(), Error> {
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
        self.creds = Some(c);
        self.connect().await?;
        Ok(())
    }
}
