import { Volume1, Volume2 } from "lucide-react";

interface VolumeControlProps {
    volume: number; // expects 0 - 1
}

const VolumeControl = ({ volume }: VolumeControlProps) => {
    const clampedVolume = Math.max(0, Math.min(volume, 1));
    const volumePercent = clampedVolume * 100;

    return (
        <div className="flex items-center gap-3 px-2">
            <Volume1 size={18} className="text-white/65" />
            <div className="flex-1 h-1.5 overflow-hidden rounded-full bg-white/25">
                <div
                    className="h-1.5 rounded-full bg-white transition-[width] duration-200"
                    style={{ width: `${volumePercent}%` }}
                />
            </div>
            <Volume2 size={18} className="text-white/65" />
        </div>
    );
};

export default VolumeControl;
