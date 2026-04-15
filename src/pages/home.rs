use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main>
            <div class="home-container">

                // ✅ Big title card (replaces greeting)
                <section class="greeting-card">
                    <h1 class="home-title">
                        { "Nozawana Site" }
                    </h1>
                </section>

                // ✅ Content card (replaces game)
                <section class="game-card">
                    <h1>{ "Welcome" }</h1>
                    <p>
                        { "This is your homepage. You can add blog highlights, photos, or any information here later." }
                    </p>

                    <p>
                        { "More features coming soon..." }
                    </p>
                </section>

            </div>
        </main>
    }
}
