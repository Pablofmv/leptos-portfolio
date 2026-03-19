use leptos::prelude::*;
use crate::components::details::Details;

#[component]
pub fn Summary() -> impl IntoView {
    view! {
        <section class="summary">
            <div class="summary__content">
                <h2 class="title-lg">
                    "Tech Professional interested in System Design and Coding"
                </h2>

                <div class="summary__meta text-body">
                    <p>
                        "🎓 Msc @ "
                        <a
                            href="https://www.columbia.edu"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="link"
                        >
                            "Columbia"
                        </a>
                    </p>

                    <p>
                        "💻 Current. @ "
                        <a
                            href="https://www.spglobal.com/en"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="link"
                        >
                            "SP"
                        </a>
                        " // Prev. @ "
                        <a
                            href="https://www.regeneron.com"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="link"
                        >
                            "Regeneron"
                        </a>
                    </p>
                </div>
                <Details />
            </div>

            <div class="summary__visual">
                <img
                    src="/illustration.png"
                    alt="Developer illustration"
                    class="summary__image"
                />
            </div>
        </section>
    }
}