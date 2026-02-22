use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::components::{ItemForm, ItemList};
use crate::types::Item;

#[component]
pub fn App() -> impl IntoView {
    let (items, set_items) = signal(Vec::<Item>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);
    let (editing_item, set_editing_item) = signal(Option::<Item>::None);
    let (refresh_trigger, set_refresh_trigger) = signal(0u32);

    Effect::new(move |_| {
        let _ = refresh_trigger.get();
        set_loading.set(true);
        set_error.set(None);
        spawn_local(async move {
            match api::fetch_items().await {
                Ok(fetched_items) => {
                    set_items.set(fetched_items);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    });

    let refresh = move || {
        set_refresh_trigger.update(|n| *n += 1);
    };

    view! {
        <div class="container mx-auto px-4 py-8 max-w-4xl">
            <h1 class="text-3xl font-bold text-gray-800 mb-8">"Items CRUD"</h1>

            <div class="bg-white rounded-lg shadow-md p-6 mb-8">
                <h2 class="text-xl font-semibold text-gray-700 mb-4">
                    {move || {
                        if editing_item.get().is_some() {
                            "Edit Item"
                        } else {
                            "Create Item"
                        }
                    }}
                </h2>
                <ItemForm
                    editing_item=editing_item
                    set_editing_item=set_editing_item
                    on_success=refresh
                />
            </div>

            <div class="bg-white rounded-lg shadow-md p-6">
                <h2 class="text-xl font-semibold text-gray-700 mb-4">"Items"</h2>

                {move || {
                    if loading.get() {
                        view! { <p class="text-gray-500">"Loading..."</p> }.into_any()
                    } else if let Some(err) = error.get() {
                        view! { <p class="text-red-500">{err}</p> }.into_any()
                    } else {
                        view! {
                            <ItemList
                                items=items
                                set_items=set_items
                                set_editing_item=set_editing_item
                            />
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
