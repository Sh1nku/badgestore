import type {BadgeStyle} from "./BadgeStyle";

interface CreateBadge {
    left_label: string;
    left_color: string;
    right_label: string;
    right_color: string;
    style: BadgeStyle;
}

export default CreateBadge;