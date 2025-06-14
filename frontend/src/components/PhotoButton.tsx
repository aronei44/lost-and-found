const PhotoButton = (p: {
    children?: React.ReactNode;
    onClick?: () => void;
    scale_right?: 1 | 2;
}) => {
    // Default right position if not provided
    let rightPosition = 'right-24';
    if (p.scale_right === 2) {
        rightPosition = 'right-48';
    }
    return (
        <button onClick={p.onClick} className={`fixed bottom-12 ${rightPosition} rounded-full bg-white shadow-2xl h-8 w-8 cursor-pointer border border-slate-500 hover:bg-slate-500`}>{p.children}</button>
    )
}

export default PhotoButton;