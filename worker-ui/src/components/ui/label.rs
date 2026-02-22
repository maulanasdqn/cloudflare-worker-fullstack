use leptos::prelude::*;

#[component]
pub fn Label(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] html_for: Option<String>,
) -> impl IntoView {
    let base = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

    view! {
        <label class=format!("{} {}", base, class) for=html_for>
            {children()}
        </label>
    }
}
