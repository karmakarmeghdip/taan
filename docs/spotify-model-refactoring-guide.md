# Spotify Model Refactoring Guide

## Current State Analysis

Your current `SpotifyState` in `src/spotify/mod.rs` is a monolithic structure that mixes:
- Service responsibilities (API communication, authentication)
- Resource management (player, session, cache)
- Business logic (track loading, playlist fetching)
- Direct UI manipulation (through player events)

## Target Architecture

### 1. Service Layer (`src/services/spotify_service.rs`)

```rust
// New service structure - handles external communication only
pub struct SpotifyService {
    session: Session,
    player: Arc<Player>,
    web_client: Arc<AuthCodeSpotify>,
    // Remove UI references - services shouldn't know about UI
}

impl SpotifyService {
    // Pure async operations - return Result types
    pub async fn authenticate(&self, credentials: Credentials) -> Result<(), SpotifyError>;
    pub async fn get_user_playlists(&self, limit: u32, offset: u32) -> Result<Vec<SimplifiedPlaylist>, SpotifyError>;
    pub async fn get_playlist_tracks(&self, id: PlaylistId, limit: u32, offset: u32) -> Result<Vec<PlaylistItem>, SpotifyError>;
    pub async fn load_track(&self, track_id: SpotifyId) -> Result<(), SpotifyError>;
    
    // Player control - return success/failure only
    pub fn play(&self) -> Result<(), SpotifyError>;
    pub fn pause(&self) -> Result<(), SpotifyError>;
    pub fn seek(&self, position_ms: u32) -> Result<(), SpotifyError>;
    
    // Event subscription - return events, don't handle them
    pub fn player_events(&self) -> mpsc::Receiver<PlayerEvent>;
}
```

### 2. Model Layer (`src/models/spotify.rs`)

```rust
// Pure data structures with business logic
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub duration_ms: u32,
    pub cover_urls: Vec<String>,
}

impl Track {
    // Business logic methods
    pub fn duration_seconds(&self) -> u32 {
        self.duration_ms / 1000
    }
    
    pub fn primary_artist(&self) -> Option<&Artist> {
        self.artists.iter().find(|a| a.role == ArtistRole::MainArtist)
    }
    
    pub fn best_cover_url(&self) -> Option<&String> {
        // Logic to select best resolution cover
        self.cover_urls.first()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub track_count: u32,
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub current_track: Option<Track>,
    pub is_playing: bool,
    pub position_ms: u32,
    pub volume: f32,
    pub shuffle: bool,
    pub repeat: RepeatMode,
}

// Custom error types for better error handling
#[derive(Debug, thiserror::Error)]
pub enum SpotifyError {
    #[error("Authentication failed: {0}")]
    Authentication(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Player error: {0}")]
    Player(String),
    #[error("Rate limited. Retry after {seconds} seconds")]
    RateLimit { seconds: u64 },
}
```

### 3. ViewModel Layer (`src/viewmodels/player_vm.rs`)

```rust
// Coordinates between Service, Model, and UI
pub struct PlayerViewModel {
    ui_weak: slint::Weak<MainWindow>,
    spotify_service: Arc<SpotifyService>,
    player_model: Arc<Mutex<PlayerModel>>, // Separate from UI state
    rt_handle: tokio::runtime::Handle,
}

impl PlayerViewModel {
    pub fn new(
        ui_weak: slint::Weak<MainWindow>,
        spotify_service: Arc<SpotifyService>,
        rt_handle: tokio::runtime::Handle,
    ) -> Self {
        let vm = Self {
            ui_weak,
            spotify_service,
            player_model: Arc::new(Mutex::new(PlayerModel::default())),
            rt_handle,
        };
        
        vm.setup_callbacks();
        vm.setup_event_handlers();
        vm
    }
    
    fn setup_callbacks(&self) {
        // Register UI callbacks
        if let Ok(ui) = self.ui_weak.upgrade() {
            let app = ui.global::<AppState>();
            
            // Play callback
            let service = self.spotify_service.clone();
            app.on_play(move || {
                if let Err(e) = service.play() {
                    eprintln!("Play failed: {}", e);
                }
            });
            
            // Pause callback
            let service = self.spotify_service.clone();
            app.on_pause(move || {
                if let Err(e) = service.pause() {
                    eprintln!("Pause failed: {}", e);
                }
            });
            
            // Seek callback
            let service = self.spotify_service.clone();
            app.on_seek(move |position_seconds| {
                let position_ms = (position_seconds * 1000) as u32;
                if let Err(e) = service.seek(position_ms) {
                    eprintln!("Seek failed: {}", e);
                }
            });
        }
    }
    
    fn setup_event_handlers(&self) {
        let ui_weak = self.ui_weak.clone();
        let player_model = self.player_model.clone();
        let mut events = self.spotify_service.player_events();
        
        self.rt_handle.spawn(async move {
            while let Some(event) = events.recv().await {
                match event {
                    PlayerEvent::TrackChanged { track } => {
                        // Update model first
                        {
                            let mut model = player_model.lock().unwrap();
                            model.current_track = Some(track.clone());
                        }
                        
                        // Then update UI
                        if let Err(e) = Self::update_ui_track(&ui_weak, &track).await {
                            eprintln!("Failed to update UI: {}", e);
                        }
                    }
                    PlayerEvent::Playing { position_ms } => {
                        {
                            let mut model = player_model.lock().unwrap();
                            model.is_playing = true;
                            model.position_ms = position_ms;
                        }
                        
                        Self::update_ui_playback_state(&ui_weak, true, position_ms);
                    }
                    // Handle other events...
                }
            }
        });
    }
    
    async fn update_ui_track(ui_weak: &slint::Weak<MainWindow>, track: &Track) -> Result<(), anyhow::Error> {
        // Load cover art asynchronously
        if let Some(cover_url) = track.best_cover_url() {
            let cover_data = load_cover_image(cover_url).await?;
            
            ui_weak.upgrade_in_event_loop({
                let track = track.clone();
                move |ui| {
                    let app = ui.global::<AppState>();
                    app.set_song_title(track.name.into());
                    app.set_album_art(cover_data);
                    if let Some(artist) = track.primary_artist() {
                        app.set_artist_name(artist.name.clone().into());
                    }
                    app.set_music_duration(track.duration_seconds() as i32);
                }
            })?;
        }
        Ok(())
    }
}
```

## Migration Steps

### Step 1: Create Service Interface

1. Create `src/services/mod.rs`:
```rust
pub mod spotify_service;
pub use spotify_service::*;
```

2. Move networking and external API logic from `SpotifyState` to `SpotifyService`
3. Remove all UI dependencies from the service layer
4. Return proper `Result` types instead of printing errors

### Step 2: Extract Data Models

1. Create `src/models/spotify.rs`
2. Define clean data structures for Track, Playlist, Artist, etc.
3. Add conversion methods from librespot/rspotify types to your models
4. Add business logic methods (duration conversion, artist filtering, etc.)

### Step 3: Create Player ViewModel

1. Create `src/viewmodels/player_vm.rs`
2. Move callback registration logic from `handlers/player.rs`
3. Implement proper error handling and logging
4. Add state synchronization between model and UI

### Step 4: Update Main Application

1. Update `lib.rs` to create and wire ViewModels
2. Replace direct `SpotifyState` usage with `PlayerViewModel`
3. Remove old `handlers/` registrations

## Benefits of This Refactoring

### 1. Clear Separation of Concerns
- **Service**: Only handles Spotify communication
- **Model**: Only contains data and business logic
- **ViewModel**: Only handles UI coordination

### 2. Better Error Handling
- Services return `Result` types
- ViewModels handle errors appropriately
- UI shows user-friendly error messages

### 3. Testability
- Services can be mocked for testing
- Models can be unit tested independently
- ViewModels can be tested with mock services

### 4. Maintainability
- Changes to Spotify API only affect service layer
- UI changes only affect ViewModels
- Business logic changes only affect models

## Key Patterns to Follow

### 1. Async Coordination Pattern
```rust
// ViewModel coordinates async operations
pub async fn load_track(&self, track_id: String) -> Result<(), SpotifyError> {
    // 1. Call service
    let track = self.spotify_service.load_track(track_id).await?;
    
    // 2. Update model
    {
        let mut model = self.player_model.lock().unwrap();
        model.current_track = Some(track.clone());
    }
    
    // 3. Update UI
    self.update_ui_with_track(track).await?;
    
    Ok(())
}
```

### 2. Event Handling Pattern
```rust
// Service emits events, ViewModel handles them
fn setup_event_handlers(&self) {
    let events = self.service.player_events();
    self.rt_handle.spawn(async move {
        while let Some(event) = events.recv().await {
            // Handle event: Update model then UI
        }
    });
}
```

### 3. Error Propagation Pattern
```rust
// ViewModels handle service errors gracefully
match self.spotify_service.play().await {
    Ok(()) => {
        // Update UI to show playing state
    }
    Err(SpotifyError::Authentication(_)) => {
        // Trigger re-authentication flow
    }
    Err(SpotifyError::Network(_)) => {
        // Show network error to user
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

This refactoring will transform your Spotify integration from a monolithic component into a clean, maintainable, and testable architecture.