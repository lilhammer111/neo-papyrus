use sycamore::prelude::*;

#[component]
pub fn PageHeader<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="h-14 bg-primary") {}
    )
}