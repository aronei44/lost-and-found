const NavButton = (p: {
    onClick?: () => void;
    label: string;
}) => {
    return (
        <button
            onClick={p.onClick}
            className="text-white cursor-pointer px-3">
            {p.label}
        </button>
    )
}

export default NavButton;