import CloseButton from "./CloseButton";
import AlbumArt from "./AlbumArt";
import ProgressBar from "./ProgressBar";
import SongDetails from "./SongDetails";
import PrimaryControls from "./PrimaryControls";
import VolumeControl from "./VolumeControl";
import SecondaryControls from "./SecondaryControls";

export interface TrackInfo {
    title: string;
    artist: string;
    writer: string;
    artworkUrl: string;
}

export interface PlaybackState {
    positionSeconds: number;
    durationSeconds: number;
    isPlaying: boolean;
    volume: number;
}

interface MusicPlayerProps {
    track: TrackInfo;
    playback: PlaybackState;
    onClose?: () => void;
}

const MusicPlayer = ({ track, playback, onClose }: MusicPlayerProps) => (
    <div className="flex h-full w-full min-h-[31.25rem] min-w-[18.75rem] flex-col overflow-hidden text-white">
        <div className="flex h-12 items-center justify-end px-6" data-tauri-drag-region>
            <CloseButton onClick={onClose} className="shrink-0" />
        </div>
        <div className="flex flex-1 flex-col gap-6 overflow-hidden px-7 pb-7">
            <div className="flex-1 overflow-hidden">
                <AlbumArt src={track.artworkUrl} alt={`${track.title} album art`} />
            </div>
            <div className="flex flex-none flex-col gap-5">
                <ProgressBar
                    positionSeconds={playback.positionSeconds}
                    durationSeconds={playback.durationSeconds}
                />
                <SongDetails title={track.title} artist={track.artist} writer={track.writer} />
                <PrimaryControls isPlaying={playback.isPlaying} />
                <VolumeControl volume={playback.volume} />
                <SecondaryControls />
            </div>
        </div>
    </div>
);

export default MusicPlayer;
