import Card from "./Card";

const Main = () => {
    return (
        <div className="container mx-auto p-4 mt-20">
            <Card>
                <h1 className="text-6xl font-bold mt-10">Lost And Found</h1>
                <hr />
                <h1 className="text-3xl font-bold mt-10">Lapor dan temukan</h1>
                <ul className="list-disc">
                    <li>Orang Hilang</li>
                    <li>Orang Terindikasi kejahatan</li>
                    <li>Tersangka Kriminal</li>
                    <li>Teroris</li>
                </ul>
            </Card>
        </div>
    )
}

export default Main;