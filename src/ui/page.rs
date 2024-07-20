use sycamore::{component,view, reactive::Scope, view::View, web::Html};
use super::page_header::PageHeader;
use super::page_main::PageMain;

#[component]
pub fn Page<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        PageHeader()
        div(class="bg-primary h-full") {
            PageMain()
        }
    )
}