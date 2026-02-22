use leptos::prelude::*;

#[component]
pub fn Textarea(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] rows: Option<u32>,
    #[prop(optional)] node_ref: NodeRef<leptos::html::Textarea>,
) -> impl IntoView {
    let base = "flex min-h-[60px] w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm shadow-sm placeholder:text-gray-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-1 disabled:cursor-not-allowed disabled:opacity-50";

    view! {
        <textarea
            class=format!("{} {}", base, class)
            placeholder=placeholder
            name=name
            id=id
            disabled=disabled
            required=required
            rows=rows
            node_ref=node_ref
        />
    }
}
