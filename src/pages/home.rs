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
