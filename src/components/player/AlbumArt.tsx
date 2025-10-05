interface AlbumArtProps {
    src: string;
    alt: string;
}

const AlbumArt = ({ src, alt }: AlbumArtProps) => (
    <div className="flex w-full flex-row justify-center h-full">
        <img
            src={src}
            alt={alt}
            className="relative rounded-[1.5rem] border border-white/15 bg-black/40 object-contain shadow-[0_28px_55px_rgba(0,0,0,0.55)]"
        />
    </div>

);

export default AlbumArt;
