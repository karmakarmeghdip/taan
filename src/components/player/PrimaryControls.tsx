import { FastForward, Pause, Play, Rewind } from "lucide-react";
import IconButton from "./IconButton";

interface PrimaryControlsProps {
    isPlaying: boolean;
}

const PrimaryControls = ({ isPlaying }: PrimaryControlsProps) => (
    <div className="flex items-center justify-center gap-6">
        <IconButton label="Previous track" rounded="full">
            <Rewind size={28} />
        </IconButton>
        <IconButton label={isPlaying ? "Pause" : "Play"} variant="solid" size="lg" rounded="full">
            {isPlaying ? <Pause size={32} /> : <Play size={32} />}
        </IconButton>
        <IconButton label="Next track" rounded="full">
            <FastForward size={28} />
        </IconButton>
    </div>
);

export default PrimaryControls;
