use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FaqItemProps {
    pub question: String,
    pub answer: String,
}

#[function_component(FaqItem)]
pub fn faq_item(props: &FaqItemProps) -> Html {
    let is_open = use_state(|| false);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class="faq-item">
            <button class="faq-question" onclick={toggle}>
                { &props.question }
            </button>

            {
                if *is_open {
                    html! {
                        <div class="faq-answer">
                            { &props.answer }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component(Faq)]
pub fn faq() -> Html {
    html! {
        <main class="page-content faq-page">
            <h1>{ "Frequently Asked Questions" }</h1>

            <FaqItem
                question={"What is this site?".to_string()}
                answer={"This is a Yew-based Rust website.".to_string()}
            />

            <FaqItem
                question={"Is this built with Rust?".to_string()}
                answer={"Yes. The frontend is written in Rust using Yew.".to_string()}
            />

            <FaqItem
                question={"Does it work on mobile?".to_string()}
                answer={"Yes. The layout is responsive.".to_string()}
            />
        </main>
    }
}





