use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

fn variant_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground shadow hover:bg-primary/90",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90",
        ButtonVariant::Outline => "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
    }
}

fn size_class(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "h-9 px-4 py-2",
        ButtonSize::Sm => "h-8 rounded-md px-3 text-xs",
        ButtonSize::Lg => "h-10 rounded-md px-8",
        ButtonSize::Icon => "h-9 w-9",
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] r#type: Option<String>,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
    let variant_cls = variant_class(variant);
    let size_cls = size_class(size);
    let type_attr = r#type.unwrap_or_else(|| "button".to_string());

    view! {
        <button
            type=type_attr
            class=format!("{} {} {} {}", base, variant_cls, size_cls, class)
            disabled=disabled
        >
            {children()}
        </button>
    }
}
