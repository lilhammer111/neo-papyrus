use sycamore::prelude::*;
use super::sidebar::Sidebar;
use super::page::Page;

#[component]
pub fn Layout<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="flex h-full w-full") {
            div(class="") {
                Sidebar()
            }
            div(class="w-full h-full flex flex-col") {
                Page()
            }
        }
    )
}
