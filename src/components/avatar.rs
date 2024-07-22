use sycamore::prelude::*;


#[component]
pub fn Avatar<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="w-10 h-10 rounded bg-stone-500")
    )
}