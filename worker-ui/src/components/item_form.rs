use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::types::{CreateItemRequest, Item, UpdateItemRequest};

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

    Effect::new(move |_| {
        if let Some(item) = editing_item.get() {
            set_name.set(item.name);
            set_description.set(item.description.unwrap_or_default());
        } else {
            set_name.set(String::new());
            set_description.set(String::new());
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error.set(None);

        let name_val = name.get();
        let desc_val = description.get();
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

    view! {
        <form on:submit=on_submit class="space-y-4">
            {move || error.get().map(|e| view! {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                    {e}
                </div>
            })}

            <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">"Name"</label>
                <input
                    type="text"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="Enter item name"
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    required
                />
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">"Description"</label>
                <textarea
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="Enter description (optional)"
                    rows="3"
                    prop:value=move || description.get()
                    on:input=move |ev| set_description.set(event_target_value(&ev))
                ></textarea>
            </div>

            <div class="flex gap-2">
                <button
                    type="submit"
                    class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50"
                    disabled=move || submitting.get()
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
                    <button
                        type="button"
                        class="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400"
                        on:click=move |_| set_editing_item.set(None)
                    >
                        "Cancel"
                    </button>
                })}
            </div>
        </form>
    }
}
