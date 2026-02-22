use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;

use crate::api::{self, FetchParams};
use crate::components::ui::{Button, ButtonVariant, Card, CardContent, CardHeader, CardTitle};
use crate::components::{ItemForm, ItemList};
use crate::types::{Item, PaginationMeta};

fn get_url_param(key: &str) -> Option<String> {
    web_sys::window()
        .and_then(|w| w.location().search().ok())
        .and_then(|search| {
            web_sys::UrlSearchParams::new_with_str(&search)
                .ok()
                .and_then(|params| params.get(key))
        })
}

fn update_url_params(search: &str, sort_by: &str, sort_order: &str, page: u32) {
    if let Some(window) = web_sys::window() {
        if let Ok(params) = web_sys::UrlSearchParams::new() {
            if !search.is_empty() {
                let _ = params.set("search", search);
            }
            if sort_by != "created_at" {
                let _ = params.set("sort_by", sort_by);
            }
            if sort_order != "desc" {
                let _ = params.set("sort_order", sort_order);
            }
            if page > 1 {
                let _ = params.set("page", &page.to_string());
            }

            let query = params.to_string().as_string().unwrap_or_default();
            let new_url = if query.is_empty() {
                "/".to_string()
            } else {
                format!("/?{}", query)
            };

            if let Ok(history) = window.history() {
                let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
            }
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let initial_search = get_url_param("search").unwrap_or_default();
    let initial_sort_by = get_url_param("sort_by").unwrap_or_else(|| "created_at".to_string());
    let initial_sort_order = get_url_param("sort_order").unwrap_or_else(|| "desc".to_string());
    let initial_page = get_url_param("page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(1u32);

    let (items, set_items) = signal(Vec::<Item>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);
    let (editing_item, set_editing_item) = signal(Option::<Item>::None);
    let (refresh_trigger, set_refresh_trigger) = signal(0u32);
    let (search, set_search) = signal(initial_search);
    let (sort_by, set_sort_by) = signal(initial_sort_by);
    let (sort_order, set_sort_order) = signal(initial_sort_order);
    let (dark_mode, set_dark_mode) = signal(true);
    let (page, set_page) = signal(initial_page);
    let (pagination, set_pagination) = signal(Option::<PaginationMeta>::None);

    Effect::new(move |_| {
        let _ = refresh_trigger.get();
        let search_val = search.get();
        let sort_by_val = sort_by.get();
        let sort_order_val = sort_order.get();
        let page_val = page.get();

        update_url_params(&search_val, &sort_by_val, &sort_order_val, page_val);

        set_loading.set(true);
        set_error.set(None);

        let params = FetchParams {
            page: Some(page_val),
            per_page: Some(10),
            search: if search_val.is_empty() { None } else { Some(search_val.clone()) },
            sort_by: Some(sort_by_val),
            sort_order: Some(sort_order_val),
        };

        spawn_local(async move {
            match api::fetch_items(params).await {
                Ok(result) => {
                    set_items.set(result.items);
                    set_pagination.set(Some(result.pagination));
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

    let on_sort = move |column: &'static str| -> Box<dyn Fn()> {
        Box::new(move || {
            if sort_by.get() == column {
                set_sort_order.update(|o| {
                    *o = if *o == "asc" {
                        "desc".to_string()
                    } else {
                        "asc".to_string()
                    };
                });
            } else {
                set_sort_by.set(column.to_string());
                set_sort_order.set("asc".to_string());
            }
            set_page.set(1);
            set_refresh_trigger.update(|n| *n += 1);
        })
    };

    let toggle_dark_mode = move |_| {
        let new_mode = !dark_mode.get();
        set_dark_mode.set(new_mode);
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    if new_mode {
                        let _ = html.class_list().add_1("dark");
                    } else {
                        let _ = html.class_list().remove_1("dark");
                    }
                }
            }
        }
    };

    view! {
        <div class="min-h-screen bg-background">
            <header class="border-b border-border">
                <div class="container mx-auto px-4 py-4 flex items-center justify-between">
                    <h1 class="text-2xl font-bold">"Items CRUD"</h1>
                    <Button
                        variant=ButtonVariant::Outline
                        on:click=toggle_dark_mode
                    >
                        {move || if dark_mode.get() { "Light Mode" } else { "Dark Mode" }}
                    </Button>
                </div>
            </header>

            <main class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <div class="lg:col-span-1">
                        <Card>
                            <CardHeader>
                                <CardTitle>
                                    {move || {
                                        if editing_item.get().is_some() {
                                            "Edit Item"
                                        } else {
                                            "Create Item"
                                        }
                                    }}
                                </CardTitle>
                            </CardHeader>
                            <CardContent>
                                <ItemForm
                                    editing_item=editing_item
                                    set_editing_item=set_editing_item
                                    on_success=refresh
                                />
                            </CardContent>
                        </Card>
                    </div>

                    <div class="lg:col-span-2">
                        <Card>
                            <CardHeader>
                                <CardTitle>"Items"</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <div class="mb-4">
                                    <input
                                        type="text"
                                        class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                                        placeholder="Search by name or description..."
                                        prop:value=move || search.get()
                                        on:input=move |ev| {
                                            set_search.set(event_target_value(&ev));
                                            set_page.set(1);
                                            set_refresh_trigger.update(|n| *n += 1);
                                        }
                                    />
                                </div>

                                {move || {
                                    if loading.get() {
                                        view! { <p class="text-muted-foreground text-center py-8">"Loading..."</p> }.into_any()
                                    } else if let Some(err) = error.get() {
                                        view! { <p class="text-destructive text-center py-8">{err}</p> }.into_any()
                                    } else {
                                        view! {
                                            <ItemList
                                                items=items
                                                set_items=set_items
                                                set_editing_item=set_editing_item
                                                sort_by=sort_by
                                                sort_order=sort_order
                                                on_sort=on_sort
                                            />
                                        }.into_any()
                                    }
                                }}

                                {move || {
                                    pagination.get().map(|meta| {
                                        view! {
                                            <div class="flex items-center justify-between mt-4 pt-4 border-t border-border">
                                                <div class="text-sm text-muted-foreground">
                                                    {format!("Showing {} of {} items", items.get().len(), meta.total)}
                                                </div>
                                                <div class="flex items-center gap-2">
                                                    <button
                                                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-8 px-3 border border-input bg-background hover:bg-accent hover:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none"
                                                        disabled=move || !meta.has_prev
                                                        on:click=move |_| {
                                                            set_page.update(|p| *p = (*p).saturating_sub(1).max(1));
                                                            set_refresh_trigger.update(|n| *n += 1);
                                                        }
                                                    >
                                                        "Previous"
                                                    </button>
                                                    <span class="text-sm text-muted-foreground px-2">
                                                        {format!("Page {} of {}", meta.page, meta.total_pages)}
                                                    </span>
                                                    <button
                                                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-8 px-3 border border-input bg-background hover:bg-accent hover:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none"
                                                        disabled=move || !meta.has_next
                                                        on:click=move |_| {
                                                            set_page.update(|p| *p += 1);
                                                            set_refresh_trigger.update(|n| *n += 1);
                                                        }
                                                    >
                                                        "Next"
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                    })
                                }}
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </main>
        </div>
    }
}
