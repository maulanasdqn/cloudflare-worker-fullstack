use leptos::prelude::*;

#[component]
pub fn Table(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class="relative w-full overflow-auto">
            <table class=format!("w-full caption-bottom text-sm {}", class)>
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeader(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <thead class=format!("[&_tr]:border-b {}", class)>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBody(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <tbody class=format!("[&_tr:last-child]:border-0 {}", class)>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableRow(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <tr class=format!("border-b border-border transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted {}", class)>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHead(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <th class=format!("h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}", class)>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <td class=format!("p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}", class)>
            {children()}
        </td>
    }
}
