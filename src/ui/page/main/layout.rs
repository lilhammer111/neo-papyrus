use sycamore::prelude::*;

#[component]
pub fn MainLayout<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="p-4 text-black") {
            "hello wolrd"
        }
    )
}