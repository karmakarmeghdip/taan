import { ListMusic, Repeat, Shuffle, Square } from "lucide-react";
import IconButton from "./IconButton";

const SecondaryControls = () => (
    <div className="flex w-full justify-around gap-3 pt-2">
        <IconButton label="Stop">
            <Square size={20} />
        </IconButton>
        <IconButton label="Shuffle">
            <Shuffle size={20} />
        </IconButton>
        <IconButton label="Repeat">
            <Repeat size={20} />
        </IconButton>
        <IconButton label="Open queue">
            <ListMusic size={20} />
        </IconButton>
    </div>
);

export default SecondaryControls;
