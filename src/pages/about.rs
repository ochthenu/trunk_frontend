use yew::prelude::*;

const NOZAWANA_TEXT: &str = concat!(
    "Nozawana (野沢菜), Brassica rapa var. hakabura, is a Japanese leaf vegetable, ",
    "a cultivated variety of Brassica rapa in the brassica family. It is a biennial plant ",
    "often pickled that has been cultivated in the Shin'etsu region, ",
    "centered around the village of Nozawaonsen, Shimotakai District, Nagano Prefecture. ",
    "It is of the same species as the common turnip and one of a number of Japanese varieties ",
    "of leaf mustard. Also known as shinshūna (信州菜), it is counted as one of Japan's three major pickled vegetables, ",
    "along with takana and hiroshimana. After World War II, it grew nationwide, ",
    "from Hokkaido to Kumamoto."
);

const BACKGROUND_TEXT: &str = concat!(
    "Nozawana is part of the turnip family. Currently, it is thought to be a different variety ",
    "derived from the turnip (var. hakabura, turnip greens), and is currently being used as a ",
    "traditional pickled vegetable grown in the area."
);

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <main class="page-content page-about">
            <h1>{ "Nozawana" }</h1>

            <p>{ NOZAWANA_TEXT }</p>

            <img
                src="/images/nozawana1.png"
                alt="Nozawana"
            />

            <h2>{ "Background" }</h2>

            <p>{ BACKGROUND_TEXT }</p>

            <img
                src="/images/pik.png"
                alt="Pickled Nozawana"
            />

            <p>{
                "Tradition holds that sometime between 1751 and 1764 the plant was brought from the Kyoto mountains \
                 to the village of Nozawaonsen by the master of the Buddhist temple Kenmeiji. \
                 It has been cultivated around that area ever since, and thus came to be called Nozawana."
            }</p>
        </main>
    }
}



