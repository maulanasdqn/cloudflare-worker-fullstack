use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::types::Item;

#[component]
pub fn ItemList(
    items: ReadSignal<Vec<Item>>,
    set_items: WriteSignal<Vec<Item>>,
    set_editing_item: WriteSignal<Option<Item>>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            {move || {
                let items_list = items.get();
                if items_list.is_empty() {
                    view! {
                        <p class="text-gray-500 text-center py-4">"No items yet. Create one above!"</p>
                    }.into_any()
                } else {
                    view! {
                        <table class="min-w-full divide-y divide-gray-200">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">"ID"</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">"Name"</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">"Description"</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">"Created"</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">"Actions"</th>
                                </tr>
                            </thead>
                            <tbody class="bg-white divide-y divide-gray-200">
                                <For
                                    each=move || items.get()
                                    key=|item| item.id
                                    children=move |item: Item| {
                                        let item_for_edit = item.clone();
                                        let item_id = item.id;
                                        view! {
                                            <tr>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{item.id}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{item.name.clone()}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                    {item.description.clone().unwrap_or_else(|| "-".to_string())}
                                                </td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{item.created_at.clone()}</td>
                                                <td class="px-6 py-4 whitespace-nowrap text-sm space-x-2">
                                                    <button
                                                        class="text-blue-600 hover:text-blue-900"
                                                        on:click={
                                                            let item_clone = item_for_edit.clone();
                                                            move |_| set_editing_item.set(Some(item_clone.clone()))
                                                        }
                                                    >
                                                        "Edit"
                                                    </button>
                                                    <button
                                                        class="text-red-600 hover:text-red-900"
                                                        on:click=move |_| {
                                                            spawn_local(async move {
                                                                if api::delete_item(item_id).await.is_ok() {
                                                                    set_items.update(|items| items.retain(|i| i.id != item_id));
                                                                }
                                                            });
                                                        }
                                                    >
                                                        "Delete"
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                    }
                                />
                            </tbody>
                        </table>
                    }.into_any()
                }
            }}
        </div>
    }
}
