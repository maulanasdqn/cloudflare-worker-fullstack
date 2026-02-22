use leptos::prelude::*;

#[component]
pub fn Card(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=format!("rounded-xl border border-border bg-card text-card-foreground shadow {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=format!("flex flex-col space-y-1.5 p-6 {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <h3 class=format!("font-semibold leading-none tracking-tight {}", class)>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=format!("p-6 pt-0 {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=format!("flex items-center p-6 pt-0 {}", class)>
            {children()}
        </div>
    }
}
