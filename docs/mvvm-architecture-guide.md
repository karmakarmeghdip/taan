# MVVM Architecture Refactoring Guide for Taan

## Current Architecture Analysis

Your codebase has a good foundation but needs better separation of concerns. Currently:

- **Models**: Partially implemented as thin wrappers around Slint global state
- **Views**: Well-structured Slint UI components with centralized state
- **ViewModels**: Missing - handlers directly manipulate UI state
- **Services**: Spotify functionality is isolated but not properly modeled

## Target MVVM Structure

```
src/
├── main.rs                    # Application entry point
├── lib.rs                     # Library setup and runtime management
├── models/                    # Data models and business logic
│   ├── mod.rs
│   ├── authentication.rs      # Auth state and operations
│   ├── player.rs             # Player state and metadata
│   ├── playlists.rs          # Playlist data structures
│   ├── tracks.rs             # Track data structures
│   └── spotify.rs            # NEW: Spotify service model
├── viewmodels/               # NEW: ViewModels for UI state management
│   ├── mod.rs
│   ├── authentication_vm.rs   # Auth UI logic and callbacks
│   ├── player_vm.rs          # Player UI logic and callbacks
│   ├── window_vm.rs          # Window management logic
│   └── main_vm.rs            # Main application view model
├── services/                 # External service integrations
│   ├── mod.rs
│   └── spotify_service.rs    # Refactored Spotify integration
└── ui/                       # Slint UI components (unchanged structure)
```

## MVVM Responsibilities

### Models (`src/models/`)
- **Purpose**: Pure data structures and business logic
- **Responsibilities**:
  - Data validation and transformation
  - Business rules and constraints
  - Data serialization/deserialization
  - State persistence logic
- **What they DON'T do**:
  - Direct UI manipulation
  - Async operations (delegate to services)
  - Event handling

### ViewModels (`src/viewmodels/`)
- **Purpose**: Bridge between Models and Views
- **Responsibilities**:
  - UI state management
  - Event handling and callback registration
  - Async operation coordination
  - UI-specific data formatting
  - Command pattern implementation
- **Key Pattern**: Each ViewModel owns UI callbacks and coordinates with services

### Services (`src/services/`)
- **Purpose**: External integrations and async operations
- **Responsibilities**:
  - Spotify API communication
  - Authentication flows
  - Network operations
  - Resource management (audio players, cache)
- **Pattern**: Services return data to ViewModels, never directly update UI

### Views (`ui/`)
- **Purpose**: UI presentation and user interaction
- **Responsibilities**:
  - Visual layout and styling
  - User input capture
  - Data binding to ViewModels
  - Component composition

## Key Design Principles

### 1. Single Responsibility
Each component should have ONE clear purpose:
- Models: Data and business logic
- ViewModels: UI coordination and state
- Services: External operations
- Views: Presentation only

### 2. Dependency Direction
Dependencies should flow in ONE direction:
```
View -> ViewModel -> Model
            ↓
        Service
```

### 3. Event Flow
```
User Action → View → ViewModel → Service → Model
                ↓       ↑         ↓
            UI Update ← ← ← ← Data Change
```

### 4. State Management
- **Single Source of Truth**: Slint's AppState remains the central UI state
- **Model State**: Business logic state separate from UI state
- **Synchronization**: ViewModels keep both in sync

## Migration Strategy

### Phase 1: Create ViewModels Layer
1. Create `viewmodels/` directory structure
2. Move callback registration logic from `handlers/` to ViewModels
3. Create ViewModel structs that own UI weak references
4. Implement proper error handling and logging

### Phase 2: Refactor Spotify Integration
1. Move `SpotifyState` from `spotify/mod.rs` to `services/spotify_service.rs`
2. Create `models/spotify.rs` for data structures
3. Create `viewmodels/player_vm.rs` to coordinate Spotify service with UI
4. Remove direct Spotify manipulation from handlers

### Phase 3: Enhance Models
1. Add validation to existing model structs
2. Implement proper error types
3. Add serialization for state persistence
4. Extract business logic from ViewModels to Models

### Phase 4: Clean Up and Optimize
1. Remove old `handlers/` directory
2. Simplify `main.rs` and `lib.rs`
3. Add proper dependency injection
4. Implement command pattern for user actions

## Benefits After Refactoring

- **Testability**: Each layer can be unit tested independently
- **Maintainability**: Clear separation makes changes easier
- **Scalability**: Adding new features follows established patterns
- **Debugging**: Issues are easier to locate and fix
- **Code Reuse**: ViewModels and Models can be reused across different UI contexts

## Next Steps

1. Read the specific refactoring guides:
   - `spotify-model-refactoring-guide.md`
   - `ui-callback-organization-guide.md`
   - `viewmodel-implementation-guide.md`
2. Start with Phase 1: Create basic ViewModel structure
3. Gradually migrate one component at a time
4. Test each phase before proceeding to the next

This approach ensures your codebase becomes more maintainable while preserving all existing functionality.