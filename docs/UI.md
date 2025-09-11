# Xilem Spotify UI Layout

This document outlines the UI structure for the Xilem Spotify client using a component-based approach with a flexbox layout.

## Component Structure

The UI is broken down into several components:

-   `App`
-   `LoginPage`
-   `MainPage`
    -   `Header`
    -   `Sidebar`
    -   `MainContent`
    -   `Footer` (Player)

---

## Layout

The main layout is composed of nested flexbox containers.

```
+--------------------------------------------------+
| Header                                           |
+----------------------+---------------------------+
|                      |                           |
| Sidebar              | Main Content              |
| (User, Playlists)    | (Song List)               |
|                      |                           |
|                      |                           |
|                      |                           |
|                      |                           |
+----------------------+---------------------------+
| Footer (Player)                                  |
+--------------------------------------------------+
```

## State Management

The application uses a centralized state management approach. A single `AppState` struct holds the entire state of the application. This makes the state predictable and easier to manage.

### AppState Structure

Here is a general structure for the `AppState`.

```rust
struct AppState {
    // Authentication
    credentials: Option<SpotifyCredentials>,

    // Spotify API and Player
    spotify_api: Option<Spotify>, // Assuming a spotify library object
    spotify_player: Option<SpotifyPlayer>,

    // UI State
    current_view: View, // Enum for what's in MainContent
    // e.g. View::LikedSongs, View::Playlist(id)

    // Data
    user: Option<UserProfile>,
    playlists: Vec<Playlist>,
    active_playlist: Vec<Song>,
    
    // Player state
    current_song: Option<Song>,
    is_playing: bool,
    playback_progress: Duration,
}
```

Components will no longer hold their own state. Instead, they will be functions that receive the parts of the `AppState` they need to render, or they will access the state from a shared context. Event handlers will dispatch actions to modify the `AppState`.

## Component Pseudocode

The pseudocode below reflects the flexbox layout and centralized state.

### App

This is the root component that decides whether to show the `LoginPage` or the `MainPage` based on the authentication status in `AppState`.

```
component App(app_state: &AppState) {
    render() {
        if app_state.credentials.is_some() {
            MainPage()
        } else {
            LoginPage()
        }
    }
}
```

### LoginPage

A simple page with a "Login with Spotify" button.

```
component LoginPage {
    render() {
        Flex(direction: Column, align: Center, justify: Center) {
            Button(text: "Login with Spotify", on_click: handle_login)
        }
    }
}
```

### MainPage

This component contains the main layout of the application after login, using flexbox.

```
component MainPage {
    render() {
        Flex(direction: Column) {
            Header()
            Flex(direction: Row, flex_grow: 1) {
                Sidebar() // width can be set via styles
                MainContent() // flex_grow: 1 to take remaining space
            }
            Footer()
        }
    }
}
```

### Header

The header can contain the user details (from `AppState`) and a logout button.

```
component Header(user: &Option<UserProfile>) {
    render() {
        Flex(justify: SpaceBetween, align: Center) {
            Label(text: "Xilem Spotify")
            Flex(gap: 10) {
                if let Some(user) = user {
                    Label(text: "Welcome, {user.name}")
                }
                Button(text: "Logout", on_click: handle_logout)
            }
        }
    }
}
```

### Sidebar

The sidebar will contain navigation, for now just the "Liked Songs" playlist. The list of playlists would come from `AppState`.

```
component Sidebar(playlists: &Vec<Playlist>) {
    render() {
        Flex(direction: Column, gap: 10) {
            Label(text: "Playlists")
            Button(text: "Liked Songs", on_click: show_liked_songs)
            // ... list other playlists from `playlists` prop
        }
    }
}
```

### MainContent

This will display the list of songs from the `active_playlist` in `AppState`.

```
component MainContent(songs: &Vec<Song>) {
    render() {
        VStack(spacing: 5) {
            for song in songs {
                SongItem(song: song)
            }
        }
    }
}

component SongItem {
    props: {
        song: Song
    }

    render() {
        Flex(justify: SpaceBetween, align: Center) {
            Label(text: "{song.title} - {song.artist}")
            Button(text: "Play", on_click: || play_song(song))
        }
    }
}
```

### Footer (Player)

The footer will act as the music player, showing the currently playing song and controls from `AppState`.

```
component Footer(current_song: &Option<Song>, is_playing: bool) {
    render() {
        Flex(justify: SpaceBetween, align: Center) {
            if let Some(song) = current_song {
                Label(text: "Now Playing: {song.title} - {song.artist}")
            } else {
                Label(text: "No song playing")
            }

            Flex(gap: 10) {
                Button(text: "Prev")
                Button(text: is_playing ? "Pause" : "Play")
                Button(text: "Next")
            }
        }
    }
}
```