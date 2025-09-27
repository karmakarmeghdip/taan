# Practical Implementation Roadmap

## Phase 1: Foundation Setup (Week 1)

### Create Directory Structure

```
src/
├── viewmodels/
│   ├── mod.rs
│   ├── auth_vm.rs
│   ├── player_vm.rs  
│   └── window_vm.rs
├── services/
│   ├── mod.rs
│   └── spotify_service.rs
└── models/
    ├── mod.rs (already exists)
    └── spotify.rs (new)
```

### Basic ViewModel Template

Create this template for all ViewModels:

```rust
// Template for all ViewModels
use slint::ComponentHandle;
use std::sync::Arc;

#[derive(Clone)]
pub struct TemplateViewModel {
    ui_weak: slint::Weak<crate::MainWindow>,
    rt_handle: tokio::runtime::Handle,
}

impl TemplateViewModel {
    pub fn new(
        ui_weak: slint::Weak<crate::MainWindow>,
        rt_handle: tokio::runtime::Handle,
    ) -> Self {
        let vm = Self {
            ui_weak,
            rt_handle,
        };
        
        vm.register_callbacks();
        vm
    }
    
    fn register_callbacks(&self) {
        // Implementation specific to each ViewModel
    }
    
    // Helper method for safe UI updates
    fn update_ui<F>(&self, update_fn: F) -> Result<(), slint::PlatformError>
    where
        F: FnOnce(&crate::MainWindow) + Send + 'static,
    {
        self.ui_weak.upgrade_in_event_loop(update_fn)
    }
}
```

### Start with WindowViewModel

This is the simplest ViewModel to implement first:

```rust
// src/viewmodels/window_vm.rs
use slint::ComponentHandle;

#[derive(Clone)]
pub struct WindowViewModel {
    ui_weak: slint::Weak<crate::MainWindow>,
}

impl WindowViewModel {
    pub fn new(ui_weak: slint::Weak<crate::MainWindow>) -> Self {
        let vm = Self { ui_weak };
        vm.register_callbacks();
        vm
    }
    
    fn register_callbacks(&self) {
        if let Ok(ui) = self.ui_weak.upgrade() {
            let app = ui.global::<crate::AppState>();
            
            // Close window callback
            let vm = self.clone();
            app.on_close_window(move || {
                vm.handle_close();
            });
            
            // Drag window callback  
            let vm = self.clone();
            app.on_start_drag(move || {
                vm.handle_drag();
            });
        }
    }
    
    fn handle_close(&self) {
        if let Ok(ui) = self.ui_weak.upgrade() {
            ui.hide().unwrap_or_else(|e| {
                eprintln!("Failed to hide window: {}", e);
            });
        }
    }
    
    fn handle_drag(&self) {
        #[cfg(not(target_os = "android"))]
        if let Ok(ui) = self.ui_weak.upgrade() {
            ui.window().with_winit_window(|winit_window| {
                winit_window.drag_window().unwrap_or_else(|e| {
                    eprintln!("Failed to drag window: {}", e);
                });
            });
        }
    }
}
```

## Phase 2: Service Extraction (Week 2)

### Extract Spotify Service

1. Create `src/services/spotify_service.rs`
2. Copy core functionality from `src/spotify/mod.rs`
3. Remove UI dependencies
4. Add proper error handling

```rust
// src/services/spotify_service.rs - Simplified initial version
use std::sync::Arc;
use librespot_core::Session;
use librespot_playback::Player;
use rspotify::AuthCodeSpotify;

#[derive(Clone)]
pub struct SpotifyService {
    session: Session,
    player: Arc<Player>,
    web_client: Arc<AuthCodeSpotify>,
}

impl SpotifyService {
    pub fn new() -> Self {
        // Move initialization logic from SpotifyState::default()
        // Remove any UI-related code
    }
    
    // Start with basic methods
    pub async fn init(&self) -> Result<(), crate::models::spotify::SpotifyError> {
        // Move from SpotifyState::init()
    }
    
    pub fn play(&self) -> Result<(), crate::models::spotify::SpotifyError> {
        self.player.play();
        Ok(())
    }
    
    pub fn pause(&self) -> Result<(), crate::models::spotify::SpotifyError> {
        self.player.pause();
        Ok(())
    }
}
```

### Update lib.rs Gradually

Replace the current setup step by step:

```rust
// src/lib.rs - Gradual transition
pub fn main() -> anyhow::Result<()> {
    // Existing platform setup...
    
    let token = tokio_util::sync::CancellationToken::new();
    let (rt, join) = setup_rt(token.clone())?;
    
    // OLD: let spot = rt.block_on(async { spotify::SpotifyState::default() });
    // NEW: Create service
    let spotify_service = Arc::new(services::SpotifyService::new());
    
    let ui = MainWindow::new()?;
    
    // OLD: handlers::setup(ui.as_weak(), spot.clone(), rt.clone());
    // NEW: Create ViewModels one by one
    let _window_vm = viewmodels::WindowViewModel::new(ui.as_weak());
    
    // Keep existing handlers for now, replace gradually
    // handlers::setup(ui.as_weak(), spot.clone(), rt.clone());
    
    ui.run()?;
    token.cancel();
    join.join().unwrap();
    Ok(())
}
```

## Phase 3: Player ViewModel (Week 3)

### Implement PlayerViewModel

This is the most complex ViewModel:

```rust
// src/viewmodels/player_vm.rs
use std::sync::Arc;
use slint::ComponentHandle;

#[derive(Clone)]
pub struct PlayerViewModel {
    ui_weak: slint::Weak<crate::MainWindow>,
    spotify_service: Arc<crate::services::SpotifyService>,
    rt_handle: tokio::runtime::Handle,
}

impl PlayerViewModel {
    pub fn new(
        ui_weak: slint::Weak<crate::MainWindow>,
        spotify_service: Arc<crate::services::SpotifyService>,
        rt_handle: tokio::runtime::Handle,
    ) -> Self {
        let vm = Self {
            ui_weak,
            spotify_service,
            rt_handle,
        };
        
        vm.register_callbacks();
        vm.setup_event_listeners();
        vm
    }
    
    fn register_callbacks(&self) {
        if let Ok(ui) = self.ui_weak.upgrade() {
            let app = ui.global::<crate::AppState>();
            
            // Play callback
            let vm = self.clone();
            app.on_play(move || {
                vm.handle_play();
            });
            
            // Add other callbacks...
        }
    }
    
    fn handle_play(&self) {
        // Update UI immediately for responsiveness
        let _ = self.ui_weak.upgrade_in_event_loop(|ui| {
            let app = ui.global::<crate::AppState>();
            app.set_is_playing(true);
        });
        
        // Perform async operation
        let vm = self.clone();
        self.rt_handle.spawn(async move {
            if let Err(e) = vm.spotify_service.play() {
                // Revert UI state on error
                let _ = vm.ui_weak.upgrade_in_event_loop(|ui| {
                    let app = ui.global::<crate::AppState>();
                    app.set_is_playing(false);
                });
                eprintln!("Play failed: {}", e);
            }
        });
    }
    
    fn setup_event_listeners(&self) {
        // Move player event handling from handlers/player.rs
        // This will be the most complex part
    }
}
```

## Phase 4: Auth ViewModel (Week 4)

### Implement AuthViewModel

```rust
// src/viewmodels/auth_vm.rs
#[derive(Clone)]
pub struct AuthViewModel {
    ui_weak: slint::Weak<crate::MainWindow>,
    spotify_service: Arc<crate::services::SpotifyService>,
    rt_handle: tokio::runtime::Handle,
}

impl AuthViewModel {
    // Similar pattern to PlayerViewModel
    // Move logic from handlers/auth.rs
    
    fn handle_login(&self) {
        // Set loading state
        let _ = self.ui_weak.upgrade_in_event_loop(|ui| {
            let app = ui.global::<crate::AppState>();
            app.set_login_in_progress(true);
        });
        
        // Perform authentication
        let vm = self.clone();
        self.rt_handle.spawn(async move {
            match vm.spotify_service.auth().await {
                Ok(()) => {
                    let _ = vm.ui_weak.upgrade_in_event_loop(|ui| {
                        let app = ui.global::<crate::AppState>();
                        app.set_loggedIn(true);
                        app.set_login_in_progress(false);
                    });
                }
                Err(e) => {
                    let _ = vm.ui_weak.upgrade_in_event_loop(|ui| {
                        let app = ui.global::<crate::AppState>();
                        app.set_login_in_progress(false);
                    });
                    eprintln!("Login failed: {}", e);
                }
            }
        });
    }
}
```

## Phase 5: Integration and Cleanup (Week 5)

### Create ViewModelManager

```rust
// src/viewmodels/mod.rs
mod auth_vm;
mod player_vm;
mod window_vm;

pub use auth_vm::AuthViewModel;
pub use player_vm::PlayerViewModel;
pub use window_vm::WindowViewModel;

pub struct ViewModelManager {
    pub auth_vm: AuthViewModel,
    pub player_vm: PlayerViewModel,
    pub window_vm: WindowViewModel,
}

impl ViewModelManager {
    pub fn new(
        ui_weak: slint::Weak<crate::MainWindow>,
        spotify_service: std::sync::Arc<crate::services::SpotifyService>,
        rt_handle: tokio::runtime::Handle,
    ) -> Self {
        Self {
            auth_vm: AuthViewModel::new(
                ui_weak.clone(),
                spotify_service.clone(),
                rt_handle.clone(),
            ),
            player_vm: PlayerViewModel::new(
                ui_weak.clone(),
                spotify_service.clone(),
                rt_handle.clone(),
            ),
            window_vm: WindowViewModel::new(ui_weak),
        }
    }
}
```

### Final lib.rs

```rust
// src/lib.rs - Final version
pub fn main() -> anyhow::Result<()> {
    // Platform setup...
    
    let token = tokio_util::sync::CancellationToken::new();
    let (rt, join) = setup_rt(token.clone())?;
    
    let spotify_service = std::sync::Arc::new(services::SpotifyService::new());
    let ui = MainWindow::new()?;
    
    let _vm_manager = viewmodels::ViewModelManager::new(
        ui.as_weak(),
        spotify_service.clone(),
        rt.clone(),
    );
    
    // Initialize Spotify
    rt.spawn({
        let spotify_service = spotify_service.clone();
        async move {
            if let Err(e) = spotify_service.init().await {
                eprintln!("Failed to init spotify: {}", e);
            }
        }
    });
    
    ui.run()?;
    token.cancel();
    join.join().unwrap();
    Ok(())
}
```

## Testing Strategy

### Test Each Phase

1. **Phase 1**: Ensure window controls still work
2. **Phase 2**: Verify Spotify service can be created without errors
3. **Phase 3**: Test play/pause functionality
4. **Phase 4**: Test authentication flow
5. **Phase 5**: Full integration testing

### Incremental Replacement

- Keep old `handlers/` code alongside new ViewModels initially
- Replace one callback at a time
- Test after each replacement
- Remove old code only after new code is proven working

### Rollback Plan

- Use git branches for each phase
- Keep backups of working states
- Be prepared to rollback if issues arise

## Common Pitfalls to Avoid

1. **Don't migrate everything at once** - Do it incrementally
2. **Test frequently** - After each small change
3. **Keep UI responsive** - Use optimistic updates
4. **Handle errors gracefully** - Don't let the app crash
5. **Maintain existing functionality** - Users shouldn't notice the refactoring

This roadmap provides a practical, step-by-step approach to refactoring your codebase into proper MVVM architecture while maintaining functionality throughout the process.