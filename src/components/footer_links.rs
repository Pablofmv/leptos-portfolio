use leptos::prelude::*;

#[component]
pub fn FooterLinks() -> impl IntoView {
    view! {
        <footer class="w-full max-w-2xl pt-8 flex justify-center gap-6 text-lg font-semibold text-zinc-500">
            <a href="https://www.linkedin.com" target="_blank" class="hover:text-zinc-800 underline underline-offset-4">
                "LinkedIn"
            </a>
            <a href="https://github.com" target="_blank" class="hover:text-zinc-800 underline underline-offset-4">
                "GitHub"
            </a>
            <a href="/resume.pdf" target="_blank" class="hover:text-zinc-800 underline underline-offset-4">
                "Resume"
            </a>
        </footer>
    }
}
