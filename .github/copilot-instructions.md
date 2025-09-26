# Taan - AI Coding Agent Instructions

## Architecture Overview

**Taan** is a native Spotify client built with **Rust + Slint**. The architecture separates concerns between:

- **Backend**: Rust with dual Spotify integration (`librespot` for playback, `rspotify` for Web API)
- **Frontend**: Slint UI with component-based architecture in `ui/components/`
- **Async Bridge**: Tokio runtime with message-passing between UI and Spotify services

## Key Technologies & Patterns

### Rust + Slint Integration

- Main entry: `src/main.rs` spawns dedicated Tokio runtime thread via `setup_rt()`
- UI compilation: `build.rs` uses `slint_build::compile("ui/main.slint")`
- Generated code: `slint::include_modules!()` imports compiled UI components
- **Critical**: Slint UI runs on main thread, async Spotify operations on dedicated runtime

### Spotify Integration Architecture

Located in `src/spotify/`, implements dual-client pattern:

```rust
// librespot for audio playback (lower-level, direct streaming)
use librespot_core::{Session, Credentials};
use librespot_playback::Player;

// rspotify for Web API operations (playlists, metadata)
use rspotify::AuthCodeSpotify;
```

**Authentication Flow**: OAuth → librespot session → rspotify token refresh
**Key Pattern**: `SpotifyState::web_auth()` bridges librespot tokens to rspotify client

### Component System (ui/components/)

**Centralized Design System**: All styling goes through `components/common/colors.slint`:

```slint
import { Colors, Spacing, BorderRadius } from "components/common/colors.slint";
```

**Component Hierarchy**:

- `common/` - Reusable components (buttons, sliders, colors)
- `player/` - Player-specific components (controls, album art, progress)
- Always import through `components/common/mod.slint` for consistency

## Development Workflow

### Primary Build Tool: Bacon

```bash
bacon run       # Default development mode with hot reload
bacon clippy-all # Lint all targets (bound to 'c' key)  
bacon test      # Run tests with output
```

**Key bacon.toml patterns**:

- `default_job = "run"` - Primary development workflow
- `on_change_strategy = "kill_then_restart"` for long-running processes
- Custom keybinding: `c = "job:clippy-all"`

### Standard Cargo Commands

```bash
cargo run                    # Build and run application
cargo check                  # Fast compile check
cargo clippy -- -W clippy::all  # Enhanced linting
```

## Critical Implementation Patterns

### Async/UI Bridge Pattern

```rust
// main.rs - Setup runtime but don't block main thread
fn setup_rt() -> tokio::io::Result<tokio::runtime::Handle> {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let rt_handle = rt.handle().clone();
    std::thread::spawn(move || {
        rt.block_on(std::future::pending::<()>());
    });
    Ok(rt_handle)
}
```

### Spotify Error Handling Pattern

```rust
// spotify/mod.rs - Retry with token refresh on 401, rate limit on 429
async fn requires_refresh(&self, e: ClientError) -> bool {
    if let ClientError::Http(HttpError::StatusCode(res)) = e {
        match res.status() {
            401 => {
                self.web_auth().await;
                true
            }
            429 => { /* handle rate limit */ true }
            _ => false
        }
    }
}
```

### Slint Component Patterns

```slint
// Always use centralized colors instead of hardcoded values
Rectangle {
    background: Colors.background-primary;  // ✓ Correct
    background: #4a3e4c;                   // ✗ Avoid
}

// Component handlers forwarding pattern
PlayerControls {
    volume <=> root.volume;  // Two-way binding
    play-pause-clicked => {  // Event forwarding
        root.is-playing = !root.is-playing;
    }
}
```

## File Structure Conventions

### UI Organization

```
ui/
├── main.slint           # App entry point, exports MainWindow
├── player.slint         # Main player component  
└── components/
    ├── common/          # Reusable design system
    │   ├── colors.slint # Central theming (ALWAYS use this)
    │   ├── button.slint # Reusable buttons with size/shape variants
    │   └── mod.slint    # Consolidated exports
    └── player/          # Player-specific components
```

### Spotify Module Structure

```
src/spotify/
├── mod.rs          # Core SpotifyState, dual-client integration
└── async_loop.rs   # Event-driven command processing (Xilem pattern)
```

## Essential Dependencies

- **slint**: UI framework with compile-time generation
- **librespot-***: Direct Spotify streaming (Premium required)
- **rspotify**: Web API for metadata/playlists
- **tokio**: Async runtime for Spotify operations
- **chrono**: Time handling for tokens/duration

## Documentation Access

### Slint Framework Reference

Use Context7 to access up-to-date Slint documentation:

- Library ID: `/slint-ui/slint`
- Topics: UI components, property bindings, animations, layouts
- Essential for understanding Slint-specific patterns and best practices

Example Context7 usage for Slint concepts:

- Component properties and callbacks
- Layout managers (VerticalLayout, HorizontalLayout, GridLayout)
- Animation and state management
- Custom component creation patterns

## Common Pitfalls

1. **Don't hardcode colors** - Always use `Colors.*` from `components/common/colors.slint`
2. **Slint compilation** - Changes to `.slint` files require rebuild via `bacon run` or `cargo run`
3. **Async context** - Spotify operations must run on Tokio runtime, not main thread
4. **Token refresh** - rspotify client needs librespot session token via `web_auth()`
5. **Component imports** - Use `components/common/mod.slint` for consolidated imports

## Key Files to Understand

- `src/main.rs` - Runtime setup and UI initialization
- `src/spotify/mod.rs` - Dual Spotify client integration
- `ui/components/common/colors.slint` - Design system foundation
- `ui/player.slint` - Main UI component with state management
- `bacon.toml` - Development workflow configuration
- `build.rs` - Slint UI compilation setup

## Authentication Notes

- **Client ID**: Hardcoded in `SPOTIFY_CLIENT_ID` constant
- **OAuth Flow**: Browser-based via `librespot_oauth::OAuthClientBuilder`
- **Scopes**: Comprehensive set in `OAUTH_SCOPES` for full API access
- **Cache**: Tokens cached in `.cache/` directory (gitignored)