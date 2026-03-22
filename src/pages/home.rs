use crate::pages::game::Game;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    // --- Greeting state ---
    let name = use_state(|| "".to_string());
    let greeting = use_state(|| "".to_string());

    // --- Game state ---
    let game = use_state(Game::new);
    let guess_input = use_state(|| "".to_string());
    let feedback = use_state(|| "".to_string());
    let feedback_kind = use_state(|| "".to_string()); // "correct" | "error" | "info"

    // --- Greeting input handler ---
    let on_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_click = {
        let name = name.clone();
        let greeting = greeting.clone();
        Callback::from(move |_| {
            greeting.set(format!("Hello, {}!", *name));
        })
    };

    // --- Game input handler ---
    let on_guess_input = {
        let guess_input = guess_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            guess_input.set(input.value());
        })
    };

    let on_guess_click = {
        let guess_input = guess_input.clone();
        let feedback = feedback.clone();
        let feedback_kind = feedback_kind.clone();
        let game = game.clone();

        Callback::from(move |_| {
            if let Ok(num) = (*guess_input).parse::<u32>() {
                let result = (*game).guess(num);

                if result == "Correct!" {
                    game.set(Game::new()); // reset secret number
                    guess_input.set("".to_string());
                    feedback.set("Correct! New game started.".to_string());
                    feedback_kind.set("correct".to_string());
                } else {
                    feedback.set(result.to_string());
                    feedback_kind.set("info".to_string());
                }
            } else {
                feedback.set("Please type a number!".to_string());
                feedback_kind.set("error".to_string());
            }
        })
    };

    html! {
        <main>
            <div class="home-container">

                // --- Greeting Card ---
                <section class="greeting-card">
                    <h1>{ "Greeting" }</h1>
                    <p>{ "Type your name below and click the button to get a personalized greeting." }</p>

                    <label>
                        { "Your Name: " }
                        <input
                            type="text"
                            placeholder="Enter your name"
                            oninput={on_input}
                        />
                    </label>

                    <button onclick={on_click}>{ "Greet me" }</button>
                    <p>{ &*greeting }</p>
                </section>

                // --- Game Card ---
                <section class="game-card">
                    <h1>{ "Guess The Number Game" }</h1>
                    <p>
                        { "Try to guess the secret number between 1 and 100. The game will tell you if your guess is too big, too small, or correct!" }
                    </p>

                    <label>
                        { "Your Guess: " }
                        <input
                            type="text"
                            placeholder="Enter your guess"
                            value={(*guess_input).clone()}
                            oninput={on_guess_input}
                        />
                    </label>

                    <button onclick={on_guess_click}>{ "Guess" }</button>

                    <p class={classes!("feedback", &*feedback_kind)}>
                        { &*feedback }
                    </p>
                </section>

            </div>
        </main>
    }
}
