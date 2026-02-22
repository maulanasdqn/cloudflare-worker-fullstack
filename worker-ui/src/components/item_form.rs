use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::json;
use zod_rs::prelude::*;

use crate::api;
use crate::components::ui::{Button, ButtonVariant, Label};
use crate::types::{CreateItemRequest, Item, UpdateItemRequest};

fn create_item_schema() -> impl Schema<serde_json::Value> {
    object()
        .field("name", string().min(1).max(255))
        .optional_field("description", string().max(1000))
}

fn validate_form(name: &str, description: &str) -> Result<(), Vec<String>> {
    let schema = create_item_schema();
    let desc_value = if description.is_empty() {
        serde_json::Value::Null
    } else {
        json!(description)
    };

    let data = json!({
        "name": name,
        "description": desc_value
    });

    match schema.safe_parse(&data) {
        Ok(_) => Ok(()),
        Err(result) => {
            let errors: Vec<String> = result
                .issues
                .iter()
                .map(|issue| issue.to_string())
                .collect();
            Err(errors)
        }
    }
}

#[component]
pub fn ItemForm<F>(
    editing_item: ReadSignal<Option<Item>>,
    set_editing_item: WriteSignal<Option<Item>>,
    on_success: F,
) -> impl IntoView
where
    F: Fn() + Copy + Send + Sync + 'static,
{
    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (validation_errors, set_validation_errors) = signal(Vec::<String>::new());

    Effect::new(move |_| {
        if let Some(item) = editing_item.get() {
            set_name.set(item.name);
            set_description.set(item.description.unwrap_or_default());
        } else {
            set_name.set(String::new());
            set_description.set(String::new());
        }
        set_validation_errors.set(Vec::new());
    });

    let validate_name = move |_| {
        let name_val = name.get();
        if name_val.is_empty() {
            set_validation_errors.set(vec!["Name is required".to_string()]);
        } else if name_val.len() > 255 {
            set_validation_errors.set(vec!["Name must be 255 characters or less".to_string()]);
        } else {
            set_validation_errors.set(Vec::new());
        }
    };

    let validate_description = move |_| {
        let desc_val = description.get();
        if desc_val.len() > 1000 {
            set_validation_errors
                .set(vec!["Description must be 1000 characters or less".to_string()]);
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let name_val = name.get();
        let desc_val = description.get();

        match validate_form(&name_val, &desc_val) {
            Ok(()) => {}
            Err(errors) => {
                set_validation_errors.set(errors);
                return;
            }
        }

        set_submitting.set(true);
        set_error.set(None);
        set_validation_errors.set(Vec::new());

        let desc = if desc_val.is_empty() {
            None
        } else {
            Some(desc_val)
        };

        let editing = editing_item.get_untracked();

        spawn_local(async move {
            let result = if let Some(item) = editing {
                let req = UpdateItemRequest {
                    name: Some(name_val),
                    description: desc,
                };
                api::update_item(item.id, req).await
            } else {
                let req = CreateItemRequest {
                    name: name_val,
                    description: desc,
                };
                api::create_item(req).await
            };

            match result {
                Ok(_) => {
                    set_name.set(String::new());
                    set_description.set(String::new());
                    set_editing_item.set(None);
                    on_success();
                }
                Err(e) => set_error.set(Some(e)),
            }
            set_submitting.set(false);
        });
    };

    let name_char_count = move || name.get().len();
    let desc_char_count = move || description.get().len();

    view! {
        <form on:submit=on_submit class="space-y-4">
            {move || error.get().map(|e| view! {
                <div class="bg-destructive/15 border border-destructive/50 text-destructive px-4 py-3 rounded-md text-sm">
                    {e}
                </div>
            })}

            {move || {
                let errors = validation_errors.get();
                if !errors.is_empty() {
                    Some(view! {
                        <div class="bg-yellow-500/15 border border-yellow-500/50 text-yellow-500 px-4 py-3 rounded-md text-sm">
                            <ul class="list-disc list-inside">
                                {errors.into_iter().map(|e| view! { <li>{e}</li> }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    })
                } else {
                    None
                }
            }}

            <div class="space-y-2">
                <Label html_for="name".to_string()>"Name"</Label>
                <input
                    type="text"
                    id="name"
                    class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                    placeholder="Enter item name"
                    maxlength="255"
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    on:blur=validate_name
                    required
                />
                <div class="text-xs text-muted-foreground">
                    {move || format!("{}/255 characters", name_char_count())}
                </div>
            </div>

            <div class="space-y-2">
                <Label html_for="description".to_string()>"Description"</Label>
                <textarea
                    id="description"
                    class="flex min-h-[80px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                    placeholder="Enter description (optional)"
                    rows="3"
                    maxlength="1000"
                    prop:value=move || description.get()
                    on:input=move |ev| set_description.set(event_target_value(&ev))
                    on:blur=validate_description
                ></textarea>
                <div class="text-xs text-muted-foreground">
                    {move || format!("{}/1000 characters", desc_char_count())}
                </div>
            </div>

            <div class="flex gap-2">
                <button
                    type="submit"
                    class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"
                    disabled=move || submitting.get() || !validation_errors.get().is_empty()
                >
                    {move || {
                        if submitting.get() {
                            "Saving..."
                        } else if editing_item.get().is_some() {
                            "Update"
                        } else {
                            "Create"
                        }
                    }}
                </button>

                {move || editing_item.get().map(|_| view! {
                    <Button
                        variant=ButtonVariant::Outline
                        on:click=move |_| set_editing_item.set(None)
                    >
                        "Cancel"
                    </Button>
                })}
            </div>
        </form>
    }
}
