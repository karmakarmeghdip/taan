export const formatTime = (seconds: number): string => {
    if (!Number.isFinite(seconds) || seconds <= 0) {
        return "0:00";
    }

    const clamped = Math.max(0, Math.floor(seconds));
    const minutes = Math.floor(clamped / 60);
    const remainingSeconds = clamped % 60;
    const paddedSeconds = remainingSeconds.toString().padStart(2, "0");

    return `${minutes}:${paddedSeconds}`;
};
