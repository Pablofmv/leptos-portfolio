use leptos::prelude::*;

#[component]
pub fn Summary() -> impl IntoView {
    view! {
        <section class="w-full max-w-4xl grid grid-cols-1 md:grid-cols-[1.2fr_0.8fr] gap-10 items-start">
            <div class="space-y-5">
                <h2 class="text-[2rem] font-extrabold leading-tight text-zinc-800">
                    "Tech Professional interested in Performance and Design"
                </h2>

                <div class="space-y-3 text-[1.05rem] leading-8 text-zinc-700">
                    <p>
                        "🎓 MSC @ "
                        <a
                            href="https://www.columbia.edu"
                            target="_blank"
                            class="underline hover:text-zinc-900"
                        >
                            "Columbia"
                        </a>
                    </p>

                    <p>
                        "💻 Current. @ "
                        <a
                            href="https://www.spglobal.com/en"
                            class="underline hover:text-zinc-900"
                        >
                            "SP"
                        </a>
                        " // Prev. @ "
                        <a
                            href="https://www.regeneron.com"
                            class="underline hover:text-zinc-900"
                        >
                            "Regeneron"
                        </a>
                    </p>
                </div>
            </div>

            <div class="hidden md:flex justify-center items-start">
                <img
                    src="/illustration.png"
                    alt="Developer illustration"
                    class="w-[200px] h-auto object-contain"
                />
            </div>
        

        </section>
    }
}
