const Card = (p: Readonly<React.PropsWithChildren>) => {
    return (
        <div className="bg-white shadow-lg rounded-lg p-12 max-w mx-auto m-6">
            {p.children}
        </div>
    )
}

export default Card;