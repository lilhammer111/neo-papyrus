use sycamore::prelude::*;

#[component]
#[allow(dead_code)]
pub fn MenuBar<G: Html>(cx: Scope) -> View<G> {
    view!(
        cx,
        div(class = "flex justify-start items-center gap-4 bg-gray-100 px-2 py-2") {
            div {"File"}
            div {"Edit"}
            div {"Help"}
        }
    )
}
