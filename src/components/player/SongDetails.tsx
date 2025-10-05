interface SongDetailsProps {
    title: string;
    artist: string;
    writer: string;
}

const SongDetails = ({ title, artist, writer }: SongDetailsProps) => (
    <div className="text-center">
        <h2 className="text-2xl font-bold tracking-tight text-white">{title}</h2>
        <p className="mt-1 text-base text-white/75">{artist}</p>
        <p className="mt-2 text-sm text-white/60">Written by {writer}</p>
    </div>
);

export default SongDetails;
