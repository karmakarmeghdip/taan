# Taan Global State Management System

## Overview

This document describes the comprehensive global state management system implemented for Taan, the native Spotify client. The system is built using Slint's global singletons pattern, providing a clean separation of concerns and a well-organized interface between the UI and Rust backend.

## Architecture

The global state system is organized into four main singletons, each managing a specific aspect of the application:

### 1. AppState (`ui/state/app_state.slint`)
**Purpose**: Core application lifecycle, authentication, and navigation management

**Key Responsibilities**:
- User authentication state (login status, user profile)
- App lifecycle management (initialization, shutdown)
- Navigation and view state management
- Error handling and reporting
- Application settings

**Key Properties**:
```slint
// Authentication
is-logged-in: bool
user-id: string
username: string
has-premium: bool

// App Lifecycle
app-initialized: bool
login-in-progress: bool
current-view: string  // "login", "player", "settings"

// Error Handling
last-error: string
error-count: int
```

**Key Callbacks**:
```slint
request-login() -> bool
request-logout() -> bool
navigate-to(string) -> bool
initialize-app() -> bool
report-error(string) -> bool
```

### 2. PlayerState (`ui/state/player_state.slint`)
**Purpose**: Music player state and playback control

**Key Responsibilities**:
- Current track information and metadata
- Playback state (playing, paused, stopped)
- Audio settings (volume, mute)
- Queue and context management
- Player device information

**Key Properties**:
```slint
// Track Info
track-id: string
track-title: string
track-artist: string
track-album: string
album-art-url: string

// Playback State
is-playing: bool
track-position-ms: int
track-duration-ms: int
volume: float

// Derived Properties
progress: float  // 0.0 to 1.0
current-time: string
remaining-time: string
```

**Key Callbacks**:
```slint
play() -> bool
pause() -> bool
toggle-play-pause() -> bool
next-track() -> bool
previous-track() -> bool
seek-to-position(float) -> bool
set-volume(float) -> bool
```

### 3. SpotifyAPI (`ui/state/spotify_api.slint`)
**Purpose**: Interface for all Spotify Web API operations

**Key Responsibilities**:
- OAuth authentication flow
- Playback control API calls
- Track, album, and artist operations
- Playlist management
- Library operations (saved tracks, albums, artists)
- Search functionality
- Recommendations and discovery

**Key Callbacks**:
```slint
// Authentication
start-oauth-login() -> bool
refresh-access-token() -> bool

// Playback Control
start-playback(string, int) -> bool
pause-playback() -> bool
skip-to-next() -> bool
skip-to-previous() -> bool

// Content Operations
get-track(string) -> bool
get-playlist(string) -> bool
search(string, string, int, int) -> bool

// Library Management
save-tracks(string) -> bool
follow-artists(string) -> bool
```

### 4. UIState (`ui/state/ui_state.slint`)
**Purpose**: UI-specific state and preferences management

**Key Responsibilities**:
- Window state and geometry
- Theme and appearance settings
- UI layout preferences
- Modal and popup management
- Notifications and loading states
- Keyboard and accessibility settings

**Key Properties**:
```slint
// Window State
window-width: int
window-height: int
is-maximized: bool
is-fullscreen: bool

// Theme and Appearance
current-theme: string  // "dark", "light", "auto"
ui-scale: float
font-family: string

// UI State
sidebar-visible: bool
active-modal: string
notification-message: string
is-loading: bool
```

**Key Callbacks**:
```slint
toggle-maximized() -> bool
set-theme(string) -> bool
show-notification(string, string) -> bool
show-loading(string) -> bool
```

## Integration with Components

### Main Window (`ui/main.slint`)
The main window imports and re-exports all globals for native code access:

```slint
import { AppState, PlayerState, SpotifyAPI, UIState } from "state/mod.slint";

// Re-export globals for native access
export { AppState, PlayerState, SpotifyAPI, UIState }

export component MainWindow inherits Window {
    // Key properties are bound to global state
    min-width: UIState.window-width * 1px;
    min-height: UIState.window-height * 1px;
    
    // Conditional view rendering based on app state
    if !AppState.is-logged-in: LoginWindow { /* ... */ }
    if AppState.is-logged-in && AppState.current-view == "player": MusicPlayer { /* ... */ }
}
```

### Player Component (`ui/player.slint`)
The music player component is now stateless and binds directly to global state:

```slint
import { PlayerState } from "state/mod.slint";

export component MusicPlayer {
    PlayerControls {
        // Direct binding to global state
        song-title: PlayerState.track-title;
        artist-name: PlayerState.track-artist;
        is-playing <=> PlayerState.is-playing;
        volume <=> PlayerState.volume;
        
        // Callbacks forward to global state
        play-pause-clicked => {
            PlayerState.toggle-play-pause();
        }
        seek(value) => {
            PlayerState.seek-to-position(value);
        }
    }
}
```

### Login Component (`ui/login.slint`)
The login component uses global authentication state:

```slint
import { AppState, SpotifyAPI } from "state/mod.slint";

export component LoginWindow {
    PrimaryButton {
        enabled: !AppState.login-in-progress;
        clicked => {
            SpotifyAPI.start-oauth-login();
        }
        Text {
            text: AppState.login-in-progress ? "Logging in..." : "Login with Spotify";
        }
    }
    
    if AppState.has-error: Text {
        text: AppState.last-error;
        color: Colors.error;
    }
}
```

## Rust Integration

### Accessing Globals from Rust

With the new system, the Rust code can access all globals through the main window:

```rust
// In your main Rust code
let app = App::new();

// Access AppState global
let app_state = app.global::<AppState>();
app_state.set_is_logged_in(true);
app_state.set_username("user123".into());

// Access PlayerState global
let player_state = app.global::<PlayerState>();
player_state.set_track_title("Song Title".into());
player_state.set_is_playing(true);

// Set up callbacks
app_state.on_request_login(|| {
    // Handle login request
    true
});

player_state.on_play(|| {
    // Handle play request
    true
});
```

### Callback Implementation Pattern

The Rust backend should implement all the callbacks defined in the globals:

```rust
// Example: Setting up Spotify API callbacks
let spotify_api = app.global::<SpotifyAPI>();

spotify_api.on_start_oauth_login(|| {
    // Trigger OAuth flow
    tokio::spawn(async {
        // OAuth implementation
    });
    true
});

spotify_api.on_get_current_playback(|| {
    // Get current playback state from Spotify
    tokio::spawn(async {
        let playback = spotify_client.current_playback().await?;
        // Update PlayerState with received data
    });
    true
});
```

## Benefits of This Architecture

### 1. **Separation of Concerns**
- UI components focus solely on presentation
- Business logic is handled by Rust callbacks
- State management is centralized and predictable

### 2. **Type Safety**
- All state properties are strongly typed
- Slint compiler catches type mismatches at compile time
- No runtime type errors

### 3. **Reactive Updates**
- UI automatically updates when global state changes
- No manual UI refresh needed
- Consistent state across all components

### 4. **Maintainability**
- Clear ownership of state
- Easy to locate and modify state definitions
- Logical grouping of related functionality

### 5. **Testability**
- Global state can be easily mocked for testing
- Clear interfaces between UI and business logic
- Predictable state transitions

## Migration Guide

### For New Features
1. Identify which global the feature belongs to
2. Add necessary properties and callbacks to the appropriate global
3. Update UI components to use the global state
4. Implement the callbacks in Rust

### For Existing Components
1. Remove local state properties
2. Import the relevant global(s)
3. Bind component properties to global state
4. Replace local callbacks with global callback invocations

## Future Enhancements

### Planned Additions
1. **Playlist Management State** - Dedicated global for playlist operations
2. **Search State** - Search history, filters, and results management
3. **Settings State** - User preferences and configuration
4. **Cache State** - Offline data and caching management

### Performance Optimizations
1. **Lazy Loading** - Load global state only when needed
2. **State Persistence** - Save/restore state across app restarts
3. **Debounced Updates** - Rate limit high-frequency state updates

This global state system provides a solid foundation for the Taan application, ensuring clean architecture, type safety, and maintainability as the project grows.