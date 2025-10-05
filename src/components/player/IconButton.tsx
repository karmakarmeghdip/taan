import { PropsWithChildren } from "react";

interface IconButtonProps {
    label: string;
    variant?: "ghost" | "solid";
    size?: "md" | "lg";
    rounded?: "full" | "lg";
    onClick?: () => void;
}

const IconButton = ({
    label,
    variant = "ghost",
    size = "md",
    rounded = "lg",
    onClick,
    children,
}: PropsWithChildren<IconButtonProps>) => {
    const variantClasses: Record<typeof variant, string> = {
        ghost: "bg-transparent text-slate-300 hover:bg-white/10 hover:text-white",
        solid: "bg-slate-600/60 text-white shadow-[0_10px_25px_rgba(0,0,0,0.3)] hover:bg-slate-500/70",
    };

    const sizeClasses: Record<typeof size, string> = {
        md: "p-3",
        lg: "p-4",
    };

    const roundedClasses: Record<typeof rounded, string> = {
        full: "rounded-full",
        lg: "rounded-xl",
    };

    const className = [
        "inline-flex items-center justify-center transition duration-150 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white",
        variantClasses[variant],
        sizeClasses[size],
        roundedClasses[rounded],
    ].join(" ");

    return (
        <button type="button" className={className} aria-label={label} onClick={onClick}>
            {children}
        </button>
    );
};

export default IconButton;
