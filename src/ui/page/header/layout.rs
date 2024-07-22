use sycamore::prelude::*;
use super::menu::Menu;

#[component]
pub fn HeaderLayout<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="border-b border-zinc-200") {
            Menu()
        }
    )
}