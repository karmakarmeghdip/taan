import MusicPlayer, { PlaybackState, TrackInfo } from "./components/player/MusicPlayer";
import "./App.css";

const track: TrackInfo = {
  title: "Albireo",
  artist: "Rokudenashi",
  writer: "ナユタン星人",
  artworkUrl: "https://i.scdn.co/image/ab67616d00001e02f6ccb29fbda0541861558a94",
};

const playback: PlaybackState = {
  positionSeconds: 19,
  durationSeconds: 245,
  isPlaying: false,
  volume: 0.7,
};

const App = () => (
  <div className="h-full w-full overflow-hidden font-inter text-slate-100 antialiased">
    <MusicPlayer track={track} playback={playback} />
  </div>
);

export default App;
