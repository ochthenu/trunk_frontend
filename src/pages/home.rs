use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main>

        <img
            class="home-banner"
            src="/images/title.png"
            alt="Nozawana Site"
        />

        <div class="home-container">
            <section class="page-content">

                <h1>{ "Welcome" }</h1>

                <p>
                    { "Nozawana is a delightful side dish to any meal. It is both nutritious and delicious. How can you cultivate it? How can you prepare it? But, most importantly, where can you buy ready-to-serve products? This site aims to answer these questions and introduce you to some great Nozawana products." }
                </p>

                <p>
                    { "In the production of almost everything, we take from our environment. However, I would also like to introduce companies that give something back—businesses with ecologically sustainable goals and a commitment to the future." }
                </p>

                <p>
                    { "More features coming soon..." }
                </p>

                <div class="home-images">

                    <img
                        src="/images/nozawana1.jpg"
                        alt="Nozawana 1"
                    />

                    <img
                        src="/images/nozawana2.jpg"
                        alt="Nozawana 2"
                    />

                    <img
                        src="/images/nozawana3.jpg"
                        alt="Nozawana 3"
                    />

                </div>

            </section>
                </div>

        </main>
    }
}
