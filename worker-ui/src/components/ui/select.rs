use leptos::prelude::*;

#[component]
pub fn Select(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let base = "flex h-9 w-full items-center justify-between whitespace-nowrap rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";

    view! {
        <select
            class=format!("{} {}", base, class)
            id=id
            name=name
            disabled=disabled
        >
            {children()}
        </select>
    }
}

#[component]
pub fn SelectOption(
    children: Children,
    #[prop(into)] value: String,
    #[prop(optional)] selected: bool,
) -> impl IntoView {
    view! {
        <option value=value selected=selected class="bg-background text-foreground">
            {children()}
        </option>
    }
}
