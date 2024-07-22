use sycamore::reactive::Scope;
use sycamore::view::View;
use sycamore::web::Html;
use sycamore::{component, view};

#[allow(dead_code)]
#[component]
pub fn AppSwitcher<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class="rounded-l-xl rounded-r-xl shadow-md w-10 h-5 bg-gray-200") {

        }
    )
}
