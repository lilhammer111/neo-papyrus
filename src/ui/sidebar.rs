use sycamore::prelude::*;
use crate::components::avatar::Avatar;
use super::switcher::AppSwitcher;

#[component]
pub fn Sidebar<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="w-18 flex-ver-bt h-full py-4 gap-6") {
            div(class="flex flex-col justify-between gap-4 items-center w-full h-full") {
                Avatar()
                div() {
                    div() {"Explore"}
                    div() {"Reading"}
                    div() {"Project"}
                }
            }
            AppSwitcher()
        }
    )
}