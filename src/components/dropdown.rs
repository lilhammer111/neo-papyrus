// use sycamore::prelude::*;

// use crate::ui::page::header::menu::ShouldDisplay;

// #[derive(Prop)]
// pub struct Props {
//     option_groups: Vec<Vec<&'static str>>,
// }

// #[component]
// pub fn Dropdown<G: Html>(cx: Scope, props: Props) -> View<G> {
//     let sd_ctx: &Signal<ShouldDisplay> = use_context::<Signal<ShouldDisplay>>(cx);
//     let views = props
//         .option_groups
//         .iter()
//         .map(|group| {
//             let option_views = group
//                 .iter()
//                 .map(|&option| {
//                     view!(
//                         cx,
//                         div(class="w-32") {(option)}
//                     )
//                 })
//                 .collect::<Vec<_>>();

//             View::new_fragment(option_views)
//         })
//         .collect::<Vec<_>>();

//     view!(
//         cx,
//         div(class = "absolute flex flex-col items-center shadow-lg") {
//             (if sd_ctx.get().0 {
//                 View::new_fragment(views.clone())
//             } else {
//                 view!(cx,)
//             })
//         }
//     )
// }

use sycamore::prelude::*;

#[component]
pub fn Dropdown<G: Html>(cx: Scope) -> View<G> {
    let og = use_context::<Signal<Vec<Vec<&str>>>>(cx).get();
    let views = og
        .iter()
        .map(|group| {
            let option_views = group
                .iter()
                .map(|&option| {
                    view!(
                        cx,
                        div(class="w-32") {(option)}
                    )
                })
                .collect::<Vec<_>>();
            View::new_fragment(option_views)
        })
        .collect::<Vec<_>>();

    view!(
        cx,
        div(class = "absolute flex flex-col items-center shadow-lg") {
            "Hello dropdown"
            (View::new_fragment(views.clone()))
        }
    )
}
