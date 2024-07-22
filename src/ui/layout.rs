use sycamore::prelude::*;
use crate::ui::siderbar::layout::SiderbarLayout;
use crate::ui::page::layout::PageLayout;


#[component]
pub fn RootLayout<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="flex h-full w-full") {
            div(class="") {
                SiderbarLayout()
            }
            div(class="w-full h-full flex flex-col") {
                PageLayout()
            }
        }
    )
}
