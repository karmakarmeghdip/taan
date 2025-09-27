# UI Callback Organization Guide

## Current Callback Pattern Analysis

Your current callback setup in `src/handlers/` has these characteristics:
- Direct UI manipulation from handlers
- Callbacks scattered across multiple handler files
- Service dependencies passed directly to callback closures
- No centralized error handling or logging

## Target MVVM Callback Pattern

### 1. Centralized Callback Registration

Create a single point for registering all UI callbacks within each ViewModel:

```rust
// src/viewmodels/player_vm.rs
impl PlayerViewModel {
    pub fn new(/* params */) -> Self {
        let vm = Self { /* fields */ };
        vm.register_callbacks();
        vm
    }
    
    fn register_callbacks(&self) {
        self.register_playback_callbacks();
        self.register_navigation_callbacks();
        self.register_volume_callbacks();
        // etc.
    }
    
    fn register_playback_callbacks(&self) {
        if let Ok(ui) = self.ui_weak.upgrade() {
            let app = ui.global::<AppState>();
            
            // Play callback
            let vm = self.clone(); // ViewModels should be cheaply cloneable
            app.on_play(move || {
                vm.handle_play();
            });
            
            // Pause callback
            let vm = self.clone();
            app.on_pause(move || {
                vm.handle_pause();
            });
            
            // Seek callback
            let vm = self.clone();
            app.on_seek(move |position| {
                vm.handle_seek(position);
            });
        }
    }
}
```

### 2. Command Pattern for Actions

Implement commands for all user actions:

```rust
impl PlayerViewModel {
    // Synchronous command handlers (for immediate UI feedback)
    fn handle_play(&self) {
        // 1. Immediate UI feedback
        self.update_ui_playing_state(true);
        
        // 2. Async operation
        let vm = self.clone();
        self.rt_handle.spawn(async move {
            match vm.spotify_service.play().await {
                Ok(()) => {
                    // Success - UI already updated
                    log::info!("Playback started successfully");
                }
                Err(e) => {
                    // Revert UI state and show error
                    vm.update_ui_playing_state(false);
                    vm.show_error("Failed to start playback", e);
                }
            }
        });
    }
    
    fn handle_pause(&self) {
        self.update_ui_playing_state(false);
        
        let vm = self.clone();
        self.rt_handle.spawn(async move {
            if let Err(e) = vm.spotify_service.pause().await {
                vm.update_ui_playing_state(true);
                vm.show_error("Failed to pause playback", e);
            }
        });
    }
    
    fn handle_seek(&self, position_seconds: i32) {
        let position_ms = (position_seconds * 1000) as u32;
        
        // Optimistic UI update
        self.update_ui_position(position_seconds);
        
        let vm = self.clone();
        self.rt_handle.spawn(async move {
            if let Err(e) = vm.spotify_service.seek(position_ms).await {
                // Could revert position or let player events correct it
                vm.show_error("Failed to seek", e);
            }
        });
    }
}
```

### 3. ViewModel Lifecycle Management

```rust
// src/viewmodels/mod.rs
pub struct ViewModelManager {
    player_vm: Arc<PlayerViewModel>,
    auth_vm: Arc<AuthViewModel>,
    window_vm: Arc<WindowViewModel>,
}

impl ViewModelManager {
    pub fn new(
        ui_weak: slint::Weak<MainWindow>,
        spotify_service: Arc<SpotifyService>,
        rt_handle: tokio::runtime::Handle,
    ) -> Self {
        Self {
            player_vm: Arc::new(PlayerViewModel::new(
                ui_weak.clone(),
                spotify_service.clone(),
                rt_handle.clone(),
            )),
            auth_vm: Arc::new(AuthViewModel::new(
                ui_weak.clone(),
                spotify_service.clone(),
                rt_handle.clone(),
            )),
            window_vm: Arc::new(WindowViewModel::new(ui_weak.clone())),
        }
    }
    
    pub fn initialize(&self) {
        // All ViewModels register their callbacks here
        // This ensures proper initialization order
    }
    
    pub fn cleanup(&self) {
        // Cleanup resources when app shuts down
    }
}
```

### 4. Error Handling Strategy

```rust
impl PlayerViewModel {
    fn show_error(&self, context: &str, error: SpotifyError) {
        match error {
            SpotifyError::Authentication(_) => {
                log::warn!("{}: Authentication required", context);
                self.trigger_reauth();
            }
            SpotifyError::Network(_) => {
                log::warn!("{}: Network error", context);
                self.show_user_message("Network error - please check your connection");
            }
            SpotifyError::RateLimit { seconds } => {
                log::warn!("{}: Rate limited for {} seconds", context, seconds);
                self.show_user_message(&format!("Too many requests - please wait {} seconds", seconds));
            }
            SpotifyError::Player(msg) => {
                log::error!("{}: Player error: {}", context, msg);
                self.show_user_message("Playback error occurred");
            }
        }
    }
    
    fn show_user_message(&self, message: &str) {
        if let Err(e) = self.ui_weak.upgrade_in_event_loop({
            let message = message.to_string();
            move |ui| {
                let app = ui.global::<AppState>();
                // Assume you add an error message field to AppState
                app.set_error_message(message.into());
                app.set_show_error(true);
            }
        }) {
            log::error!("Failed to show error message: {}", e);
        }
    }
}
```

## Callback Organization Best Practices

### 1. Grouped Registration

Organize callbacks by functionality:

```rust
impl AuthViewModel {
    fn register_callbacks(&self) {
        self.register_login_callbacks();
        self.register_logout_callbacks();
    }
    
    fn register_login_callbacks(&self) {
        if let Ok(ui) = self.ui_weak.upgrade() {
            let app = ui.global::<AppState>();
            
            let vm = self.clone();
            app.on_login_clicked(move || {
                vm.handle_login();
            });
        }
    }
}

impl WindowViewModel {
    fn register_callbacks(&self) {
        self.register_window_controls();
        self.register_drag_behavior();
    }
    
    fn register_window_controls(&self) {
        if let Ok(ui) = self.ui_weak.upgrade() {
            let app = ui.global::<AppState>();
            
            let vm = self.clone();
            app.on_close_window(move || {
                vm.handle_close();
            });
        }
    }
}
```

### 2. Async Operation Pattern

For all async operations, follow this pattern:

```rust
fn handle_async_action(&self, /* params */) {
    // 1. Validate input (if needed)
    
    // 2. Update UI for loading state
    self.set_loading_state(true);
    
    // 3. Spawn async task
    let vm = self.clone();
    self.rt_handle.spawn(async move {
        match vm.service.async_operation().await {
            Ok(result) => {
                // Update model
                vm.update_model(result);
                
                // Update UI
                vm.update_ui_success(result);
            }
            Err(e) => {
                vm.handle_error("async_operation", e);
            }
        }
        
        // Always clear loading state
        vm.set_loading_state(false);
    });
}
```

### 3. UI Update Helpers

Create reusable UI update methods:

```rust
impl PlayerViewModel {
    fn update_ui_playing_state(&self, is_playing: bool) {
        if let Err(e) = self.ui_weak.upgrade_in_event_loop(move |ui| {
            let app = ui.global::<AppState>();
            app.set_is_playing(is_playing);
        }) {
            log::error!("Failed to update playing state: {}", e);
        }
    }
    
    fn update_ui_position(&self, position_seconds: i32) {
        if let Err(e) = self.ui_weak.upgrade_in_event_loop(move |ui| {
            let app = ui.global::<AppState>();
            app.set_current_time(position_seconds);
        }) {
            log::error!("Failed to update position: {}", e);
        }
    }
    
    fn update_ui_track_info(&self, track: &Track) {
        let track = track.clone();
        if let Err(e) = self.ui_weak.upgrade_in_event_loop(move |ui| {
            let app = ui.global::<AppState>();
            app.set_song_title(track.name.into());
            app.set_music_duration(track.duration_seconds() as i32);
            if let Some(artist) = track.primary_artist() {
                app.set_artist_name(artist.name.clone().into());
            }
        }) {
            log::error!("Failed to update track info: {}", e);
        }
    }
}
```

## Migration Strategy

### Step 1: Create ViewModel Structure

1. Create `src/viewmodels/mod.rs`:
```rust
mod auth_vm;
mod player_vm;
mod window_vm;

pub use auth_vm::*;
pub use player_vm::*;
pub use window_vm::*;

pub struct ViewModelManager {
    // All ViewModels
}
```

### Step 2: Move Callback Registration

1. Create basic ViewModel structs
2. Move callback registration from `handlers/` to ViewModels
3. Start with one ViewModel (e.g., `WindowViewModel` for simple window operations)
4. Test and verify functionality

### Step 3: Implement Command Pattern

1. Replace direct service calls with command methods
2. Add proper error handling
3. Add loading states where appropriate

### Step 4: Add Advanced Features

1. Implement proper logging
2. Add user feedback for errors
3. Add optimistic UI updates
4. Implement retry mechanisms for failed operations

### Step 5: Clean Up

1. Remove old `handlers/` directory
2. Update `lib.rs` to use `ViewModelManager`
3. Test all functionality

## Updated lib.rs Structure

```rust
// src/lib.rs
mod models;
mod viewmodels;
mod services;

use viewmodels::ViewModelManager;
use services::SpotifyService;

pub fn main() -> anyhow::Result<()> {
    // Platform setup...
    
    let token = tokio_util::sync::CancellationToken::new();
    let (rt, join) = setup_rt(token.clone())?;
    
    let spotify_service = Arc::new(SpotifyService::new());
    let ui = MainWindow::new()?;
    
    let vm_manager = ViewModelManager::new(
        ui.as_weak(),
        spotify_service,
        rt.clone(),
    );
    
    // Initialize all ViewModels and register callbacks
    vm_manager.initialize();
    
    // Initialize Spotify service
    rt.spawn({
        let spotify_service = vm_manager.spotify_service().clone();
        async move {
            if let Err(e) = spotify_service.init().await {
                log::error!("Failed to initialize Spotify: {}", e);
            }
        }
    });
    
    ui.run()?;
    
    // Cleanup
    vm_manager.cleanup();
    token.cancel();
    join.join().unwrap();
    
    Ok(())
}
```

This organization provides:
- Clear separation of concerns
- Centralized error handling
- Proper async operation management
- Easy testing and maintenance
- Consistent patterns across all UI interactions