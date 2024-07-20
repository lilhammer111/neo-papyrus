use sycamore::prelude::*;


#[component]
pub fn Avatar<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="w-8 h-8 rounded-2xl bg-yellow-600")
    )
}