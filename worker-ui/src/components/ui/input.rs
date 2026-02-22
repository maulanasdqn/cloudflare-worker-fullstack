use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] r#type: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
) -> impl IntoView {
    let base = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";
    let type_attr = r#type.unwrap_or_else(|| "text".to_string());

    view! {
        <input
            type=type_attr
            class=format!("{} {}", base, class)
            placeholder=placeholder
            name=name
            id=id
            disabled=disabled
            required=required
            value=value
            node_ref=node_ref
        />
    }
}
