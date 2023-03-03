
interface IWebcard {
    name: string
    link: string
}

const Webcard: React.FC<IWebcard> = ({ name, link }) => {
    return (
        <a href={link} target="_blank" rel="noreferrer">
            <div className="card w-96 bg-base-100 shadow-xl image-full">
                <div className="card-body">
                    <h2 className="card-title">{name}</h2>
                </div>
            </div>
        </a>
    )
}

export default Webcard;

