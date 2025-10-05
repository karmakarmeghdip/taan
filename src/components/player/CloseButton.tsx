import { X } from "lucide-react";

interface CloseButtonProps {
    onClick?: () => void;
    className?: string;
}

const CloseButton = ({ onClick, className }: CloseButtonProps) => {
    const baseClasses = [
        "inline-flex h-9 w-9 items-center justify-center rounded-full bg-white/10 text-slate-300 transition",
        "hover:bg-white/20 hover:text-white",
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white",
        className ?? "",
    ]
        .filter(Boolean)
        .join(" ");

    return (
        <button
            type="button"
            data-tauri-drag-region="false"
            className={baseClasses}
            aria-label="Close player"
            onClick={onClick}
        >
            <X size={20} />
        </button>
    );
};

export default CloseButton;
