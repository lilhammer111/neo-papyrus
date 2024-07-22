use std::vec;

use sycamore::prelude::*;

use crate::components::dropdown::Dropdown;

const MENU_ITEMS: &[&str] = &["File", "Edit", "Help"];

// const FILE_OPTIONS: &[&[&str]] = &[&["Open File"]];

#[component]
pub fn Menu<G: Html>(cx: Scope) -> View<G> {
    let og: Vec<Vec<&str>> = vec![vec!["Open File"]];
    provide_context(cx, og);

    let visible = create_signal(cx, false);

    let views = View::new_fragment(
        MENU_ITEMS
            .iter()
            .map( |&item_label| {
                view!(
                    cx,
                    div(
                        class="relative w-14 py-1 text-center hover:bg-zinc-200 hover:rounded hover:cursor-pointer",
                        on:mouseentry=move |_| {
                            visible.set(true);
                        },
                        on:mouseleave = move |_| {
                            visible.set(false);
                        } 
                    ) {
                        (item_label)
                        // (if *visible.get() {
                        //     view!(cx, Dropdown())
                        // } else {
                        //     view!(cx,)
                        // })
                    }
                )
            })
            .collect(),
    );

    view!(
        cx,
        div(class="flex items-center px-1 py-1") {
            (views)
            Dropdown()
        }
    )
}
