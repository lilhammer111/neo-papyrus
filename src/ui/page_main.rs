use sycamore::prelude::*;

#[component]
pub fn PageMain<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="bg-zinc-200 rounded-tl-2xl h-full p-4 text-black") {
            "hello wolrd"
        }
    )
}