use sycamore::{component,view, reactive::Scope, view::View, web::Html};
use crate::ui::page::header::layout::HeaderLayout;
use crate::ui::page::main::layout::MainLayout;

#[component]
pub fn PageLayout<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="text-black bg-zinc-100 h-full") {
            HeaderLayout()
            MainLayout()
        }
    )
}