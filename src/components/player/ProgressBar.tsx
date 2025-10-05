import { formatTime } from "../../utils/formatTime";

interface ProgressBarProps {
    positionSeconds: number;
    durationSeconds: number;
}

const ProgressBar = ({ positionSeconds, durationSeconds }: ProgressBarProps) => {
    const clampedPosition = Math.max(0, Math.min(positionSeconds, durationSeconds));
    const progressPercent = durationSeconds
        ? Math.min(100, Math.max(0, (clampedPosition / durationSeconds) * 100))
        : 0;
    const remainingSeconds = Math.max(0, Math.floor(durationSeconds - clampedPosition));

    return (
        <div className="flex items-center gap-3">
            <span className="w-10 text-xs font-medium text-slate-300">{formatTime(clampedPosition)}</span>
            <div className="relative flex h-3 flex-1 items-center">
                <div className="relative h-1 w-full rounded-full bg-white/30">
                    <div
                        className="absolute left-0 top-0 h-full rounded-full bg-white transition-[width] duration-200"
                        style={{ width: `${progressPercent}%` }}
                    />
                    <div
                        className="absolute top-1/2 h-2.5 w-[3px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-white shadow-[0_2px_6px_rgba(0,0,0,0.4)]"
                        style={{ left: `${progressPercent}%` }}
                    />
                </div>
            </div>
            <span className="w-10 text-right text-xs font-medium text-slate-300">-{formatTime(remainingSeconds)}</span>
        </div>
    );
};

export default ProgressBar;
