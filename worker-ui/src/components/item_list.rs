use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::components::ui::{
    Button, ButtonSize, ButtonVariant, Table, TableBody, TableCell, TableHeader, TableRow,
};
use crate::types::Item;

#[component]
pub fn SortableHeader<F>(
    label: &'static str,
    column: &'static str,
    sort_by: ReadSignal<String>,
    sort_order: ReadSignal<String>,
    on_click: F,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let is_active = move || sort_by.get() == column;
    let arrow = move || {
        if is_active() {
            if sort_order.get() == "asc" { " ↑" } else { " ↓" }
        } else {
            ""
        }
    };

    view! {
        <th
            class="h-10 px-2 text-left align-middle font-medium text-muted-foreground cursor-pointer hover:text-foreground select-none"
            on:click=move |_| on_click()
        >
            <span class=move || if is_active() { "text-foreground" } else { "" }>
                {label}{arrow}
            </span>
        </th>
    }
}

#[component]
pub fn ItemList<F>(
    items: ReadSignal<Vec<Item>>,
    set_items: WriteSignal<Vec<Item>>,
    set_editing_item: WriteSignal<Option<Item>>,
    sort_by: ReadSignal<String>,
    sort_order: ReadSignal<String>,
    on_sort: F,
) -> impl IntoView
where
    F: Fn(&'static str) -> Box<dyn Fn()> + Copy + Send + Sync + 'static,
{
    view! {
        <div>
            {move || {
                let items_list = items.get();
                if items_list.is_empty() {
                    view! {
                        <p class="text-muted-foreground text-center py-8">"No items found."</p>
                    }.into_any()
                } else {
                    view! {
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <SortableHeader
                                        label="ID"
                                        column="id"
                                        sort_by=sort_by
                                        sort_order=sort_order
                                        on_click=move || (on_sort("id"))()
                                    />
                                    <SortableHeader
                                        label="Name"
                                        column="name"
                                        sort_by=sort_by
                                        sort_order=sort_order
                                        on_click=move || (on_sort("name"))()
                                    />
                                    <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">"Description"</th>
                                    <SortableHeader
                                        label="Created"
                                        column="created_at"
                                        sort_by=sort_by
                                        sort_order=sort_order
                                        on_click=move || (on_sort("created_at"))()
                                    />
                                    <th class="h-10 px-2 text-right align-middle font-medium text-muted-foreground">"Actions"</th>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <For
                                    each=move || items.get()
                                    key=|item| item.id
                                    children=move |item: Item| {
                                        let item_for_edit = item.clone();
                                        let item_id = item.id;
                                        view! {
                                            <TableRow>
                                                <TableCell class="font-medium".to_string()>{item.id}</TableCell>
                                                <TableCell>{item.name.clone()}</TableCell>
                                                <TableCell class="text-muted-foreground".to_string()>
                                                    {item.description.clone().unwrap_or_else(|| "-".to_string())}
                                                </TableCell>
                                                <TableCell class="text-muted-foreground".to_string()>{item.created_at.clone()}</TableCell>
                                                <TableCell class="text-right".to_string()>
                                                    <div class="flex justify-end gap-2">
                                                        <Button
                                                            variant=ButtonVariant::Ghost
                                                            size=ButtonSize::Sm
                                                            on:click={
                                                                let item_clone = item_for_edit.clone();
                                                                move |_| set_editing_item.set(Some(item_clone.clone()))
                                                            }
                                                        >
                                                            "Edit"
                                                        </Button>
                                                        <Button
                                                            variant=ButtonVariant::Destructive
                                                            size=ButtonSize::Sm
                                                            on:click=move |_| {
                                                                spawn_local(async move {
                                                                    if api::delete_item(item_id).await.is_ok() {
                                                                        set_items.update(|items| items.retain(|i| i.id != item_id));
                                                                    }
                                                                });
                                                            }
                                                        >
                                                            "Delete"
                                                        </Button>
                                                    </div>
                                                </TableCell>
                                            </TableRow>
                                        }
                                    }
                                />
                            </TableBody>
                        </Table>
                    }.into_any()
                }
            }}
        </div>
    }
}
