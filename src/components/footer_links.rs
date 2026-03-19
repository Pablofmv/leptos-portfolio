use leptos::prelude::*;

#[component]
pub fn FooterLinks() -> impl IntoView {
    view! {
        <footer class="footer">
            <a href="https://www.linkedin.com/in/pablofmv/" target="_blank">
                "LinkedIn"
            </a>

            <a href="https://github.com/Pablofmv" target="_blank">
                "GitHub"
            </a>

            <a href="https://leetcode.com/u/pm3179/" target="_blank">
                "Leetcode"
            </a>
        </footer>
    }
}
