use crate::components::avatar::Avatar;
// use crate::ui::switcher::AppSwitcher;
use sycamore::prelude::*;

#[component]
pub fn SiderbarLayout<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="w-18 flex-ver-bt h-full py-4 gap-6") {
            div(class="flex flex-col justify-between gap-4 items-center w-full h-full") {
                // AppSwitcher()
                div() {
                    div() {"Explore"}
                    div() {"Reading"}
                    div() {"Project"}
                }
            }
            Avatar()
        }
    )
}
