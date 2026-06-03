use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main>

            // ✅ Standalone banner (no card, no wrapper)
            <img
                class="home-banner"
                src="/images/title.png"
                alt="Nozawana Site"
            />

            // ✅ Content section (still styled nicely)
            <div class="home-container">
                <section class="page-content">
                    <h1>{ "Welcome" }</h1>
                    <p>
                        { "Nozawana is a delightful side dish to any meal. It is both nutritious and delicious.
                        How can you cultivate it? How can you prepare it? But, most importantly, where can you buy ready to serve products? This site aims to answer these questions and introduce you to some great nozawana products.
                        
                        And, in the production of anything, we take out. We remove things from our environment, howvever, I will in introduce you to a company which 'puts back`. A company with ecologically sustanable goals." }
                    </p>

                    <p>
                        { "More features coming soon..." }
                    </p>
                </section>
            </div>

        </main>
    }
}
