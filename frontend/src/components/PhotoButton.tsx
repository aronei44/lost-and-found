const PhotoButton = (p: {
    children?: React.ReactNode;
    onClick?: () => void;
}) => {
    return (
        <button onClick={p.onClick} className='fixed bottom-12 right-24 rounded-full bg-white shadow-2xl h-8 w-8 cursor-pointer border border-slate-500 hover:bg-slate-500'>{p.children}</button>
    )
}

export default PhotoButton;